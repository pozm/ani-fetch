// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
pub type TorrentsInfo = Vec<TorrentInfo>;

#[derive(Serialize, Deserialize)]
pub struct TorrentInfo {
    #[serde(rename = "added_on")]
    added_on: i64,

    #[serde(rename = "amount_left")]
    amount_left: i64,

    #[serde(rename = "auto_tmm")]
    auto_tmm: bool,

    #[serde(rename = "availability")]
    availability: f64,

    #[serde(rename = "category")]
    category: String,

    #[serde(rename = "completed")]
    completed: i64,

    #[serde(rename = "completion_on")]
    completion_on: i64,

    #[serde(rename = "content_path")]
    content_path: String,

    #[serde(rename = "dl_limit")]
    dl_limit: i64,

    #[serde(rename = "dlspeed")]
    dlspeed: i64,

    #[serde(rename = "download_path")]
    download_path: String,

    #[serde(rename = "downloaded")]
    downloaded: i64,

    #[serde(rename = "downloaded_session")]
    downloaded_session: i64,

    #[serde(rename = "eta")]
    eta: i64,

    #[serde(rename = "f_l_piece_prio")]
    f_l_piece_prio: bool,

    #[serde(rename = "force_start")]
    force_start: bool,

    #[serde(rename = "hash")]
    hash: String,

    #[serde(rename = "infohash_v1")]
    infohash_v1: String,

    #[serde(rename = "infohash_v2")]
    infohash_v2: String,

    #[serde(rename = "last_activity")]
    last_activity: i64,

    #[serde(rename = "magnet_uri")]
    magnet_uri: String,

    #[serde(rename = "max_ratio")]
    max_ratio: f64,

    #[serde(rename = "max_seeding_time")]
    max_seeding_time: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "num_complete")]
    num_complete: i64,

    #[serde(rename = "num_incomplete")]
    num_incomplete: i64,

    #[serde(rename = "num_leechs")]
    num_leechs: i64,

    #[serde(rename = "num_seeds")]
    num_seeds: i64,

    #[serde(rename = "priority")]
    priority: i64,

    #[serde(rename = "progress")]
    progress: f64,

    #[serde(rename = "ratio")]
    ratio: f64,

    #[serde(rename = "ratio_limit")]
    ratio_limit: i64,

    #[serde(rename = "save_path")]
    save_path: String,

    #[serde(rename = "seeding_time")]
    seeding_time: i64,

    #[serde(rename = "seeding_time_limit")]
    seeding_time_limit: i64,

    #[serde(rename = "seen_complete")]
    seen_complete: i64,

    #[serde(rename = "seq_dl")]
    seq_dl: bool,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "state")]
    state: String,

    #[serde(rename = "super_seeding")]
    super_seeding: bool,

    #[serde(rename = "tags")]
    tags: String,

    #[serde(rename = "time_active")]
    time_active: i64,

    #[serde(rename = "total_size")]
    total_size: i64,

    #[serde(rename = "tracker")]
    tracker: String,

    #[serde(rename = "trackers_count")]
    trackers_count: i64,

    #[serde(rename = "up_limit")]
    up_limit: i64,

    #[serde(rename = "uploaded")]
    uploaded: i64,

    #[serde(rename = "uploaded_session")]
    uploaded_session: i64,

    #[serde(rename = "upspeed")]
    upspeed: i64,
}
