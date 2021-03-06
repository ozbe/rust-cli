use chrono::{DateTime, Utc};
use pocket::*;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Debug)]
pub struct Item {
    pub item_id: u64,
    #[serde(with = "url_serde")]
    pub given_url: Option<Url>,
    pub given_title: Option<String>,
    pub word_count: usize,
    pub excerpt: String,
    pub time_added: Option<DateTime<Utc>>,
    pub time_read: Option<DateTime<Utc>>,
    pub time_updated: Option<DateTime<Utc>>,
    pub time_favorited: Option<DateTime<Utc>>,
    pub favorite: Option<bool>,
    pub is_index: bool,
    pub is_article: bool,
    pub has_image: ItemHas,
    pub has_video: ItemHas,
    pub resolved_id: u64,
    pub resolved_title: Option<String>,
    #[serde(with = "url_serde")]
    pub resolved_url: Option<Url>,
    pub sort_id: Option<u64>,
    pub status: Option<ItemStatus>,
    pub tags: Option<Vec<Tag>>,
    pub images: Option<Vec<Image>>,
    pub videos: Option<Vec<Video>>,
    pub authors: Option<Vec<Author>>,
    pub lang: Option<String>,
    pub time_to_read: Option<u64>,
    pub domain_metadata: Option<DomainMetadata>,
    pub listen_duration_estimate: Option<u64>,
    pub image: Option<Image>,
    #[serde(with = "url_serde")]
    pub amp_url: Option<Url>,
    #[serde(with = "url_serde")]
    pub top_image_url: Option<Url>,
}

#[derive(Debug, Serialize)]
pub struct Image {
    pub item_id: u64,
    pub image_id: Option<u64>,
    #[serde(with = "url_serde")]
    pub src: Option<Url>,
    pub width: u16,
    pub height: u16,
    pub credit: Option<String>,
    pub caption: Option<String>,
}

impl From<ItemImage> for Image {
    fn from(i: ItemImage) -> Self {
        Image {
            item_id: i.item_id,
            image_id: None,
            src: i.src,
            width: i.width,
            height: i.height,
            credit: None,
            caption: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DomainMetadata {
    pub name: Option<String>,
    pub logo: String,
    pub greyscale_logo: String,
}

impl From<DomainMetaData> for DomainMetadata {
    fn from(d: DomainMetaData) -> Self {
        DomainMetadata {
            name: d.name,
            logo: d.logo,
            greyscale_logo: d.greyscale_logo,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Tag(String);

impl From<ItemTag> for Tag {
    fn from(t: ItemTag) -> Self {
        Tag(t.tag)
    }
}

#[derive(Debug, Serialize)]
pub struct Video {
    pub item_id: u64,
    pub video_id: u64,
    #[serde(with = "url_serde")]
    pub src: Option<Url>,
    pub width: u16,
    pub height: u16,
    pub length: Option<usize>,
    pub vid: String,
    #[serde(rename = "type")]
    pub vtype: u16,
}

impl From<ItemVideo> for Video {
    fn from(v: ItemVideo) -> Self {
        Video {
            item_id: v.item_id,
            video_id: v.video_id,
            src: v.src,
            width: v.width,
            height: v.height,
            length: v.length,
            vid: v.vid,
            vtype: v.vtype,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub item_id: u64,
    pub author_id: u64,
    pub name: String,
    pub url: String,
}

impl From<ItemAuthor> for Author {
    fn from(a: ItemAuthor) -> Self {
        Author {
            item_id: a.item_id,
            author_id: a.author_id,
            name: a.name,
            url: a.url,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ItemHas {
    No,
    Yes,
    Is,
}

impl From<PocketItemHas> for ItemHas {
    fn from(h: PocketItemHas) -> Self {
        match h {
            PocketItemHas::No => ItemHas::No,
            PocketItemHas::Yes => ItemHas::Yes,
            PocketItemHas::Is => ItemHas::Is,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ItemStatus {
    Normal,
    Archived,
    Deleted,
}

impl From<PocketItemStatus> for ItemStatus {
    fn from(s: PocketItemStatus) -> Self {
        match s {
            PocketItemStatus::Normal => ItemStatus::Normal,
            PocketItemStatus::Archived => ItemStatus::Archived,
            PocketItemStatus::Deleted => ItemStatus::Deleted,
        }
    }
}

#[derive(Serialize)]
pub struct IndividualSendResponse {
    pub status: u16,
    pub success: bool,
}

impl From<PocketSendResponse> for IndividualSendResponse {
    fn from(p: PocketSendResponse) -> Self {
        IndividualSendResponse {
            status: p.status,
            success: p.action_results[0],
        }
    }
}
