use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum SendError<T> {
    Full(T),
    Closed(T),
    Timeout(T),
}

impl<T> Display for SendError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SendError::Full(_) => write!(f, "channel is full"),
            SendError::Closed(_) => write!(f, "channel has been closed"),
            SendError::Timeout(_) => write!(f, "channel sending timeout"),
        }
    }
}

impl<T: Debug> Error for SendError<T> {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RecvError {
    Empty,
    Closed,
    TimeOut,
}

impl Display for RecvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecvError::Empty => write!(f, "channel is empty"),
            RecvError::Closed => write!(f, "channel has been closed"),
            RecvError::TimeOut => write!(f, "channel receiving timeout"),
        }
    }
}

impl Error for RecvError {}
