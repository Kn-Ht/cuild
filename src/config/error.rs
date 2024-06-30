use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    SerializeError(toml::ser::Error),
    DeserializeError(toml::de::Error),
}
pub type Result<T> = std::result::Result<T, Error>;


// Implementations
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializeError(value)
    }
}
impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::SerializeError(value)
    }
}