use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest;
use tokio;
use http::StatusCode;

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



#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}



#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{


    let spotify_key = if let Some(spotify_key) = std::env::args().nth(1){
        spotify_key
    }else{
        let error_message:&str = "please provide a proper key";
        eprintln!("Error: {:?}", error_message);
        error_message.into()
       
        
    };

    let client = reqwest::Client::new();

    let spotify_user_profile_endpoint = "https://api.spotify.com/v1/me/top/tracks?time_range=long_term&limit=5";

    let spotify_response= client.get(spotify_user_profile_endpoint)
        .bearer_auth(spotify_key)
        .send()
        .await?;

    //let key_check = assert!(spotify_response.status(), StatusCode::OK);



 
    

    eprintln!("Response: {:?} {}", spotify_response.version(), spotify_response.status());
    eprintln!("Headers: {:#?}\n", spotify_response.headers());

    //let mut = HashMap::new();
    

    let body = spotify_response.text().await?;

    eprintln!("{body}");





    Ok(())

// spotify docs https://developer.spotify.com/documentation/web-api
// spotify get album api https://developer.spotify.com/documentation/web-api/reference/get-users-saved-albums
// spotify get playlists https://developer.spotify.com/documentation/web-api/reference/get-playlist

//IDEA HERE: 
// 1st: take the albums and playlists of the user^^
// // 2nd: go to a series of places to purchase from that exist
// 3rd: look at each album and see what would cost what
// 4th: look at each playlist and see what they'd cost

}
