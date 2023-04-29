use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Request<'a> {
    client: &'a Client,
    method: Method,
    relative_url: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<String>,
    validate_statuscode: Option<u16>,
}

// Constructors
impl<'a> Request<'a> {
    fn init<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            client,
            method: Method::GET,
            relative_url: relative_url.as_ref().to_string(),
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
            validate_statuscode: None,
        }
    }

    pub fn get<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: Method::GET,
            ..Self::init(client, relative_url)
        }
    }

    pub fn patch<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: Method::PATCH,
            ..Self::init(client, relative_url)
        }
    }

    pub fn post<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: Method::POST,
            ..Self::init(client, relative_url)
        }
    }

    pub fn put<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: Method::PUT,
            ..Self::init(client, relative_url)
        }
    }

    pub fn delete<S: AsRef<str>>(client: &'a Client, relative_url: S) -> Self {
        Self {
            method: Method::DELETE,
            ..Self::init(client, relative_url)
        }
    }

    pub fn build(self) -> Result<RequestBuilder, RequestError> {
        let url = format!("{}/{}", self.client.home(), self.relative_url);
        let mut req = self
            .client
            .inner()
            .request(self.method, url)
            .query(&self.query);
        for (k, v) in self.headers.iter() {
            let header_key = HeaderName::try_from(k.as_bytes())?;
            let header_value = HeaderValue::try_from(v.as_bytes())?;
            req = req.header(header_key, header_value);
        }
        if let Some(b) = self.body {
            req = req.body(b);
        }
        Ok(req)
    }
}

// Accessors
impl<'a> Request<'a> {
    pub fn query<K, V>(mut self, name: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.query
            .push((name.as_ref().to_string(), value.as_ref().to_string()));
        self
    }

    pub fn queries<K, V>(mut self, queries: &[(K, V)]) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (k, v) in queries {
            self = self.query(k, v);
        }
        self
    }

    pub fn header<K, V>(mut self, name: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.headers
            .push((name.as_ref().to_string(), value.as_ref().to_string()));
        self
    }

    pub fn headers<K, V>(mut self, headers: &[(K, V)]) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (k, v) in headers {
            self = self.header(k, v);
        }
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn validate_statuscode(mut self, statuscode: u16) -> Self {
        self.validate_statuscode = Some(statuscode);
        self
    }
}

// Network IO
impl Request<'_> {
    pub async fn send(self) -> Result<Response, RequestError> {
        Ok(self.build()?.send().await?)
    }

    pub async fn text_raw(self) -> Result<(u16, String), RequestError> {
        let res = self.build()?.send().await?;
        Ok((res.status().as_u16(), res.text().await?))
    }

    pub async fn text(self) -> Result<Result<String, (u16, String)>, RequestError> {
        let expected_statuscode = self.validate_statuscode;
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
        let expected_statuscode = self.validate_statuscode;
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
