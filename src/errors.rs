#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}
impl Error {
    pub fn new(message: &str) -> Self {
        Self {message: message.into()}
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new(&format!("IO error: {error}"))
    }
}

impl From<adafruit_ble_fs_client::Error> for Error {
    fn from(error: adafruit_ble_fs_client::Error) -> Self {
        Self::new(&format!("Bluetooth error: {error}"))
    }
}


impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Self::new(&format!("Error extracting zipfile: {error}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::new(&format!("Error deserializing json: {error}"))
    }
}
