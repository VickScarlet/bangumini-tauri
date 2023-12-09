use crate::{bangumi, utils, Result, S};
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

#[tauri::command]
pub async fn get_formhash(s: S<'_>) -> Result<String> {
    let text = s.client.get(bangumi!("login"))?.text().await?;
    let formhash = utils::html(&text)
        .select(&scraper::Selector::parse("input[name=formhash]").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap()
        .to_string();
    Ok(formhash)
}

#[tauri::command]
pub async fn get_captcha(s: S<'_>) -> Result<Vec<u8>> {
    Ok(s.client
        .get(bangumi!("signup" / "captcha"))?
        .query(&[("t", chrono::Local::now().timestamp())])
        .bytes()
        .await?)
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

#[tauri::command]
pub async fn signup(s: S<'_>, params: SignupParams) -> Result<SignupResult> {
    let form = Form::new()
        .text("formhash", params.formhash)
        .text("email", params.email)
        .text("password", params.password)
        .text("captcha_challenge_field", params.captcha)
        .text("referer", bangumi!(/))
        .text("dreferer", bangumi!(/))
        .text("cookietime", "31536000")
        .text("loginsubmit", "登录");
    let text = s
        .client
        .post(bangumi!("FollowTheRabbit"))?
        .multipart(form)
        .text()
        .await?;
    if regex::Regex::new(r"呜咕，出错了")?.is_match(&text) {
        let message = scraper::Html::parse_document(&text)
            .select(&scraper::Selector::parse("#colunmNotice .message>p.text").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        Ok(SignupResult::fail(message))
    } else {
        s.client.save().unwrap_or(());
        Ok(SignupResult::success())
    }
}

#[tauri::command]
pub async fn logout(s: S<'_>) -> Result<()> {
    s.client.clear()?;
    s.client.save()?;
    Ok(())
}
