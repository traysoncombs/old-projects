#[derive(Clone, Debug)]
pub enum SpecialErrorKind {
    ExchangeImportError,
    TransactionCreationError,
    InsuffcientBalance,
    ErrorOpeningFile,
    InvalidFileFormat,
}

#[derive(Debug, Clone)]
pub struct SpecialError {
    pub kind: SpecialErrorKind,
    pub msg: String,
}

pub type SpecialResult<T> = Result<T, SpecialError>;

impl SpecialError {
    pub fn new(kind: SpecialErrorKind, msg: &str) -> SpecialError {
        SpecialError {
            kind,
            msg: msg.to_string(),
        }
    }
}
