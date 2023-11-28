use crate::{
    bangumi,
    utils::{html, regex, selecter},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexResult {
    pub login: bool,
    pub id: Option<String>,
    pub data: ResultEnum,
}
impl IndexResult {
    pub fn form(dom: scraper::Html) -> Self {
        let (login, id, data) = ResultEnum::split(dom);
        Self { login, id, data }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResultEnum {
    User(UserResult),
    Guest(GuestResult),
}

impl ResultEnum {
    pub fn split(dom: scraper::Html) -> (bool, Option<String>, Self) {
        let avatar = dom
            .select(&selecter("#headerNeue2 .idBadgerNeue .avatar"))
            .next();
        if avatar.is_none() {
            (false, None, Self::Guest(GuestResult::from(dom)))
        } else {
            let id = avatar.unwrap().value().attr("href").unwrap();
            let id = regex(r".*/user/([^/?]+)").captures(id).unwrap()[1].to_string();
            (true, Some(id), Self::User(UserResult::from(dom)))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub tv_subjects: Vec<Subject>,
}

impl UserResult {
    pub fn from(dom: scraper::Html) -> Self {
        let tv_subjects: Vec<Subject> = dom
            .select(&selecter("#cloumnSubjectInfo .infoWrapper_tv > div"))
            .map(Subject::from)
            .collect();
        Self { tv_subjects }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuestResult {}
impl GuestResult {
    pub fn from(dom: scraper::Html) -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub id: String,
    pub img: String,
    pub name_cn: String,
    pub name: String,
    pub hot: i32,
    pub eps: Vec<EpTypedList>,
}

impl Subject {
    pub fn from<'a>(e: scraper::ElementRef<'a>) -> Self {
        let img = e
            .select(&selecter("div.header.clearit a .image img"))
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap()
            .to_string();
        let title = e
            .select(&selecter("div.header.clearit .headerInner h3 a.textTip"))
            .next()
            .unwrap()
            .value();
        let name_cn = title.attr("data-subject-name-cn").unwrap().to_string();
        let name = title.attr("data-subject-name").unwrap().to_string();
        let id = title.attr("href").unwrap();
        let id = regex(r".*/subject/([^/?]+)").captures(id).unwrap()[1].to_string();
        let hot = e
            .select(&selecter("div.header.clearit .headerInner p.tip small"))
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");

        let hot = regex(r"(\d+)").captures(&hot).unwrap()[1]
            .parse::<i32>()
            .unwrap();

        let eps = e.select(&selecter(".epGird ul.prg_list")).next().unwrap();
        let eps = EpTypedList::from(eps);

        Self {
            id,
            img,
            name_cn,
            name,
            hot,
            eps,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpTypedList {
    pub def: bool,
    pub typed: Option<String>,
    pub eps: Vec<Ep>,
}

impl EpTypedList {
    pub fn new(typed: Option<String>, eps: Vec<Ep>) -> Self {
        Self {
            def: typed.is_none(),
            typed,
            eps,
        }
    }

    pub fn from<'a>(e: scraper::ElementRef<'a>) -> Vec<Self> {
        let mut typed = None;
        let mut eps = Vec::new();
        let mut typed_list = Vec::new();
        e.select(&selecter("li")).for_each(|e| {
            if e.value()
                .has_class("subtitle", scraper::CaseSensitivity::CaseSensitive)
            {
                typed_list.push(Self::new(typed.clone(), eps.clone()));
                eps = Vec::new();
                typed = Some(e.text().collect::<Vec<_>>().join(""));
                return;
            }
            let ep = Ep::from(e);
            eps.push(ep);
        });
        typed_list.push(Self::new(typed, eps));
        typed_list
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ep {
    pub id: String,
    pub state: String,
}

impl Ep {
    pub fn from<'a>(e: scraper::ElementRef<'a>) -> Self {
        let e = e.select(&selecter("a.load-epinfo")).next().unwrap();
        let id = e.attr("subject_id").unwrap().to_string();
        let state = e.attr("class").unwrap();
        let state = regex(r".*epBtn([a-zA-Z]+)").captures(state).unwrap()[1].to_string();
        Self { id, state }
    }
}

// export
#[tauri::command]
pub async fn index(s: crate::S<'_>) -> Result<IndexResult, String> {
    let client = s.client.get_client()?;
    let res = client
        .get(bangumi!("/"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(IndexResult::form(html(&res)))
}
