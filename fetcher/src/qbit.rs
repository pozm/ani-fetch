use std::borrow::Borrow;
use reqwest::Response;
use crate::json_struct::torrent_info::TorrentsInfo;

pub mod qbit {
    const NEW_TORRENT: &'static str = "/api/v2/torrents/add";
    pub async fn download_torrent(torrent_url: &str, host:&str) -> u16 {

        let client = reqwest::Client::new();

        let safe = torrent_url.to_string().clone();
        let form = reqwest::multipart::Form::new()
            .text("urls", safe)
            .text("category", "anime");

        let outcome = client.post(format!("http://{}{}", host, NEW_TORRENT).as_str())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .multipart(form)
            .send().await
            .unwrap();
        outcome.status().as_u16()
    }
    pub async fn download_torrents(torrents: Vec<String>, host:&str) -> u16 {
        download_torrent(torrents.join("\n").as_str(), host).await
    }
    pub async fn download_torrents_str(torrents: Vec<&str>, host:&str) -> u16 {
        download_torrent(torrents.join("\n").as_str(), host).await
    }
}

pub struct QBitClient<'a> {
    host: &'a str,
}
impl<'a> QBitClient<'a> {
    const fn new(host: &'a str) -> Self {
        QBitClient {
            host: host,
        }
    }
    pub const instance: QBitClient<'static> = QBitClient::new("localhost:8080");
    pub fn set_host(&mut self, host: &'a str) {
        self.host = host;
    }

    pub fn get_url(&mut self, url: &str) -> String {
        format!("http://{}{}", self.host, url)
    }

    pub async fn fetch_torrent_data(&mut self) -> Result<TorrentsInfo, reqwest::Error> {
        let http = reqwest::Client::new();
        Ok(http.get(self.get_url("/api/v2/torrents/info").as_str())
            .send().await?
            .json::<TorrentsInfo>().await.unwrap())
    }

    pub async fn download_torrents(&self, torrents: Vec<String>) -> u16 {
        qbit::download_torrents(torrents, &self.host).await
    }
    pub async fn download_torrent(&self, torrent: &str) -> u16 {
        qbit::download_torrent(torrent, &self.host).await
    }
    pub async fn download_torrents_str(&self, torrents: Vec<&str>) -> u16 {
        qbit::download_torrents_str(torrents, &self.host).await
    }
}