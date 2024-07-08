#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic(0")]
    Generic(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Generic(err.to_string())
    }
}
