use scraper::{Element, Html, Selector};
use anyhow::{anyhow, Result};
use reqwest::Url;
use tracing::error;
use crate::ln_source::bili::SOURCE_URL;
use crate::scrape::{Catalog, NovelInfo, Volume};

pub(super) fn detect_cloudflare_block(html: &Html) -> bool {
    let selector = Selector::parse("title").unwrap();
    let title = html.select(&selector).next();
    match title {
        Some(title) => title.text().any(|x| x.contains("Cloudflare")),
        None => false
    }
}

pub(super) fn parse_info_page(html: Html, id: i32, url: Url) -> Result<NovelInfo> {
    let title_selector = Selector::parse(".book-title").unwrap();
    let title = html.select(&title_selector)
        .next().ok_or(anyhow!("Cannot find book title"))?
        .text().collect::<String>();

    let cover_selector = Selector::parse(".book-layout img").unwrap();
    let cover_url = html.select(&cover_selector)
        .next().ok_or(anyhow!("Cannot find book cover"))?.value()
        .attr("src").ok_or(anyhow!("cover url not found"))?
        .to_owned();

    let tags_selector = Selector::parse(".book-cell .book-meta span em").unwrap();
    let tags = html.select(&tags_selector)
        .map(|tag| tag.text().collect::<String>())
        .collect::<Vec<_>>();

    let publisher_selector = Selector::parse(".tag-small.orange").unwrap();
    let publisher = html.select(&publisher_selector).next()
        .ok_or(anyhow!("Cannot find publisher"))?
        .text().collect::<String>();

    let status_selector = Selector::parse(".book-cell .book-meta+.book-meta").unwrap();
    let status_text = html.select(&status_selector)
        .last().ok_or(anyhow!("Cannot find status"))?
        .text().collect::<String>();

    let author_selector = Selector::parse(".book-rand-a span").unwrap();
    let author = html.select(&author_selector).next()
        .ok_or(anyhow!("Cannot find author"))?
        .text().collect::<String>();

    let description_selector = Selector::parse("#bookSummary content").unwrap();
    let description = html.select(&description_selector).next()
        .ok_or(anyhow!("Cannot find description"))?
        .text().collect::<String>();

    Ok(NovelInfo {
        url,
        id: id.to_string(),
        title,
        author,
        status: status_text,
        cover: Some(cover_url),
        tags,
        publisher: Some(publisher),
        description,
    })
}

enum FromSingleLiNode {
    ChapterBar{volume_title: String},
    VolumeCover{cover_url: String},
    Chapter{chapter_name: String, chapter_url: Option<String>},
    Unknown
}

fn parse_li_element(li: scraper::ElementRef) -> Result<FromSingleLiNode> {
    fn simple_has_class(li: &scraper::ElementRef, class: &str) -> bool {
        li.has_class(
            &scraper::selector::CssLocalName(class.into()),
            scraper::CaseSensitivity::AsciiCaseInsensitive
        )
    }
    if simple_has_class(&li, "chapter-bar") {
        let volume_title = li.text().collect::<String>();
        Ok(FromSingleLiNode::ChapterBar{volume_title})
    } else if simple_has_class(&li, "volume-cover") {
        let selector = Selector::parse("a>img").unwrap();
        let element = li.select(&selector).next().unwrap();
        let cover_url = element.value().attr("src").ok_or(anyhow!("cover url not found"))?.to_owned();
        Ok(FromSingleLiNode::VolumeCover{cover_url})
    } else if simple_has_class(&li, "jsChapter") {
        let selector = Selector::parse("a").unwrap();
        let element = li.select(&selector).next().unwrap();
        let chapter_name = element.text().collect::<String>();
        let maybe_url = element.value().attr("href")
            .filter(|x| !x.contains("javascript"))
            .map(|x| format!("{}{}", SOURCE_URL, x));
        Ok(FromSingleLiNode::Chapter{chapter_name, chapter_url: maybe_url})
    } else {
        Ok(FromSingleLiNode::Unknown)
    }
}

fn parse_volumes_li_elements(doc: Html) -> Vec<FromSingleLiNode> {
    let selector = Selector::parse("#volumes>*.catalog-volume").unwrap();
    let lis = doc.select(&selector);
    lis.map(parse_li_element)
        .filter_map(|x| {
            match x {
                Ok(n) => Some(n),
                Err(_) => {
                    error!("Html Parse Error! Error occurs in ln_source::bili::parser::parse_li_element");
                    None
                }
            }
        })
        .collect::<Vec<_>>()
}