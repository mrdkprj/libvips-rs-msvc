#[derive(Debug)]
pub enum Error {
    InitializationError(&'static str),
    IOError(&'static str),
    OperationError(&'static str),
    OperationErrorExt(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OperationErrorExt(msg) => {
                write!(
                    f,
                    "vips error: OperationError - {}",
                    msg
                )
            }
            Error::InitializationError(msg) => {
                write!(
                    f,
                    "vips error: InitializationError - {}",
                    msg
                )
            }
            Error::OperationError(msg) => write!(
                f,
                "vips error: OperationError - {}",
                msg
            ),
            Error::IOError(msg) => write!(
                f,
                "vips error: IOError - {}",
                msg
            ),
        }
    }
}

impl std::error::Error for Error {}
