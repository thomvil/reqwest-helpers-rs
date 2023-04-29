use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Request<'a> {
    client: &'a Client,
    method: ReqwestMethod,
    relative_url: String,
    query: Option<&'static [(&'static str, &'static str)]>,
    headers: Option<&'static [(&'static str, String)]>,
    body: Option<String>,
    validate_status_code: Option<u16>,
}

impl<'a> Request<'a> {
    fn init<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            client,
            method: ReqwestMethod::GET,
            relative_url: relative_url.as_ref().to_string(),
            query: None,
            headers: None,
            body: None,
            validate_status_code: None,
        }
    }

    pub fn get<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: ReqwestMethod::GET,
            ..Self::init(client, relative_url)
        }
    }

    pub fn patch<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: ReqwestMethod::PATCH,
            ..Self::init(client, relative_url)
        }
    }

    pub fn post<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: ReqwestMethod::POST,
            ..Self::init(client, relative_url)
        }
    }

    pub fn put<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: ReqwestMethod::PUT,
            ..Self::init(client, relative_url)
        }
    }

    pub fn delete<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: ReqwestMethod::DELETE,
            ..Self::init(client, relative_url)
        }
    }

    pub fn build(self) -> ReqwestRequestBuilder {
        let url = format!("{}/{}", self.client.home(), self.relative_url);
        let mut req = self.client.inner().request(self.method, url);
        if let Some(q) = self.query {
            req = req.query(q);
        }
        if let Some(h) = self.headers {
            for (k, v) in h.iter() {
                req = req.header(k.to_string(), v.to_string());
            }
        }
        if let Some(b) = self.body {
            req = req.body(b);
        }
        req
    }

    pub async fn send(self) -> Result<ReqwestReponse, ReqwestError> {
        self.build().send().await
    }

    pub async fn text_raw(self) -> Result<(u16, String), ReqwestError> {
        let res = self.build().send().await?;
        Ok((res.status().as_u16(), res.text().await?))
    }

    pub async fn text(self) -> Result<Result<String, (u16, String)>, ReqwestError> {
        let expected_statuscode = self.validate_status_code;
        let (status, response_body) = self.text_raw().await?;
        if let Some(expected) = expected_statuscode && status == expected {
            Ok(Ok(response_body))
        } else {
            Ok(Err((status, response_body)))
        }
    }

    pub async fn json<T: DeserializeOwned, E: DeserializeOwned>(
        self,
    ) -> Result<Result<T, (u16, E)>, RequestError> {
        let expected_statuscode = self.validate_status_code;
        let (statuscode, body) = self.text_raw().await?;

        if let Some(expected) = expected_statuscode && statuscode == expected {
            let parsed = serde_json::from_str::<T>(&body).map_err(|_| RequestError::Unparsable { statuscode, body })?;
            Ok(Ok(parsed))
        } else {
            let parsed = serde_json::from_str::<E>(&body.clone()).map_err(|_| RequestError::Unparsable { statuscode, body })?;
            Ok(Err((statuscode, parsed)))
        }
    }
}
