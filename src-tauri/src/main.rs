#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bgm;
mod net;
mod utils;
use net::HttpClient;
use std::sync::Arc;
use tauri::{api::path, Env, State, WindowEvent};

static UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";

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

        let client = HttpClient::new(path, UA);
        Self { client }
    }

    pub fn save(&self) -> Result<(), String> {
        self.client.save().map_err(|e| e.to_string())?;
        Ok(())
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
            bgm::index::index
        ])
        .on_window_event(move |event| match event.event() {
            WindowEvent::Destroyed => {
                state.save().unwrap();
            }
            _ => {}
        })
        .run(ctx)
        .expect("error while running tauri application");
}
