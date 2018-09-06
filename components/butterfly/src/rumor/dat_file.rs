// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::{Path, PathBuf};

use byteorder::{ByteOrder, LittleEndian};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use error::{Error, Result};
use habitat_core::util::perm;
use member::{MemberList, Membership};
use protocol::{newscast, Message};
use rumor::{
    Departure, Election, ElectionUpdate, Rumor, RumorStore, Service, ServiceConfig, ServiceFile,
};
use server::Server;

const HEADER_VERSION: u8 = 2;

/// The rumor file should only be readable and writable by the user
/// that creates it (i.e., the Supervisor).
pub const DAT_FILE_PERMISSIONS: u32 = 0o600;

/// A versioned binary file containing rumors exchanged by the butterfly server which have
/// been periodically persisted to disk.
///
/// The contents of the DatFile can be used to rebuild and resume a butterfly server connection
/// if it has been destroyed or lost.
///
/// * Header Version - 1 byte
/// * Header Body - Variable bytes - see Header
/// * Rumors - Variable bytes
#[derive(Debug)]
pub struct DatFile {
    header: Header,
    header_size: u64,
    path: PathBuf,
}

impl DatFile {
    pub fn new<T: AsRef<Path>>(member_id: &str, data_path: T) -> Self {
        DatFile {
            path: data_path.as_ref().join(format!("{}.rst", member_id)),
            header_size: 0,
            header: Header::default(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read_into(&mut self, server: &Server) -> Result<()> {
        let mut version = [0; 1];
        let mut size_buf = [0; 8];
        // JW: Resizing this buffer is terrible for performance, but it's the easiest way to
        // read exactly N bytes from a file. I'm not sure what the right approach is but this
        // won't be a performance issue for a long time anyway, if ever.
        let mut rumor_buf: Vec<u8> = vec![];
        let mut bytes_read = 0;
        let file = File::open(&self.path).map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        let mut reader = BufReader::new(file);
        reader
            .read_exact(&mut version)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        debug!("Header Version: {}", version[0]);
        let (header_size, real_header) = Header::from_file(&mut reader, version[0])
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        self.header = real_header;
        self.header_size = header_size;
        debug!("Header Size: {:?}", self.header_size);
        debug!("Header: {:?}", self.header);

        reader
            .seek(SeekFrom::Start(self.member_offset()))
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        debug!("Reading membership list from {}", self.path().display());
        loop {
            if bytes_read >= self.header.member_len {
                break;
            }
            reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            bytes_read += size_buf.len() as u64 + rumor_size;
            match Membership::from_bytes(&rumor_buf) {
                Ok(membership) => server.insert_member(membership.member, membership.health),
                Err(err) => warn!("Error reading membership rumor from dat file, {}", err),
            }
        }

        debug!("Reading service rumors from {}", self.path().display());
        bytes_read = 0;
        loop {
            if bytes_read >= self.header.service_len {
                break;
            }
            reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor = Service::from_bytes(&rumor_buf)?;
            server.insert_service(rumor);
            bytes_read += size_buf.len() as u64 + rumor_size;
        }

        debug!(
            "Reading service-config rumors from {}",
            self.path().display()
        );
        bytes_read = 0;
        loop {
            if bytes_read >= self.header.service_config_len {
                break;
            }
            reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor = ServiceConfig::from_bytes(&rumor_buf)?;
            server.insert_service_config(rumor);
            bytes_read += size_buf.len() as u64 + rumor_size;
        }

        debug!("Reading service-file rumors from {}", self.path().display());
        bytes_read = 0;
        loop {
            if bytes_read >= self.header.service_file_len {
                break;
            }
            reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor = ServiceFile::from_bytes(&rumor_buf)?;
            server.insert_service_file(rumor);
            bytes_read += size_buf.len() as u64 + rumor_size;
        }

        debug!("Reading election rumors from {}", self.path().display());
        bytes_read = 0;
        loop {
            if bytes_read >= self.header.election_len {
                break;
            }
            reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor = Election::from_bytes(&rumor_buf)?;
            server.insert_election(rumor);
            bytes_read += size_buf.len() as u64 + rumor_size;
        }

        debug!(
            "Reading update election rumors list from {}",
            self.path().display()
        );
        bytes_read = 0;
        loop {
            if bytes_read >= self.header.update_len {
                break;
            }
            reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor = ElectionUpdate::from_bytes(&rumor_buf)?;
            server.insert_update_election(rumor);
            bytes_read += size_buf.len() as u64 + rumor_size;
        }

        if version[0] >= 2 {
            debug!(
                "Reading departure rumors list from {}",
                self.path().display()
            );
            bytes_read = 0;
            loop {
                if bytes_read >= self.header.departure_len {
                    break;
                }
                reader
                    .read_exact(&mut size_buf)
                    .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
                let rumor_size = LittleEndian::read_u64(&size_buf);
                rumor_buf.resize(rumor_size as usize, 0);
                reader
                    .read_exact(&mut rumor_buf)
                    .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
                let rumor = Departure::from_bytes(&rumor_buf)?;
                server.insert_departure(rumor);
                bytes_read += size_buf.len() as u64 + rumor_size;
            }
        }

        Ok(())
    }

    pub fn write(&self, server: &Server) -> Result<usize> {
        let mut header = Header::default();
        let tmp_path = self.path.with_extension(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .collect::<String>(),
        );
        {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&tmp_path)
                .map_err(|err| Error::DatFileIO(tmp_path.clone(), err))?;
            let mut writer = BufWriter::new(&file);
            self.init(&mut writer)?;
            header.member_len = self.write_member_list(&mut writer, &server.member_list)?;
            header.service_len = self.write_rumor_store(&mut writer, &server.service_store)?;
            header.service_config_len =
                self.write_rumor_store(&mut writer, &server.service_config_store)?;
            header.service_file_len =
                self.write_rumor_store(&mut writer, &server.service_file_store)?;
            header.election_len = self.write_rumor_store(&mut writer, &server.election_store)?;
            header.update_len = self.write_rumor_store(&mut writer, &server.update_store)?;
            header.departure_len = self.write_rumor_store(&mut writer, &server.departure_store)?;
            writer
                .seek(SeekFrom::Start(1))
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            self.write_header(&mut writer, &header)?;
            writer
                .flush()
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;

            file.sync_all()
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        }

        perm::set_permissions(&tmp_path, DAT_FILE_PERMISSIONS)?;

        fs::rename(&tmp_path, &self.path).map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        self.sync_parent_dir()?;
        Ok(0)
    }

    /// sync_parent_dir calls sync_all (fsync) on the parent directory
    /// of the dat_file. The goal of this function is to ensure file
    /// operations such as rename are persisted to disk.
    ///
    /// On windows this function does nothing.
    #[cfg(unix)]
    fn sync_parent_dir(&self) -> Result<()> {
        let parent = self.path.parent().ok_or(Error::DatFileIO(
            self.path.clone(),
            io::Error::new(io::ErrorKind::Other, "Dat file has no parent directory"),
        ))?;
        fs::File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|err| Error::DatFileIO(parent.to_path_buf(), err))?;
        Ok(())
    }

    #[cfg(not(unix))]
    fn sync_parent_dir(&self) -> Result<()> {
        Ok(())
    }

    fn init<W>(&self, writer: &mut W) -> Result<usize>
    where
        W: Write,
    {
        let mut total = 0;
        let header_reserve = vec![0; mem::size_of::<Header>() + 8];
        total += writer
            .write(&[HEADER_VERSION])
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        total += writer
            .write(&header_reserve)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        Ok(total)
    }

    fn member_offset(&self) -> u64 {
        1 + self.header_size
    }

    #[allow(dead_code)]
    fn service_offset(&self) -> u64 {
        self.member_offset() + self.header.member_len
    }

    #[allow(dead_code)]
    fn service_config_offset(&self) -> u64 {
        self.service_offset() + self.header.service_len
    }

    #[allow(dead_code)]
    fn service_file_offset(&self) -> u64 {
        self.service_config_offset() + self.header.service_config_len
    }

    #[allow(dead_code)]
    fn election_offset(&self) -> u64 {
        self.service_file_offset() + self.header.service_file_len
    }

    #[allow(dead_code)]
    fn update_offset(&self) -> u64 {
        self.election_offset() + self.header.election_len
    }

    #[allow(dead_code)]
    fn departure_offset(&self) -> u64 {
        self.update_offset() + self.header.update_len
    }

    fn write_header<W>(&self, writer: &mut W, header: &Header) -> Result<usize>
    where
        W: Write,
    {
        let bytes = header.write_to_bytes().unwrap();
        let total = writer
            .write(&bytes)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        Ok(total)
    }

    fn write_member_list<W>(&self, writer: &mut W, member_list: &MemberList) -> Result<u64>
    where
        W: Write,
    {
        let mut total = 0;
        let members = member_list
            .members
            .read()
            .expect("Member list lock poisoned");
        for member in members.values() {
            if let Some(membership) = member_list.membership_for(&member.id) {
                total += self.write_member(writer, &membership)?;
            }
        }
        Ok(total)
    }

    fn write_member<W>(&self, writer: &mut W, membership: &Membership) -> Result<u64>
    where
        W: Write,
    {
        let mut total = 0;
        let mut len_buf = [0; 8];
        let bytes = membership.clone().write_to_bytes().unwrap();
        LittleEndian::write_u64(&mut len_buf, bytes.len() as u64);
        total += writer
            .write(&len_buf)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))? as u64;
        total += writer
            .write(&bytes)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))? as u64;
        Ok(total)
    }

    fn write_rumor_store<T, W>(&self, writer: &mut W, store: &RumorStore<T>) -> Result<u64>
    where
        T: Rumor,
        W: Write,
    {
        let mut total = 0;
        for member in store
            .list
            .read()
            .expect("Rumor store lock poisoned")
            .values()
        {
            for rumor in member.values() {
                total += self.write_rumor(writer, rumor)?;
            }
        }
        Ok(total)
    }

    fn write_rumor<T, W>(&self, writer: &mut W, rumor: &T) -> Result<u64>
    where
        T: Message<newscast::Rumor>,
        W: Write,
    {
        let mut total = 0;
        let mut rumor_len = [0; 8];
        let bytes = rumor.write_to_bytes().unwrap();
        LittleEndian::write_u64(&mut rumor_len, bytes.len() as u64);
        total += writer
            .write(&rumor_len)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))? as u64;
        total += writer
            .write(&bytes)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))? as u64;
        Ok(total)
    }
}

