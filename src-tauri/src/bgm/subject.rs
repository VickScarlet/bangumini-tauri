use crate::{bangumi, S};
use anyhow::Result;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EpAction {
    Watched(Vec<String>),
    Queue(String),
    Drop(String),
    Remove(String),
}

#[tauri::command]
pub async fn subject_ep_action(s: S<'_>, action: EpAction) -> Result<bool, String> {
    let (ep, action, form) = match action {
        EpAction::Watched(eps) => (
            eps.first().unwrap().clone(),
            "watched",
            if eps.len() > 1 {
                Some(Form::new().text("ep_id", eps.join(",")))
            } else {
                None
            },
        ),
        EpAction::Queue(ep) => (ep, "queue", None),
        EpAction::Drop(ep) => (ep, "drop", None),
        EpAction::Remove(ep) => (ep, "remove", None),
    };
    s.client
        .post(bangumi!("subject" / "ep" / ep / "status" / action))
        .map_err(|e| e.to_string())?
        .opt_multipart(form)
        .gh()
        .ajax()
        .ret()
        .await
        .map_err(|e| e.to_string())
}

// async fn watched() -> Result<bool> {}
