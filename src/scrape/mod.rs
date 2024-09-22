mod image;
mod source;

use std::ptr;
use reqwest::Url;

pub use source::LightNovelSource;
pub use image::*;

#[derive(Debug)]
pub struct NovelInfo {
    pub url: Url,
    pub id: String,
    pub title: String,
    pub author: String,
    pub status: String,
    pub cover: Option<String>,
    pub tags: Vec<String>,
    pub publisher: Option<String>,
    pub description: String,
}

#[derive(Debug)]
pub struct Catalog<'a> {
    pub novel: NovelInfo,
    pub volumes: Vec<Volume<'a>>,
}

#[derive(Debug)]
pub struct Volume<'a> {
    pub volume_name: String,
    pub belongs_to_catalog: &'a Catalog<'a>,
    pub chapters: Vec<Chapter<'a>>,
    pub cover: Option<String>,
}

impl<'a> PartialEq<Self> for Volume<'a> {
    fn eq(&self, other: &Self) -> bool {
        let name_same = self.volume_name == other.volume_name;
        let catalog_same = ptr::eq(self.belongs_to_catalog, other.belongs_to_catalog);
        let chapters_same = self.chapters == other.chapters;
        let cover_same = self.cover == other.cover;
        name_same && catalog_same && chapters_same && cover_same
    }
}

impl<'a> Eq for Volume<'a> {}

#[derive(Debug)]
pub struct Chapter<'a> {
    pub chapter_name: String,
    pub url: &'a Url,
    pub content: Option<String>,
    pub belongs_to_volume: &'a Volume<'a>,
}

impl<'a> PartialEq<Self> for Chapter<'a> {
    fn eq(&self, other: &Self) -> bool {
        let name_same = self.chapter_name == other.chapter_name;
        let url_same = self.url == other.url;
        let content_same = self.content == other.content;
        let volume_same = ptr::eq(self.belongs_to_volume, other.belongs_to_volume);
        name_same && url_same && content_same && volume_same
    }
}

impl<'a> Eq for Chapter<'a> {}

