use std::vec;
use reqwest::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Podcast>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Podcast {
    #[serde(rename = "_id")]
    pub id: String,
    pub thumbnail: Thumbnail,
    #[serde(rename = "last_published")]
    pub last_published: String,
    pub description: String,
    pub title: String,
    #[serde(rename = "show_title")]
    pub show_title: String,
    #[serde(rename = "fn__media_tags")]
    pub fn_media_tags: Vec<String>,
    #[serde(rename = "live_stream")]
    pub live_stream: bool,
    #[serde(rename = "fn__tve_authentication_required")]
    pub fn_tve_authentication_required: bool,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(rename = "bc__account_id")]
    pub bc_account_id: String,
    #[serde(rename = "audio_only")]
    pub audio_only: bool,
    pub streams: Vec<Stream>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub url: String,
}


// use tracing::Level;
trait LogErr {
    fn log_err(self, msg: &'static str) -> Self;
}
impl<T, E> LogErr for Result<T, E>
where
    E: std::fmt::Display,
{
    fn log_err(self, msg: &'static str) -> Self {
        if let Err(ref e) = self {
            tracing::error!("{}: {}", msg, e);
        }
        self
    }
}

use std::collections::HashMap;
fn main() -> Result<(), Error> {
    increaser();
    Ok(())
}

fn increaser() {
    let resp = hit_api(); //.log_err("blah"); 
    // let mut mult = Vec::new();

    // for mut x in resp.unwrap().data {
    //     let k = "1";
    //     x.title = k.to_string();
    //     mult.push(x);
    // }
    // println!("{:#?}", mult);
    let m = convert_to_map(resp.unwrap());
    println!("{:#?}", m)

}

fn hit_api() -> Result<Root, Error> {
    let url = "https://feeds.foxnews.com/podcasts?size=2&from=0&tag=on_air%7Cpodcast%7Cfox_news_radio";
    let resp = reqwest::blocking::get(url)?
        .json::<Root>();
    Ok(resp?)
}

fn convert_to_map(data: Root) -> HashMap<i32, Podcast> {
    let mut map = HashMap::new();
    for (i, pc) in data.data.into_iter().enumerate() {
        map.insert(i as i32, pc);
    }
    map
}
