use async_trait::async_trait;
use reqwest::Url;
use anyhow::Result;
use scraper::Html;
use crate::request_sender::RequestSenderTrait;
use crate::scrape::{Catalog, Chapter, NovelInfo};

#[async_trait]
pub trait LightNovelSource: Send + Sync {
    fn is_this_source(url: &Url) -> bool;
    async fn get_novel_info(&self, id: &str, request_sender: impl RequestSenderTrait) -> Result<NovelInfo>;
    async fn get_novel_catalog(&self, novel: &NovelInfo, request_sender: impl RequestSenderTrait) -> Result<Catalog>;
    async fn get_chapter_content(&self, chapter: &Chapter, request_sender: impl RequestSenderTrait) -> Result<Html>;
    async fn get_image(&self, src: &str, request_sender: impl RequestSenderTrait) -> Result<Vec<u8>>;
}