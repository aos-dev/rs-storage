use std::error;
use std::fmt;
use std::path::Path;

use tokio::io;

use async_trait::async_trait;

mod fs;

#[derive(Debug)]
enum Error {
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO: {}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

#[async_trait]
trait Storager {
    async fn stat(&self, path: &Path);
    async fn delete(&self, path: &Path) -> Result<()>;
    async fn read(&self, path: &Path) -> Result<Box<dyn tokio::io::AsyncRead + Unpin>>;
    async fn write<R: io::AsyncRead + Unpin + Send + Sync>(
        &self,
        path: &Path,
        r: &mut R,
    ) -> Result<()>;
}
