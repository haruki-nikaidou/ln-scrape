use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use crate::ln_source::bili::SOURCE_URL;

lazy_static! {
    pub(super) static ref NOVEL_ID_RE: Regex = Regex::new(r"/novel/(\d+)\.html$").unwrap();
}

pub(super) fn get_novel_id(url: &Url) -> Option<i32> {
    let path = url.path();
    NOVEL_ID_RE.captures(path)
        .and_then(|caps| caps.get(1))?
        .as_str().parse().ok()
}

pub(super) fn get_info_url(id: i32) -> Url {
    Url::parse(&format!("{}/novel/{}.html", SOURCE_URL, id)).unwrap()
}

pub(super) fn get_catalog_url(id: i32) -> Url {
    Url::parse(&format!("{}/novel/{}/catalog", SOURCE_URL, id)).unwrap()
}