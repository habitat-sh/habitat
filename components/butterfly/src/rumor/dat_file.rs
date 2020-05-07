use crate::{error::{Error,
                    Result},
            member::{MemberList,
                     Membership},
            protocol::{newscast,
                       Message},
            rumor::{Departure,
                    Election,
                    ElectionUpdate,
                    Rumor,
                    RumorStore,
                    Service,
                    ServiceConfig,
                    ServiceFile,
                    ServiceHealth},
            server::Server};
use byteorder::{ByteOrder,
                LittleEndian};
use habitat_core::fs::AtomicWriter;
use std::{collections::HashMap,
          fs::{File,
               OpenOptions},
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

// And now for a riveting discussion on version 1 vs version 2 headers in this magical file. The
// version 1 header was a struct consisting of 6 u64 fields. It did not contain any information on
// its own size, and thus that size was hardcoded into this file. The version 2 header contained
// 7 u64 fields, plus the size of the header itself, also a u64. The tidy bundle of constants below
// are necessary because after switching the Header to hold a HashMap of MESSAGE_ID -> offset, we
// can't rely on std::mem::size_of to give us the correct size of the header any more. This ensures
// that parsing and writing files continues to work.
const SIZE_OF_HEADER_FIELD: usize = mem::size_of::<u64>();
const HEADER_VERSION_1_NUM_FIELDS: usize = 6;
const HEADER_VERSION_2_NUM_FIELDS: usize = 7;
const HEADER_VERSION_1_SIZE: usize = SIZE_OF_HEADER_FIELD * HEADER_VERSION_1_NUM_FIELDS;
const HEADER_VERSION_2_SIZE: usize =
    (SIZE_OF_HEADER_FIELD * HEADER_VERSION_2_NUM_FIELDS) + SIZE_OF_HEADER_FIELD;

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
struct DatFile(PathBuf);

#[derive(Debug)]
pub struct DatFileReader {
    header:   Header,
    dat_file: DatFile,
    reader:   BufReader<File>,
}

#[derive(Debug)]
pub struct DatFileWriter(DatFile);

impl DatFileReader {
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    #[allow(clippy::too_many_arguments)]
    pub fn read_or_create_rsr_mlr(data_path: PathBuf,
                                  member_list: &MemberList,
                                  service_store: &RumorStore<Service>,
                                  service_config_store: &RumorStore<ServiceConfig>,
                                  service_file_store: &RumorStore<ServiceFile>,
                                  election_store: &RumorStore<Election>,
                                  update_store: &RumorStore<ElectionUpdate>,
                                  departure_store: &RumorStore<Departure>)
                                  -> Result<Self> {
        let size = OpenOptions::new().create(true)
                                     .read(true)
                                     .write(true)
                                     .open(&data_path)
                                     .map_err(|err| Error::DatFileIO(data_path.clone(), err))?
                                     .metadata()
                                     .map_err(|err| Error::DatFileIO(data_path.clone(), err))?
                                     .len();

        if size == 0 {
            DatFileWriter::new(data_path.clone()).write_rsr_mlr(member_list,
                                                                service_store,
                                                                service_config_store,
                                                                service_file_store,
                                                                election_store,
                                                                update_store,
                                                                departure_store)?;
        }

        Self::reader_creation(data_path)
    }

    pub fn read(data_path: PathBuf) -> Result<Self> { Self::reader_creation(data_path) }

    fn reader_creation(data_path: PathBuf) -> Result<Self> {
        let mut reader = BufReader::new(File::open(&data_path)?);
        let header = DatFile::read_header(&data_path, &mut reader)?;
        let dat_file_reader = DatFileReader { header,
                                              dat_file: DatFile(data_path),
                                              reader };
        Ok(dat_file_reader)
    }

    pub fn path(&self) -> &Path { &self.dat_file.0 }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    pub fn read_into_rsw_mlw_rhw_msr(&mut self, server: &Server) -> Result<()> {
        for Membership { member, health } in self.read_members()? {
            server.insert_member_mlw_rhw(member, health);
        }

        for service in self.read_rumors::<Service>()? {
            server.insert_service_rsw_mlw_rhw(service);
        }

        for service_config in self.read_rumors::<ServiceConfig>()? {
            server.insert_service_config_rsw_rhw(service_config);
        }

        for service_file in self.read_rumors::<ServiceFile>()? {
            server.insert_service_file_rsw_rhw(service_file);
        }

        for service_health in self.read_rumors::<ServiceHealth>()? {
            server.insert_service_health_rsw_rhw(service_health);
        }

        for election in self.read_rumors::<Election>()? {
            server.insert_election_rsw_mlr_rhw_msr(election);
        }

        for update_election in self.read_rumors::<ElectionUpdate>()? {
            server.insert_update_election_rsw_mlr_rhw(update_election);
        }

        for departure in self.read_rumors::<Departure>()? {
            server.insert_departure_rsw_mlw_rhw(departure);
        }

        Ok(())
    }

    pub fn read_rumors<T>(&mut self) -> Result<Vec<T>>
        where T: Message<newscast::Rumor>
    {
        let mut rumors = Vec::new();

        if let Some(offset) = self.header.offset_for_rumor(T::MESSAGE_ID) {
            self.dat_file
                .read_and_process(&mut self.reader, offset, |r| {
                    rumors.push(T::from_bytes(&r)?);
                    Ok(())
                })?;
        }

        Ok(rumors)
    }

    pub fn read_members(&mut self) -> Result<Vec<Membership>> {
        let mut members = Vec::new();

        if let Some(offset) = self.header.member_offset() {
            self.dat_file
                .read_and_process(&mut self.reader, offset, |r| {
                    members.push(Membership::from_bytes(&r)?);
                    Ok(())
                })?;
        }

        Ok(members)
    }
}

impl DatFileWriter {
    pub fn new(data_path: PathBuf) -> Self { DatFileWriter(DatFile(data_path)) }

    pub fn path(&self) -> &Path { &(self.0).0 }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    #[allow(clippy::too_many_arguments)]
    pub fn write_rsr_mlr(&self,
                         member_list: &MemberList,
                         service_store: &RumorStore<Service>,
                         service_config_store: &RumorStore<ServiceConfig>,
                         service_file_store: &RumorStore<ServiceFile>,
                         election_store: &RumorStore<Election>,
                         update_store: &RumorStore<ElectionUpdate>,
                         departure_store: &RumorStore<Departure>)
                         -> Result<usize> {
        let mut header = Header::default();
        let w = AtomicWriter::new(self.path()).map_err(|err| {
                                                  Error::DatFileIO(self.path().to_path_buf(), err)
                                              })?;
        w.with_writer(|mut f| {
             let mut writer = BufWriter::new(&mut f);
             let header_reserve = vec![0; HEADER_VERSION_2_SIZE];
             writer.write(&[HEADER_VERSION])
                   .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?;
             writer.write(&header_reserve)
                   .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?;
             header.insert_member_offset(self.write_member_list_mlr(&mut writer, member_list)?);
             header.insert_offset_for_rumor(Service::MESSAGE_ID,
                                            self.write_rumor_store_rsr(&mut writer,
                                                                       service_store)?);
             header.insert_offset_for_rumor(ServiceConfig::MESSAGE_ID,
                                            self.write_rumor_store_rsr(&mut writer,
                                                                       service_config_store)?);
             header.insert_offset_for_rumor(ServiceFile::MESSAGE_ID,
                                            self.write_rumor_store_rsr(&mut writer,
                                                                       service_file_store)?);
             header.insert_offset_for_rumor(Election::MESSAGE_ID,
                                            self.write_rumor_store_rsr(&mut writer,
                                                                       election_store)?);
             header.insert_offset_for_rumor(ElectionUpdate::MESSAGE_ID,
                                            self.write_rumor_store_rsr(&mut writer, update_store)?);
             header.insert_offset_for_rumor(Departure::MESSAGE_ID,
                                            self.write_rumor_store_rsr(&mut writer,
                                                                       departure_store)?);
             writer.seek(SeekFrom::Start(1))?;
             self.write_header(&mut writer, &header)?;
             writer.flush()?;
             Ok(0)
         })
         .map_err(|err| {
             match err {
                 Error::UnknownIOError(e) => Error::DatFileIO(self.path().to_path_buf(), e),
                 e => e,
             }
         })
    }

    fn write_header<W>(&self, writer: &mut W, header: &Header) -> Result<usize>
        where W: Write
    {
        let bytes = header.write_to_bytes();
        let total = writer.write(&bytes)
                          .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?;
        Ok(total)
    }

    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read)
    fn write_member_list_mlr(&self,
                             writer: &mut impl Write,
                             member_list: &MemberList)
                             -> Result<u64> {
        let mut total = 0;
        member_list.with_memberships_mlr(|membership| {
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
                       .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?
                 as u64;
        total += writer.write(&bytes)
                       .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?
                 as u64;
        Ok(total)
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    fn write_rumor_store_rsr<T, W>(&self, writer: &mut W, store: &RumorStore<T>) -> Result<u64>
        where T: Rumor,
              W: Write
    {
        let mut total = 0;
        for rumor in store.lock_rsr().rumors() {
            total += self.write_rumor(writer, rumor)?;
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
                       .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?
                 as u64;
        total += writer.write(&bytes)
                       .map_err(|err| Error::DatFileIO(self.path().to_path_buf(), err))?
                 as u64;
        Ok(total)
    }
}

impl DatFile {
    fn read_header(path: &Path, reader: &mut BufReader<File>) -> Result<Header> {
        let mut version = [0; 1];

        reader.read_exact(&mut version)
              .map_err(|err| Error::DatFileIO(path.to_path_buf(), err))?;
        debug!("Header Version: {}", version[0]);

        // If this has happened, it's likely that the file is corrupt
        if version[0] > HEADER_VERSION {
            let msg = format!("Unable to read Dat File {}: corrupt file header.",
                              path.display());
            let err = io::Error::new(io::ErrorKind::InvalidData, msg);
            return Err(Error::DatFileIO(path.to_path_buf(), err));
        }

        let header = Header::from_file(reader, version[0]).map_err(|err| {
                                                              Error::DatFileIO(path.to_path_buf(),
                                                                               err)
                                                          })?;
        debug!("Header: {:?}", header);

        reader.seek(SeekFrom::Start(header.header_offset()))
              .map_err(|err| Error::DatFileIO(path.to_path_buf(), err))?;
        Ok(header)
    }

    fn read_and_process<F>(&mut self,
                           reader: &mut BufReader<File>,
                           offset: u64,
                           mut op: F)
                           -> Result<()>
        where F: FnMut(&mut Vec<u8>) -> Result<()>
    {
        let mut bytes_read = 0;
        let mut size_buf = [0; 8];
        let mut rumor_buf: Vec<u8> = vec![];

        loop {
            if bytes_read >= offset {
                break;
            }

            reader.read_exact(&mut size_buf)
                  .map_err(|err| Error::DatFileIO(self.0.clone(), err))?;
            let rumor_size = LittleEndian::read_u64(&size_buf);
            rumor_buf.resize(rumor_size as usize, 0);
            reader.read_exact(&mut rumor_buf)
                  .map_err(|err| Error::DatFileIO(self.0.clone(), err))?;
            bytes_read += size_buf.len() as u64 + rumor_size;
            op(&mut rumor_buf)?;
        }

        Ok(())
    }
}

/// Describes contents and structure of dat file.
///
/// The information in this header is used to enable IO seeking operations on a binary dat
/// file containing rumors exchanged by the butterfly server.
#[derive(Debug, Default, PartialEq)]
struct Header {
    offsets: HashMap<String, u64>,
    size:    u64,
    version: u8,
}

impl Header {
    fn from_file<R>(reader: &mut R, version: u8) -> io::Result<Self>
        where R: Read
    {
        let mut bytes = match version {
            1 => vec![0; HEADER_VERSION_1_SIZE],
            2 => vec![0; HEADER_VERSION_2_SIZE],
            _ => unimplemented!(),
        };
        reader.read_exact(&mut bytes)?;
        Ok(Self::from_bytes(&bytes, version))
    }

    pub fn header_offset(&self) -> u64 { 1 + self.size }

    fn insert_member_offset(&mut self, offset: u64) {
        self.offsets
            .insert(Membership::MESSAGE_ID.to_string(), offset);
    }

    fn insert_offset_for_rumor(&mut self, message_id: &str, offset: u64) {
        self.offsets.insert(message_id.to_string(), offset);
    }

    fn offset_for_rumor(&self, message_id: &str) -> Option<u64> {
        self.offsets.get(message_id).copied()
    }

    fn member_offset(&self) -> Option<u64> { self.offsets.get(Membership::MESSAGE_ID).copied() }

    // Returns the size of the struct in bytes *as written*,
    // along with the struct itself future-proofed to the latest version.
    fn from_bytes(bytes: &[u8], version: u8) -> Self {
        match version {
            // The version 1 header didn't have the size of the header struct itself
            // embedded within it, so we fake it.
            1 => {
                let size: u64 = HEADER_VERSION_1_SIZE as u64;
                let mut offsets = HashMap::new();
                offsets.insert(Membership::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[0..8]));
                offsets.insert(Service::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[8..16]));
                offsets.insert(ServiceConfig::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[16..24]));
                offsets.insert(ServiceFile::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[24..32]));
                offsets.insert(Election::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[32..40]));
                offsets.insert(ElectionUpdate::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[40..48]));
                offsets.insert(Departure::MESSAGE_ID.to_string(), 0);
                Header { offsets,
                         version,
                         size }
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
                let size = LittleEndian::read_u64(&bytes[0..8]);
                let mut offsets = HashMap::new();
                offsets.insert(Membership::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[8..16]));
                offsets.insert(Service::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[16..24]));
                offsets.insert(ServiceConfig::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[24..32]));
                offsets.insert(ServiceFile::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[32..40]));
                offsets.insert(Election::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[40..48]));
                offsets.insert(ElectionUpdate::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[48..56]));
                offsets.insert(Departure::MESSAGE_ID.to_string(),
                               LittleEndian::read_u64(&bytes[56..64]));
                Header { offsets,
                         version,
                         size }
            }
        }
    }

    fn write_to_bytes(&self) -> Vec<u8> {
        let header_size = HEADER_VERSION_2_SIZE;
        let mut bytes = vec![0; header_size];
        LittleEndian::write_u64(&mut bytes[0..8], header_size as u64);
        LittleEndian::write_u64(&mut bytes[8..16],
                                self.member_offset().expect("member offset"));
        LittleEndian::write_u64(&mut bytes[16..24],
                                self.offset_for_rumor(Service::MESSAGE_ID)
                                    .expect("service offset"));
        LittleEndian::write_u64(&mut bytes[24..32],
                                self.offset_for_rumor(ServiceConfig::MESSAGE_ID)
                                    .expect("service config offset"));
        LittleEndian::write_u64(&mut bytes[32..40],
                                self.offset_for_rumor(ServiceFile::MESSAGE_ID)
                                    .expect("service file offset"));
        LittleEndian::write_u64(&mut bytes[40..48],
                                self.offset_for_rumor(Election::MESSAGE_ID)
                                    .expect("election offset"));
        LittleEndian::write_u64(&mut bytes[48..56],
                                self.offset_for_rumor(ElectionUpdate::MESSAGE_ID)
                                    .expect("election update offset"));
        LittleEndian::write_u64(&mut bytes[56..64],
                                self.offset_for_rumor(Departure::MESSAGE_ID)
                                    .expect("departure offset"));
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn read_write_header() {
        let mut original = Header::default();
        original.version = 2;
        original.insert_member_offset(rand::random::<u64>());
        original.insert_offset_for_rumor(Service::MESSAGE_ID, rand::random::<u64>());
        original.insert_offset_for_rumor(ServiceConfig::MESSAGE_ID, rand::random::<u64>());
        original.insert_offset_for_rumor(ServiceFile::MESSAGE_ID, rand::random::<u64>());
        original.insert_offset_for_rumor(Election::MESSAGE_ID, rand::random::<u64>());
        original.insert_offset_for_rumor(ElectionUpdate::MESSAGE_ID, rand::random::<u64>());
        original.insert_offset_for_rumor(Departure::MESSAGE_ID, rand::random::<u64>());

        let bytes = original.write_to_bytes();
        let restored = Header::from_bytes(&bytes, HEADER_VERSION);
        assert_eq!(bytes.len() as u64, restored.size);
        assert_eq!(original.offsets, restored.offsets);
        assert_eq!(original.version, restored.version);
    }

    /// This has to actually touch the file system because the nature of the bug its testing
    /// for is Windows-specific: AtomicWriter will fail its rename if the file is held open
    /// by the existence of a BufReader<File>.
    #[test]
    fn read_or_create_mlr_successfully_creates_when_no_file_exists() {
        let dir = tempdir().expect("temp dir created");
        let file_path = dir.path().join("test-datfile");

        assert!(!file_path.exists());

        let result = DatFileReader::read_or_create_rsr_mlr(file_path.to_path_buf(),
                                                           &MemberList::new(),
                                                           &RumorStore::default(),
                                                           &RumorStore::default(),
                                                           &RumorStore::default(),
                                                           &RumorStore::default(),
                                                           &RumorStore::default(),
                                                           &RumorStore::default());

        assert!(result.is_ok(), "{:?}", result);
        assert!(file_path.is_file());
        let dat_file_length = fs::metadata(file_path).map(|md| md.len());
        assert_ne!(dat_file_length.unwrap(), 0);
    }
}
