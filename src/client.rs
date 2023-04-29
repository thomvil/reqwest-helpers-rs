use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Client {
    home: String,
    cookie_store: Arc<CookieStoreMutex>,
    client: ReqwestClient,
}

// Constructors
impl Client {
    fn new_raw(home: &str, default_headers: Option<&[(&'static str, &'static str)]>) -> Self {
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::default()));

        let mut res = ReqwestClient::builder()
            .deflate(true)
            .gzip(true)
            .brotli(true)
            .cookie_provider(Arc::clone(&cookie_store));

        if let Some(dh) = default_headers {
            let mut headers = HeaderMap::new();
            dh.iter().for_each(|(key, value)| {
                headers.insert(*key, HeaderValue::from_static(value));
            });
            res = res.default_headers(headers);
        }

        let client = res.build().expect("Building client should not fail");

        Self {
            home: home.to_string(),
            cookie_store,
            client,
        }
    }

    pub fn new(home: &str) -> Self {
        Self::new_raw(home, None)
    }

    pub fn with_default_headers(
        home: &str,
        default_headers: &[(&'static str, &'static str)],
    ) -> Self {
        Self::new_raw(home, Some(default_headers))
    }
}

// Crate-helpers
impl Client {
    pub(crate) fn inner(&self) -> &ReqwestClient {
        &self.client
    }

    pub(crate) fn cookies(&self) -> &Arc<CookieStoreMutex> {
        &self.cookie_store
    }
}

impl Client {
    // TODO: should return Client?
    pub fn manipulate(&mut self, f: impl Fn(&mut ReqwestClient) -> ReqwestClient) {
        self.client = f(&mut self.client)
    }

    // TODO: should return Client?
    pub fn try_manipulate<E>(
        &mut self,
        f: impl Fn(&mut ReqwestClient) -> Result<ReqwestClient, E>,
    ) -> Result<(), E> {
        self.client = f(&mut self.client)?;
        Ok(())
    }

    pub fn home(&self) -> &str {
        self.home.as_str()
    }

    pub fn get(&self, relative_url: &str) -> Request {
        Request::get(self, relative_url)
    }

    pub fn patch(&self, relative_url: &str) -> Request {
        Request::patch(self, relative_url)
    }

    pub fn post(&self, relative_url: &str) -> Request {
        Request::post(self, relative_url)
    }

    pub fn put(&self, relative_url: &str) -> Request {
        Request::put(self, relative_url)
    }

    pub fn delete(&self, relative_url: &str) -> Request {
        Request::delete(self, relative_url)
    }
}
