use crate::{error::{Error,
                    Result},
            member::{MemberList,
                     Membership},
            protocol::{newscast,
                       Message},
            rumor::{Rumor,
                    RumorStore},
            server::Server};
use byteorder::{ByteOrder,
                LittleEndian};
use habitat_core::fs::AtomicWriter;
use std::{fs::File,
          io::{self,
               BufReader,
               BufWriter,
               Read,
               Seek,
               SeekFrom,
               Write},
          mem,
          path::{Path,
                 PathBuf}};

const HEADER_VERSION: u8 = 2;

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
    header:      Header,
    header_size: u64,
    path:        PathBuf,
    reader:      BufReader<File>,
}

impl DatFile {
    pub fn new(data_path: PathBuf) -> Result<Self> {
        let file = File::open(&data_path).map_err(|err| Error::DatFileIO(data_path.clone(), err))?;
        let reader = BufReader::new(file);

        Ok(DatFile { path: data_path,
                     header_size: 0,
                     header: Header::default(),
                     reader })
    }

    pub fn member_len(&self) -> u64 { self.header.member_len }

    pub fn service_len(&self) -> u64 { self.header.service_len }

    pub fn service_config_len(&self) -> u64 { self.header.service_config_len }

    pub fn service_file_len(&self) -> u64 { self.header.service_file_len }

    pub fn election_len(&self) -> u64 { self.header.election_len }

    pub fn update_len(&self) -> u64 { self.header.update_len }

    pub fn departure_len(&self) -> u64 { self.header.departure_len }

    pub fn path(&self) -> &Path { &self.path }

    pub fn read_header(&mut self, version: &mut [u8]) -> Result<()> {
        self.reader
            .read_exact(version)
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        debug!("Header Version: {}", version[0]);
        let (header_size, real_header) =
            Header::from_file(&mut self.reader, version[0]).map_err(|err| {
                                                               Error::DatFileIO(self.path.clone(),
                                                                                err)
                                                           })?;
        self.header = real_header;
        self.header_size = header_size;
        debug!("Header Size: {:?}", self.header_size);
        debug!("Header: {:?}", self.header);

        self.reader
            .seek(SeekFrom::Start(self.member_offset()))
            .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        Ok(())
    }

