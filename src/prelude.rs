pub(crate) use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue},
    Client as ReqwestClient, Error as ReqwestError, Method, RequestBuilder, Response,
};
pub(crate) use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use std::sync::Arc;

pub use crate::{client::Client, error::RequestError, request::Request};

pub type ResponseResult<T, E> = Result<Result<T, (u16, E)>, RequestError>;