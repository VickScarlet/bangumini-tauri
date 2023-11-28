use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;

pub struct HttpClient {
    ua: String,
    path: String,
    store: Arc<CookieStoreMutex>,
}

impl HttpClient {
    pub fn new<P: AsRef<Path>>(path: P, ua: &str) -> Self {
        let store = match File::open(&path).map(BufReader::new) {
            Ok(file) => CookieStore::load_json(file).unwrap_or_default(),
            Err(_) => CookieStore::default(),
        };
        Self {
            ua: ua.to_string(),
            path: path.as_ref().to_str().unwrap().to_string(),
            store: Arc::new(CookieStoreMutex::new(store)),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = Path::new(&self.path).parent().unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        let mut writer = File::create(&self.path)
            .map(BufWriter::new)
            .map_err(|e| e.to_string())?;
        let store = self.store.lock().map_err(|e| e.to_string())?;
        store.save_json(&mut writer).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|e| e.to_string())?;
        store.clear();
        Ok(())
    }

    pub fn get_client(&self) -> Result<Client, String> {
        Client::builder()
            .cookie_store(true)
            .cookie_provider(self.store.clone())
            .user_agent(&self.ua)
            .build()
            .map_err(|e| e.to_string())
    }
}
