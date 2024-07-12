use std::error::Error;
use fake_user_agent::get_chrome_rua;
use regex::Regex;
use retry::delay::Fibonacci;
use retry::{OperationResult, retry};
use scraper::{Html, Node};
use tracing::{error, warn};
use crate::novel_source::{DownloadedChapter, NovelCatalog, NovelProfile, NovelSource, NovelSourceError, NovelVolumeInfo};
use crate::novel_source::NovelSourceError::{InvalidUrl, ParseError};

const URL_REGEX_STR: &str = r#"(?:linovelib|bilinovel)\\.com/novel/(\\d+)"#;
const DOMAIN: &str = "https://linovelib.com";

const COOKIE: &str = "night=0";

lazy_static::lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(URL_REGEX_STR).unwrap();
}

pub struct BiliNovel;

fn try_get_novel_id(url: &str) -> Option<String> {
    URL_REGEX.captures(url)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

async fn get_home_page(novel_id: String) -> Result<String, reqwest::Error> {
    let url = format!("{}/novel/{}", DOMAIN, novel_id);
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header("cookie", COOKIE)
        .header("user-agent", get_chrome_rua())
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

fn try_get_title(home_page_fragment: &scraper::Html) -> Option<String> {
    home_page_fragment.select(&scraper::Selector::parse(".book-title").unwrap())
        .next()
        .map(|e| e.text().collect())
}

fn try_get_cover_url(home_page_fragment: &scraper::Html) -> Option<String> {
    home_page_fragment.select(&scraper::Selector::parse(".book-layout img").unwrap())
        .next()
        .and_then(|e| e.value().attr("src"))
        .map(|s| s.to_string())
}

fn try_get_tags(home_page_fragment: &scraper::Html) -> Vec<String> {
    home_page_fragment.select(&scraper::Selector::parse(".book-cell .book-meta span em").unwrap())
        .map(|e| e.text().collect())
        .collect()
}

fn try_get_publisher(home_page_fragment: &scraper::Html) -> Option<String> {
    home_page_fragment.select(&scraper::Selector::parse(".tag-small.orange").unwrap())
        .next()
        .map(|e| e.text().collect())
}

fn try_get_author(home_page_fragment: &scraper::Html) -> Option<String> {
    home_page_fragment.select(&scraper::Selector::parse(".book-rand-a span").unwrap())
        .next()
        .map(|e| e.text().collect())
}

fn try_get_description(home_page_fragment: &scraper::Html) -> Option<String> {
    home_page_fragment.select(&scraper::Selector::parse("#bookSummary content").unwrap())
        .next()
        .map(|e| e.text().collect())
}

async fn get_catalog_page(novel_id: String) -> Result<String, reqwest::Error> {
    let url = format!("{}/novel/{}/catalog", DOMAIN, novel_id);
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header("cookie", COOKIE)
        .header("user-agent", get_chrome_rua())
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

fn get_volumes_nodes<'a>(html: Html) -> Vec<&'a Node> {
    let parent = html.select(&scraper::Selector::parse("#volumes").unwrap());
    let children: Vec<_> = parent
        .flat_map(|e| e.children())
        .map(|e| e.value())
        .collect();
    children
}



#[async_trait::async_trait]
impl NovelSource for BiliNovel {
    fn url_belongs_to_source(url: &str) -> bool {
        URL_REGEX.is_match(url)
    }

    async fn get_novel_profile(home_url: &str) -> Result<NovelProfile, NovelSourceError> {
        let novel_id = match try_get_novel_id(home_url) {
            Some(id) => id,
            None => {
                warn!("Failed to get novel id from url: {}", home_url);
                return Err(InvalidUrl(home_url.to_string()));
            }
        };

        let home_page = match get_home_page(novel_id.clone()).await {
            Ok(page) => page,
            Err(e) => {
                error!("Failed to get home page for novel {}: {}", novel_id, e);
                return Err(NovelSourceError::NetworkError(e));
            }
        };
        let fragment = scraper::Html::parse_document(&home_page);
        let title = match try_get_title(&fragment) {
            Some(t) => t,
            None => {
                return Err(ParseError("Failed to get title".to_owned()));
            }
        };
        let cover_url = try_get_cover_url(&fragment);
        let tags = try_get_tags(&fragment);
        let publisher = try_get_publisher(&fragment);
        let author = match try_get_author(&fragment) {
            Some(a) => a,
            None => {
                return Err(ParseError("Failed to get author".to_owned()));
            }
        };
        let description = try_get_description(&fragment);
        Ok(NovelProfile {
            id: novel_id,
            title,
            author,
            cover_image: cover_url,
            tags: if tags.is_empty() { None } else { Some(tags) },
            publisher,
            description: description.unwrap_or_default(),
        })
    }

    async fn get_novel_catalog(profile: NovelProfile) -> Result<NovelCatalog, NovelSourceError> {
        todo!()
    }

    async fn download_chapter_content(volume: &NovelVolumeInfo) -> Result<Vec<DownloadedChapter>, NovelSourceError> {
        todo!()
    }

    async fn download_image(image_url: &str) -> Result<Vec<u8>, NovelSourceError> {
        todo!()
    }
}