use std::fmt::Display;
use std::io::Write;
use std::io::Error as IOError;
use std::mem::size_of;

use crate::version::Version;

pub struct PackageFlags {
    pub base64: bool
}
impl From<PackageFlags> for u8 {
    fn from(value: PackageFlags) -> Self {
        todo!()
    }
}

#[repr(u8)]
pub enum StringEncoding {
    None    = 0,
    ASCII   = 1,
    UTF8    = 2,
    UTF16LE = 3,
    UTF16BE = 4,
    UTF32   = 5
}

#[repr(u8)]
pub enum DataFormat {
    None   = 0,
    JSON   = 1,
    XML    = 2,
    CSV    = 3,
    TSV    = 4,
    Binary = 5
}

#[repr(u8)]
pub enum EncryptionKinds {
    None = 0,
    RSA  = 1,
    AES  = 2
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TooLargeError {
    field: String,
    total_size: usize,
    allowed_max: usize
}
impl Display for TooLargeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is too large. it is {}, but the max is {}", &self.field, self.total_size, self.allowed_max)
    }
}
impl std::error::Error for TooLargeError { }
impl TooLargeError {
    pub fn new<S>(field: S, total_size: usize, allowed_max: usize) -> Self where S: Into<String> {
        Self {
            field: field.into(),
            total_size,
            allowed_max
        }
    }
}

#[derive(Debug)]
pub enum NetworkReadError {
    IO(IOError),
    MissingElement(String)
}

pub struct NetworkPackage {
    version: Version<u8>,
    flags: PackageFlags,
    str_encode: StringEncoding,
    data_kind: DataFormat,
    encrypt: EncryptionKinds,
    encrypt_extra: Box<[u8]>,
    content_extra: Box<[u8]>,
    data: Box<[u8]>
}
impl NetworkPackage {
    pub fn new(version: Version<u8>, flags: PackageFlags, str_encode: StringEncoding, data_kind: DataFormat, encrypt: EncryptionKinds, encrypt_extra: Box<[u8]>, content_extra: Box<[u8]>, data: Box<[u8]>) -> Result<Self, TooLargeError> {
        if encrypt_extra.len() > u8::MAX as usize {
            return Err(TooLargeError::new("encrypt_extra", encrypt_extra.len(), u8::MAX as usize));
        }
        if content_extra.len() > u8::MAX as usize {
            return Err(TooLargeError::new("content_extra", encrypt_extra.len(), u8::MAX as usize));
        }

        let max_data_size = Self::max_data_size();
        if data.len() as u32 > max_data_size {
            return Err(TooLargeError::new("data", data.len(), max_data_size as usize));
        }

        Ok(
            Self {
                version,
                flags,
                str_encode,
                data_kind,
                encrypt,
                encrypt_extra,
                content_extra,
                data
            }
        )
    }

    pub const fn max_data_size() -> u32 {
        u32::MAX - Self::header_size()
    }
    pub const fn header_size() -> u32 {
        let u8_size = size_of::<u8>() as u32;
        let total = 
            u8_size * 3 + //header
            u8_size     + //flags
            u8_size     + //str_encode
            u8_size     + //data_kind
            u8_size     + //encrypt (kind)
            u8_size     + //encrypt_extra size
            u8_size     ; //content_extra size

        total
    }

    pub fn write_to_stream<S>(self, stream: &mut S) -> std::io::Result<()> where S: Write {
        let length = Self::header_size() as usize + self.encrypt_extra.len() + self.content_extra.len() + self.data.len();
        let length = (length as u32).to_be_bytes();
        stream.write_all(&length)?;
        
        let version:       [u8; 3] = self.version.into();
        let flags:         u8      = self.flags.into();
        let str_encoding:  u8      = self.str_encode as u8;
        let data_kind:     u8      = self.data_kind as u8;
        let encrypt:       u8      = self.encrypt as u8;
        let encrypt_size:  u8      = self.encrypt_extra.len() as u8;
        let content_extra: u8      = self.content_extra.len() as u8;

        let header: [u8; 6] = [flags, str_encoding, data_kind, encrypt, encrypt_size, content_extra];

        stream.write_all(&version)?;
        stream.write_all(&header)?;

        if encrypt_size != 0 { stream.write_all(&self.encrypt_extra)?; }
        if content_extra != 0 { stream.write_all(&self.encrypt_extra)?; }
        
        stream.write_all(&self.data)?;

        Ok(())
    }

    pub fn read_from_stream<S>(stream: &mut S) -> Result<Self, NetworkReadError> where S: Write {
        todo!()
    }
}

#[test]
fn test_network_package() {

}