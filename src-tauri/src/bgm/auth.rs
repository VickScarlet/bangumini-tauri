use crate::bangumi;
use anyhow::Result;
use reqwest::{multipart::Form, Client};
use serde::{Deserialize, Serialize};

async fn _get_captcha(client: Client) -> Result<Vec<u8>> {
    let params = [("t", chrono::Local::now().timestamp())];
    let url = bangumi!("signup" / "captcha");
    let captcha = client
        .get(url)
        .query(&params)
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();
    Ok(captcha)
}

async fn _get_formhash(client: Client) -> Result<String> {
    let login_resp = client.get(&bangumi!("login")).send().await?.text().await?;
    let dom = scraper::Html::parse_document(&login_resp);
    let formhash = dom
        .select(&scraper::Selector::parse("input[name=formhash]").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap()
        .to_string();
    Ok(formhash)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupParams {
    formhash: String,
    email: String,
    password: String,
    captcha: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupResult {
    pub success: bool,
    pub message: Option<String>,
}
impl SignupResult {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
        }
    }
    pub fn fail(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
        }
    }
}

async fn _signup(params: SignupParams, client: Client) -> Result<SignupResult> {
    let form = Form::new()
        .text("formhash", params.formhash)
        .text("email", params.email)
        .text("password", params.password)
        .text("captcha_challenge_field", params.captcha)
        .text("referer", bangumi!(/))
        .text("dreferer", bangumi!(/))
        .text("loginsubmit", "登录");
    let url = bangumi!("FollowTheRabbit");
    let resp = client.post(url).multipart(form).send().await?;
    let ret = match resp.status() {
        reqwest::StatusCode::OK => {
            let resp = resp.text().await?;
            if regex::Regex::new(r"呜咕，出错了")?.is_match(resp.as_str()) {
                let message = scraper::Html::parse_document(&resp)
                    .select(&scraper::Selector::parse("#colunmNotice .message>p.text").unwrap())
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                SignupResult::fail(message)
            } else {
                SignupResult::success()
            }
        }
        _ => SignupResult::fail(format!("Unexpected status code: {}", resp.status())),
    };
    Ok(ret)
}

// export
#[tauri::command]
pub async fn get_formhash(s: crate::S<'_>) -> Result<String, String> {
    _get_formhash(s.client.get_client()?)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_captcha(s: crate::S<'_>) -> Result<Vec<u8>, String> {
    _get_captcha(s.client.get_client()?)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn signup(s: crate::S<'_>, params: SignupParams) -> Result<SignupResult, String> {
    let result = _signup(params, s.client.get_client()?)
        .await
        .map_err(|e| e.to_string())?;
    if result.success {
        s.client.save()?;
    }
    Ok(result)
}

#[tauri::command]
pub async fn logout(s: crate::S<'_>) -> Result<(), String> {
    s.client.clear()?;
    s.client.save()?;
    Ok(())
}
