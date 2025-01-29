use failchain::UnboxedError;
use std::result::Result as StdResult;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ErrorKind(pub(crate) String);

pub type Error = UnboxedError<ErrorKind>;
pub type Result<T> = StdResult<T, Error>;
