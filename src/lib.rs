// #![allow(dead_code, unused)]
#![feature(let_chains)]

mod client;
mod debug;
mod prelude;
mod request;

pub use crate::prelude::*;

pub enum RequestError {
    Reqwest(ReqwestError),
    Unparsable { statuscode: u16, body: String },
}

impl From<ReqwestError> for RequestError {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}
