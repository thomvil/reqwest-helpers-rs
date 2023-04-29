use crate::prelude::*;

pub enum RequestError {
    Reqwest(ReqwestError),
    Unparsable { statuscode: u16, body: String },
    InvalidHeaderName(InvalidHeaderName),
    InvalidHeaderValue(InvalidHeaderValue),
}

impl From<ReqwestError> for RequestError {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}

impl From<InvalidHeaderName> for RequestError {
    fn from(e: InvalidHeaderName) -> Self {
        Self::InvalidHeaderName(e)
    }
}

impl From<InvalidHeaderValue> for RequestError {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}
