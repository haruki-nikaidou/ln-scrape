use std::error::Error;
use regex::Regex;
use crate::novel_source::{DownloadedChapter, NovelCatalog, NovelProfile, NovelSource, NovelVolumeInfo};

const URL_REGEX_STR: &str = r#"(?:linovelib|bilinovel)\\.com/novel/(\\d+)"#;
const DOMAIN: &str = "https://linovelib.com";

lazy_static::lazy_static! {
    static ref URL_RE_STR: Regex = Regex::new(URL_REGEX_STR).unwrap();
}

pub struct BiliNovel;

#[async_trait::async_trait]
impl NovelSource for BiliNovel {
    fn url_belongs_to_source(url: &str) -> bool {
        URL_RE_STR.is_match(url)
    }

    async fn get_novel_profile(home_url: &str) -> Result<NovelProfile, Box<dyn Error>> {
        todo!()
    }

    async fn get_novel_catalog(profile: NovelProfile) -> Result<NovelCatalog, Box<dyn Error>> {
        todo!()
    }

    async fn download_chapter_content(volume: &NovelVolumeInfo) -> Result<Vec<DownloadedChapter>, Box<dyn Error>> {
        todo!()
    }

    async fn download_image(image_url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}