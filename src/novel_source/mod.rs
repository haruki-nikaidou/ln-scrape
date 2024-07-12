mod bili_novel;

use std::sync::{Arc};

#[async_trait::async_trait]
trait NovelSource {
    fn url_belongs_to_source(url: &str) -> bool;
    async fn get_novel_profile(home_url: &str) -> Result<NovelProfile, NovelSourceError>;

    async fn get_novel_catalog(profile: NovelProfile) -> Result<NovelCatalog, NovelSourceError>;

    async fn download_chapter_content(volume: &NovelVolumeInfo) -> Result<Vec<DownloadedChapter>, NovelSourceError>;

    async fn download_image(image_url: &str) -> Result<Vec<u8>, NovelSourceError>;
}

#[derive(Clone)]
struct NovelProfile {
    pub id: String,
    pub title: String,
    pub author: String,
    pub cover_image: Option<String>,
    pub tags: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub description: String,
}

struct NovelCatalog {
    pub profile: Arc<NovelProfile>,
    pub volumes: Vec<NovelVolumeInfo>,
}

struct NovelVolumeInfo {
    pub index: usize,
    pub novel_profile: Arc<NovelProfile>,
    pub volume_name: String,
    pub cover_image: Option<String>,
    pub chapter_titles: Vec<String>,
}

struct DownloadedChapter {
    pub title: String,
    pub content: String,
}

struct DownloadedVolume {
    pub volume_name: String,
    pub chapters: Vec<DownloadedChapter>,
    pub cover_image: Option<Vec<u8>>,
    pub profile: Arc<NovelProfile>,
}

impl DownloadedVolume {
    fn from_chapters(
        volume_name: String,
        chapters: Vec<DownloadedChapter>,
        cover_image: Option<Vec<u8>>,
        profile: Arc<NovelProfile>,
    ) -> Self {
        Self {
            volume_name,
            chapters,
            cover_image,
            profile,
        }
    }
}

enum NovelSourceError {
    InvalidUrl(String),
    NetworkError(reqwest::Error),
    ParseError(String),
}