/// Describes contents and structure of dat file.
///
/// The information in this header is used to enable IO seeking operations on a binary dat
/// file containing rumors exchanged by the butterfly server.
#[derive(Debug, Default, PartialEq)]
pub struct Header {
    pub member_len: u64,
    pub service_len: u64,
    pub service_config_len: u64,
    pub service_file_len: u64,
    pub election_len: u64,
    pub update_len: u64,
    pub departure_len: u64,
}

impl Header {
    pub fn from_file<R>(reader: &mut R, version: u8) -> io::Result<(u64, Self)>
    where
        R: Read,
    {
        let mut bytes = match version {
            1 => vec![0; mem::size_of::<Self>()],
            _ => vec![0; mem::size_of::<Self>() + 8],
        };
        reader.read_exact(&mut bytes)?;
        Ok(Self::from_bytes(&bytes, version))
    }

    // Returns the size of the struct in bytes *as written*,
    // along with the struct itself future-proofed to the latest version.
    pub fn from_bytes(bytes: &[u8], version: u8) -> (u64, Self) {
        match version {
            // The version 1 header didn't have the size of the header struct itself
            // embedded within it, so we fake it.
            1 => {
                (
                    48, // This is the size
                    Header {
                        member_len: LittleEndian::read_u64(&bytes[0..8]),
                        service_len: LittleEndian::read_u64(&bytes[8..16]),
                        service_config_len: LittleEndian::read_u64(&bytes[16..24]),
                        service_file_len: LittleEndian::read_u64(&bytes[24..32]),
                        election_len: LittleEndian::read_u64(&bytes[32..40]),
                        update_len: LittleEndian::read_u64(&bytes[40..48]),
                        departure_len: 0,
                    },
                )
            }
            // This should be the latest version of the header. As we deprecate
            // header versions, just roll this code up, and match it, then add
            // your new structure.
            //
            // So copy this struct to the last version number. Then add 8 to the previous struct's
            // (the size of a 64 bit integer) size. Then start the empty fields at 0. The result
            // will be that you read the back-compat version of the data format, and then write the
            // new.
            _ => (
                LittleEndian::read_u64(&bytes[0..8]),
                Header {
                    member_len: LittleEndian::read_u64(&bytes[8..16]),
                    service_len: LittleEndian::read_u64(&bytes[16..24]),
                    service_config_len: LittleEndian::read_u64(&bytes[24..32]),
                    service_file_len: LittleEndian::read_u64(&bytes[32..40]),
                    election_len: LittleEndian::read_u64(&bytes[40..48]),
                    update_len: LittleEndian::read_u64(&bytes[48..56]),
                    departure_len: LittleEndian::read_u64(&bytes[56..64]),
                },
            ),
        }
    }

