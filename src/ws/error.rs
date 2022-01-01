use super::Channel;
use tungstenite::error::Error as WsError;
<<<<<<< HEAD
=======
use thiserror::Error;
>>>>>>> f057ff1... added thiserror

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Ws(WsError),
    Json(serde_json::Error),
    NotConnected,
    NotAuthenticated,
    MissingSubscriptionConfirmation(Channel),
    SubscriptionFailed(Channel),
    NotSubscribed(Channel),
}

<<<<<<< HEAD
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ws: {}", self)
    }
}

=======
>>>>>>> f057ff1... added thiserror
impl From<WsError> for Error {
    fn from(err: WsError) -> Self {
        Self::Ws(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
