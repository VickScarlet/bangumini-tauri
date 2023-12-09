#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bgm;
mod net;
mod utils;
use net::HttpClient;
use std::sync::Arc;
use tauri::{api::path, Env, State, WindowEvent};

use serde::{ser::Serializer, Serialize};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    CookieStore(#[from] cookie_store::Error),
}

// we must manually implement serde::Serialize
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T, E = CommandError> = anyhow::Result<T, E>;

pub struct _S {
    client: HttpClient,
}

impl _S {
    pub fn new<A: tauri::Assets>(ctx: &tauri::Context<A>) -> Self {
        let path = path::resolve_path(
            ctx.config(),
            ctx.package_info(),
            &Env::default(),
            "cookies.json",
            Some(path::BaseDirectory::AppConfig),
        )
        .expect("failed to resolve cookies path");
        let version = ctx.package_info().version.to_string();
        let ua = format!("Bangumini/{} (Vick Scarlet<vick@syaro.io> https://github.com/VickScarlet/bangumini-tauri)", version);
        println!("{}", ua);
        let client = HttpClient::new(path, &ua, "");
        Self { client }
    }
}

pub type S<'a> = State<'a, Arc<_S>>;

#[tokio::main]
async fn main() {
    let ctx = tauri::generate_context!();
    let state = _S::new(&ctx);
    let state = Arc::new(state);
    tauri::Builder::default()
        .manage(state.clone())
        // .setup(|app| {
        //     let app_handle = app.handle();
        //     tauri::async_runtime::spawn(async move {
        //         let mut rx = rx.lock().await;
        //         loop {
        //             if let Some(payload) = rx.recv().await {
        //                 app_handle.emit_all("state-update", payload).unwrap();
        //             }
        //         }
        //     });
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            bgm::auth::get_formhash,
            bgm::auth::get_captcha,
            bgm::auth::signup,
            bgm::auth::logout,
            bgm::index::index,
            bgm::subject::subject_ep_action
        ])
        .on_window_event(move |event| match event.event() {
            WindowEvent::Destroyed => {
                state.client.save().unwrap();
            }
            _ => {}
        })
        .run(ctx)
        .expect("error while running tauri application");
}
