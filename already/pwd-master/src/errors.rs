#[derive(Debug, Clone, PartialEq, Fail)]
pub enum PwdError {
    #[fail(display = "Error during string conversion: {}", _0)]
    StringConvError(String),
    #[fail(display = "Ptr was null")]
    NullPtr,
}

pub type Result<T> = ::std::result::Result<T, PwdError>;