    fn read_and_process<F>(&mut self, offset: u64, mut op: F) -> Result<()>
        where F: FnMut(&mut Vec<u8>) -> Result<()>
    {
        let mut bytes_read = 0;
        let mut size_buf = [0; 8];
        let mut rumor_buf: Vec<u8> = vec![];

        loop {
            if bytes_read >= offset {
                break;
            }
            self.reader
                .read_exact(&mut size_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            self.reader
                .read_exact(&mut rumor_buf)
                .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
            bytes_read += size_buf.len() as u64 + rumor_size;
            op(&mut rumor_buf)?;
        }

        Ok(())
    }

    pub fn read_rumors<T>(&mut self, offset: u64) -> Result<Vec<T>>
        where T: Message<newscast::Rumor>
    {
        let mut rumors = Vec::new();

        self.read_and_process(offset, |r| {
                rumors.push(T::from_bytes(&r)?);
                Ok(())
            })?;

        Ok(rumors)
    }

    pub fn read_members(&mut self) -> Result<Vec<Membership>> {
        let mut members = Vec::new();
        debug!("Reading membership rumors from {}", self.path().display());
        self.read_and_process(self.member_len(), |r| {
                members.push(Membership::from_bytes(&r)?);
                Ok(())
            })?;
        Ok(members)
    }

    pub fn read_into(&mut self, server: &Server) -> Result<()> {
        let mut version = [0; 1];

        self.read_header(&mut version)?;

        for Membership { member, health } in self.read_members()? {
            server.insert_member(member, health);
        }

        for service in self.read_rumors(self.service_len())? {
            server.insert_service(service);
        }

        for service_config in self.read_rumors(self.service_config_len())? {
            server.insert_service_config(service_config);
        }

        for service_file in self.read_rumors(self.service_file_len())? {
            server.insert_service_file(service_file);
        }

        for election in self.read_rumors(self.election_len())? {
            server.insert_election(election);
        }

        for update_election in self.read_rumors(self.update_len())? {
            server.insert_update_election(update_election);
        }

        if version[0] >= 2 {
            for departure in self.read_rumors(self.departure_len())? {
                server.insert_departure(departure);
            }
        }

        Ok(())
    }

    pub fn write(&self, server: &Server) -> Result<usize> {
        let mut header = Header::default();
        let w =
            AtomicWriter::new(&self.path).map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        w.with_writer(|mut f| {
             let mut writer = BufWriter::new(&mut f);
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
             writer.seek(SeekFrom::Start(1))?;
             self.write_header(&mut writer, &header)?;
             writer.flush()?;
             Ok(0)
         })
         .map_err(|err| {
             match err {
                 Error::UnknownIOError(e) => Error::DatFileIO(self.path.clone(), e),
                 e => e,
             }
         })
    }

    fn init<W>(&self, writer: &mut W) -> Result<usize>
        where W: Write
    {
        let mut total = 0;
        let header_reserve = vec![0; mem::size_of::<Header>() + 8];
        total += writer.write(&[HEADER_VERSION])
                       .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        total += writer.write(&header_reserve)
                       .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        Ok(total)
    }

    fn member_offset(&self) -> u64 { 1 + self.header_size }

    fn write_header<W>(&self, writer: &mut W, header: &Header) -> Result<usize>
        where W: Write
    {
        let bytes = header.write_to_bytes().unwrap();
        let total = writer.write(&bytes)
                          .map_err(|err| Error::DatFileIO(self.path.clone(), err))?;
        Ok(total)
    }

    fn write_member_list<W>(&self, writer: &mut W, member_list: &MemberList) -> Result<u64>
        where W: Write
    {
        let mut total = 0;
        member_list.with_memberships(|membership| {
                       total += self.write_member(writer, &membership)?;
                       Ok(total)
                   })
    }

    fn write_member<W>(&self, writer: &mut W, membership: &Membership) -> Result<u64>
        where W: Write
    {
        let mut total = 0;
        let mut len_buf = [0; 8];
        let bytes = membership.clone().write_to_bytes().unwrap();
        LittleEndian::write_u64(&mut len_buf, bytes.len() as u64);
        total += writer.write(&len_buf)
                       .map_err(|err| Error::DatFileIO(self.path.clone(), err))?
                 as u64;
        total += writer.write(&bytes)
                       .map_err(|err| Error::DatFileIO(self.path.clone(), err))?
                 as u64;
        Ok(total)
    }

    fn write_rumor_store<T, W>(&self, writer: &mut W, store: &RumorStore<T>) -> Result<u64>
        where T: Rumor,
              W: Write
    {
        let mut total = 0;
        for member in store.list
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
        where T: Message<newscast::Rumor>,
              W: Write
    {
        let mut total = 0;
        let mut rumor_len = [0; 8];
        let bytes = rumor.write_to_bytes().unwrap();
        LittleEndian::write_u64(&mut rumor_len, bytes.len() as u64);
        total += writer.write(&rumor_len)
                       .map_err(|err| Error::DatFileIO(self.path.clone(), err))?
                 as u64;
        total += writer.write(&bytes)
                       .map_err(|err| Error::DatFileIO(self.path.clone(), err))?
                 as u64;
        Ok(total)
    }
}

/// Describes contents and structure of dat file.
///
/// The information in this header is used to enable IO seeking operations on a binary dat
/// file containing rumors exchanged by the butterfly server.
#[derive(Debug, Default, PartialEq)]
pub struct Header {
    pub member_len:         u64,
    pub service_len:        u64,
    pub service_config_len: u64,
    pub service_file_len:   u64,
    pub election_len:       u64,
    pub update_len:         u64,
    pub departure_len:      u64,
}

impl Header {
    pub fn from_file<R>(reader: &mut R, version: u8) -> io::Result<(u64, Self)>
        where R: Read
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
                (48, // This is the size
                 Header { member_len:         LittleEndian::read_u64(&bytes[0..8]),
                          service_len:        LittleEndian::read_u64(&bytes[8..16]),
                          service_config_len: LittleEndian::read_u64(&bytes[16..24]),
                          service_file_len:   LittleEndian::read_u64(&bytes[24..32]),
                          election_len:       LittleEndian::read_u64(&bytes[32..40]),
                          update_len:         LittleEndian::read_u64(&bytes[40..48]),
                          departure_len:      0, })
            }
            // This should be the latest version of the header. As we deprecate
            // header versions, just roll this code up, and match it, then add
            // your new structure.
            //
            // So copy this struct to the last version number. Then add 8 to the previous struct's
            // (the size of a 64 bit integer) size. Then start the empty fields at 0. The result
            // will be that you read the back-compat version of the data format, and then write the
            // new.
            _ => {
                (LittleEndian::read_u64(&bytes[0..8]),
                 Header { member_len:         LittleEndian::read_u64(&bytes[8..16]),
                          service_len:        LittleEndian::read_u64(&bytes[16..24]),
                          service_config_len: LittleEndian::read_u64(&bytes[24..32]),
                          service_file_len:   LittleEndian::read_u64(&bytes[32..40]),
                          election_len:       LittleEndian::read_u64(&bytes[40..48]),
                          update_len:         LittleEndian::read_u64(&bytes[48..56]),
                          departure_len:      LittleEndian::read_u64(&bytes[56..64]), })
            }
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
