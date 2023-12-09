use crate::{
    bangumi,
    utils::{self, selector},
    Result, S,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

pub struct Reg {
    pub hash: regex::Regex,
    pub number: regex::Regex,
    pub subject: regex::Regex,
    pub user: regex::Regex,
    pub title_cn: regex::Regex,
    pub on_air: regex::Regex,
    pub duration: regex::Regex,
}
impl Reg {
    fn new() -> Self {
        Self {
            hash: utils::regex(r".*/logout/([0-9a-zA-Z]*)"),
            number: utils::regex(r"(\d+)"),
            subject: utils::regex(r".*/subject/([^/?]+)"),
            user: utils::regex(r".*/user/([^/?]+)"),
            title_cn: utils::regex(r"中文标题:(.+)"),
            on_air: utils::regex(r"首播:(.+)"),
            duration: utils::regex(r"时长:(.+)"),
        }
    }
}

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
    pub async fn create(s: S<'_>) -> Result<Self> {
        let res = s.client.get(bangumi!())?.text().await?;
        Ok(Self::form(utils::html(&res)))
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
        let r = Arc::new(Reg::new());
        let avatar = dom
            .select(&selector("#headerNeue2 .idBadgerNeue .avatar"))
            .next();
        if avatar.is_none() {
            (false, None, Self::Guest(GuestResult::from(dom)))
        } else {
            let id = avatar.unwrap().value().attr("href").unwrap();
            let id = r.user.captures(id).unwrap()[1].to_string();
            (true, Some(id), Self::User(UserResult::from(dom, r)))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub hash: String,
    pub anime: Vec<Subject>,
}

impl UserResult {
    pub fn from(dom: scraper::Html, r: Arc<Reg>) -> Self {
        let hash = dom.select(&selector("#dock")).next().unwrap().html();
        let hash = r.hash.captures(&hash).unwrap()[1].to_string();
        let prg = dom.select(&selector("#subject_prg_content")).next();
        if prg.is_none() {
            return Self {
                hash,
                anime: Vec::new(),
            };
        }
        let prg = prg.unwrap();
        let prg = EpInfo::split(prg, r.clone());
        let prg = Arc::new(prg);
        let anime: Vec<Subject> = dom
            .select(&selector("#cloumnSubjectInfo .infoWrapper_tv > div"))
            .map(|e| Subject::from(e, prg.clone(), r.clone()))
            .collect();
        Self { hash, anime }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuestResult {}
impl GuestResult {
    pub fn from(_dom: scraper::Html) -> Self {
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
    pub fn from<'a>(
        e: scraper::ElementRef<'a>,
        prg: Arc<HashMap<String, EpInfo>>,
        r: Arc<Reg>,
    ) -> Self {
        let img = e
            .select(&selector("div.header.clearit a .image img"))
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap()
            .to_string();
        let title = e
            .select(&selector("div.header.clearit .headerInner h3 a.textTip"))
            .next()
            .unwrap()
            .value();
        let name_cn = title.attr("data-subject-name-cn").unwrap().to_string();
        let name = title.attr("data-subject-name").unwrap().to_string();
        let id = title.attr("href").unwrap();
        let id = r.subject.captures(id).unwrap()[1].to_string();
        let hot = e
            .select(&selector("div.header.clearit .headerInner p.tip small"))
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        let hot = r.number.captures(&hot).unwrap()[1].parse::<i32>().unwrap();
        let eps = e.select(&selector(".epGird ul.prg_list")).next().unwrap();
        let eps = EpTypedList::from(eps, prg);
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

    pub fn from<'a>(e: scraper::ElementRef<'a>, prg: Arc<HashMap<String, EpInfo>>) -> Vec<Self> {
        let mut typed = None;
        let mut eps = Vec::new();
        let mut typed_list = Vec::new();
        e.select(&selector("li")).for_each(|e| {
            if e.value()
                .has_class("subtitle", scraper::CaseSensitivity::CaseSensitive)
            {
                typed_list.push(Self::new(typed.clone(), eps.clone()));
                eps = Vec::new();
                typed = Some(e.text().next().unwrap().to_string());
                return;
            }
            eps.push(Ep::from(e, prg.clone()));
        });
        typed_list.push(Self::new(typed, eps));
        typed_list
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EpState {
    Watched,
    Queue,
    Drop,
}
impl EpState {
    pub fn from(s: Option<String>) -> Option<Self> {
        match s {
            Some(s) => match s.as_str() {
                "看过" => Some(Self::Watched),
                "抛弃" => Some(Self::Drop),
                "想看" => Some(Self::Queue),
                _ => None,
            },
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ep {
    pub ep: String,
    pub id: String,
    pub comment: i32,
    pub state: Option<EpState>,
    pub title: Option<String>,
    pub title_cn: Option<String>,
    pub on_air: Option<String>,
    pub duration: Option<String>,
}

impl Ep {
    pub fn new(id: String, ep: String, title: Option<String>, info: Option<&EpInfo>) -> Self {
        if let Some(info) = info {
            Self {
                ep,
                id,
                title,
                comment: info.comment,
                state: info.state.clone(),
                title_cn: info.title_cn.clone(),
                on_air: info.on_air.clone(),
                duration: info.duration.clone(),
            }
        } else {
            Self {
                ep,
                id,
                title,
                comment: 0,
                state: None,
                title_cn: None,
                on_air: None,
                duration: None,
            }
        }
    }

    pub fn from<'a>(e: scraper::ElementRef<'a>, prg: Arc<HashMap<String, EpInfo>>) -> Self {
        let ep = e.select(&selector("a")).next().unwrap();
        let id = ep.value().attr("rel").unwrap()[9..].to_string();
        let title = ep.value().attr("title").map(|s| s.to_string());
        let ep = ep.text().next().unwrap().to_string();
        let info = prg.get(&id);
        Self::new(id, ep, title, info)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpInfo {
    pub id: String,
    pub comment: i32,
    pub state: Option<EpState>,
    pub title_cn: Option<String>,
    pub on_air: Option<String>,
    pub duration: Option<String>,
}

impl EpInfo {
    pub fn handle_prg<'a>(prg: scraper::ElementRef<'a>, r: Arc<Reg>) -> Self {
        let id = prg.value().attr("id").unwrap()[8..].to_string();
        let state = prg
            .select(&selector(".epStatusTool .epBtnCu"))
            .next()
            .map(|e| e.text().next().unwrap().to_string());
        let state = EpState::from(state);
        if let Some(tip) = prg.select(&selector(".tip")).next() {
            let comment = tip
                .select(&selector("small.na"))
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap();
            let comment = comment[1..comment.len() - 1].parse::<i32>().unwrap_or(0);
            let tip = tip.text().collect::<Vec<_>>().join("\n");
            let title_cn = r.title_cn.captures(&tip).map(|c| c[1].trim().to_string());
            let on_air = r.on_air.captures(&tip).map(|c| c[1].trim().to_string());
            let duration = r.duration.captures(&tip).map(|c| c[1].trim().to_string());
            Self {
                id,
                comment,
                state,
                title_cn,
                on_air,
                duration,
            }
        } else {
            Self {
                id,
                comment: 0,
                state,
                title_cn: None,
                on_air: None,
                duration: None,
            }
        }
    }

    pub fn split<'a>(e: scraper::ElementRef<'a>, r: Arc<Reg>) -> HashMap<String, Self> {
        let mut map = HashMap::new();
        e.select(&selector(".prg_popup")).for_each(|e| {
            let ep = Self::handle_prg(e, r.clone());
            map.insert(ep.id.clone(), ep);
        });
        map
    }
}

// export
#[tauri::command]
pub async fn index(s: crate::S<'_>) -> Result<IndexResult> {
    let ret = IndexResult::create(s.clone()).await?;
    if let ResultEnum::User(data) = &ret.data {
        s.client.set_gh(data.hash.clone())
    }
    Ok(ret)
}
