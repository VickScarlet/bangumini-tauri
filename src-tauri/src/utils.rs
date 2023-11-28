pub fn selecter(selecters: &str) -> scraper::Selector {
    scraper::Selector::parse(selecters).unwrap()
}

pub fn html(html: &str) -> scraper::Html {
    scraper::Html::parse_document(&html)
}

pub fn regex(regex: &str) -> regex::Regex {
    regex::Regex::new(regex).unwrap()
}
