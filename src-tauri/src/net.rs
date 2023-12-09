use crate::Result;
use anyhow::anyhow;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::{Arc, RwLock};
#[derive(Serialize, Deserialize, Debug)]
struct Ret {
    status: String,
}

pub struct Request {
    req: reqwest::RequestBuilder,
    gh: String,
}

#[allow(dead_code)]
impl Request {
    pub fn new(req: reqwest::RequestBuilder, gh: String) -> Self {
        Self { req, gh }
    }

    pub fn gh(mut self) -> Self {
        self.req = self.req.query(&[("gh", &self.gh)]);
        self
    }

    pub fn ajax(mut self) -> Self {
        self.req = self.req.query(&[("ajax", "1")]);
        self
    }

    pub fn query<T: serde::Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.req = self.req.query(query);
        self
    }

    pub fn opt_query<T: serde::Serialize + ?Sized>(mut self, query: Option<&T>) -> Self {
        if let Some(query) = query {
            self.req = self.req.query(query);
        }
        self
    }

    pub fn form<T: serde::Serialize + ?Sized>(mut self, form: &T) -> Self {
        self.req = self.req.form(form);
        self
    }

    pub fn multipart(mut self, form: reqwest::multipart::Form) -> Self {
        self.req = self.req.multipart(form);
        self
    }

    pub fn opt_multipart(mut self, form: Option<reqwest::multipart::Form>) -> Self {
        if let Some(form) = form {
            self.req = self.req.multipart(form);
        }
        self
    }

    pub async fn bytes(self) -> Result<Vec<u8>, reqwest::Error> {
        Ok(self.req.send().await?.bytes().await?.to_vec())
    }

    pub async fn text(self) -> Result<String, reqwest::Error> {
        Ok(self.req.send().await?.text().await?)
    }

    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, reqwest::Error> {
        Ok(self.req.send().await?.json().await?)
    }

    pub async fn ret(self) -> Result<bool, reqwest::Error> {
        let ret = self.req.send().await?.text().await?;
        if let Ok(ret) = serde_json::from_str::<Ret>(&ret) {
            if ret.status == "ok" {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub struct HttpClient {
    ua: String,
    path: String,
    store: Arc<CookieStoreMutex>,
    gh: Arc<RwLock<String>>,
}

impl HttpClient {
    pub fn new<P: AsRef<Path>>(path: P, ua: &str, gh: &str) -> Self {
        let store = match File::open(&path).map(BufReader::new) {
            Ok(file) => CookieStore::load_json(file).unwrap_or_default(),
            Err(_) => CookieStore::default(),
        };
        Self {
            ua: ua.to_string(),
            path: path.as_ref().to_str().unwrap().to_string(),
            store: Arc::new(CookieStoreMutex::new(store)),
            gh: Arc::new(RwLock::new(gh.to_string())),
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = Path::new(&self.path).parent().unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }
        let mut writer = File::create(&self.path).map(BufWriter::new)?;
        let store = self.store.lock().map_err(|e| anyhow!(e.to_string()))?;
        store.save_json(&mut writer)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        let mut store = self.store.lock().map_err(|e| anyhow!(e.to_string()))?;
        store.clear();
        Ok(())
    }

    pub fn set_gh(&self, gh: String) -> () {
        *self.gh.write().unwrap() = gh;
    }

    pub fn get_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(self.store.clone())
            .user_agent(&self.ua)
            .build()
    }

    pub fn get_gh(&self) -> String {
        self.gh.read().unwrap().clone()
    }

    pub fn get<U: reqwest::IntoUrl>(&self, url: U) -> anyhow::Result<Request> {
        Ok(Request::new(self.get_client()?.get(url), self.get_gh()))
    }

    pub fn post<U: reqwest::IntoUrl>(&self, url: U) -> anyhow::Result<Request> {
        Ok(Request::new(self.get_client()?.post(url), self.get_gh()))
    }
}