    pub fn write_to_bytes(&self) -> Result<Vec<u8>> {
        // The header is the size of the struct plus 8 bytes for the length of the header itself.
        let header_size = mem::size_of::<Self>() + 8;
        let mut bytes = vec![0; header_size];
        LittleEndian::write_u64(&mut bytes[0..8], header_size as u64);
        LittleEndian::write_u64(&mut bytes[8..16], self.member_len);
        LittleEndian::write_u64(&mut bytes[16..24], self.service_len);
        LittleEndian::write_u64(&mut bytes[24..32], self.service_config_len);
        LittleEndian::write_u64(&mut bytes[32..40], self.service_file_len);
        LittleEndian::write_u64(&mut bytes[40..48], self.election_len);
        LittleEndian::write_u64(&mut bytes[48..56], self.update_len);
        LittleEndian::write_u64(&mut bytes[56..64], self.departure_len);
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;
    use rand;

    #[test]
    fn read_write_header() {
        let mut original = Header::default();
        original.service_len = rand::random::<u64>();
        original.service_config_len = rand::random::<u64>();
        original.service_file_len = rand::random::<u64>();
        original.election_len = rand::random::<u64>();
        original.update_len = rand::random::<u64>();
        let bytes = original.write_to_bytes().unwrap();
        let (_size_of_header, restored) = Header::from_bytes(&bytes, HEADER_VERSION);
        assert_eq!(bytes.len(), mem::size_of::<Header>() + 8);
        assert_eq!(original, restored);
    }
}
