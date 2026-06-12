/// Application error representation.

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError{
    #[error("Number of messages in a window exceeds predefined limit")]
    RateLimit,
    #[error("Cannot create a new room - limit is reached")]
    RoomLimit,
    #[error("Cannot connect to a room - client limit is reached")]
    ClientLimit,
    #[error("Message contains invalid characters")]
    InvalidMsg,
    #[error("Message is too long")]
    MsgTooLong,
    #[error("Username contains invalid character(s)")]
    InvalidUsername,
    // 
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Error")]
    Utf8(#[from] std::str::Utf8Error),
}
