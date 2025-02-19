#[derive(Debug)]
pub enum EiError {
    Connect,
    Init,
    Receive,
    Send,
    Decode,
}

pub type EiResult<T> = Result<T, EiError>;
