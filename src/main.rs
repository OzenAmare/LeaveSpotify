use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i64,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<LinkedTrack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: i32,
    pub preview_url: Option<String>,
    pub track_number: i32,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedAlbum {
    pub album_type: String,
    pub total_tracks: i32,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
    pub artists: Vec<SimplifiedArtist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restrictions {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedTrack {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}


fn main() {
// spotify docs https://developer.spotify.com/documentation/web-api
// spotify get album api https://developer.spotify.com/documentation/web-api/reference/get-users-saved-albums
// spotify get playlists https://developer.spotify.com/documentation/web-api/reference/get-playlist
}
