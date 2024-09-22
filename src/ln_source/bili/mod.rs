mod utils;
mod parser;

use anyhow::anyhow;
use regex::Regex;
use reqwest::Url;
use scraper::Html;
use crate::ln_source::bili::parser::detect_cloudflare_block;
use crate::request_sender::RequestSenderTrait;
use crate::scrape::{Catalog, Chapter, LightNovelSource, NovelInfo};

pub struct BiliNovelSource;

const SOURCE_URL: &str = "https://www.bilinovel.com";

#[async_trait::async_trait]
impl LightNovelSource for BiliNovelSource {
    fn is_this_source(url: &Url) -> bool {
        let re = Regex::new(r"(?:linovelib|bilinovel)\.com/novel/(\d+)").unwrap();
        re.is_match(url.as_str())
    }

    async fn get_novel_info(&self, id: &str, request_sender: impl RequestSenderTrait) -> anyhow::Result<NovelInfo> {
        let id: i32 = id.parse()?;
        let url = utils::get_info_url(id);
        let html = request_sender.req(&url).await?.text().await?;
        let html = Html::parse_document(&html);
        if detect_cloudflare_block(&html) {
            return Err(anyhow!("Cloudflare block detected"));
        }
        parser::parse_info_page(html, id, url)
    }

    async fn get_novel_catalog(&self, novel: &NovelInfo, request_sender: impl RequestSenderTrait) -> anyhow::Result<Catalog> {
        todo!()
    }

    async fn get_chapter_content(&self, chapter: &Chapter, request_sender: impl RequestSenderTrait) -> anyhow::Result<Html> {
        todo!()
    }

    async fn get_image(&self, src: &str, request_sender: impl RequestSenderTrait) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::ln_source::bili::BiliNovelSource;
    use crate::scrape::LightNovelSource;

    #[tokio::test]
    async fn test_get_novel_info() {
        let source = BiliNovelSource;
        let request_sender = crate::request_sender::RequestSender::new().cookie("night=0".to_owned());
        let novel_info = source.get_novel_info("2890", request_sender).await.unwrap();
        print!("{:?}", novel_info);
    }
}