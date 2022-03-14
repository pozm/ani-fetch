pub mod qbit {
    const NEW_TORRENT: &'static str = "/api/v2/torrents/add";
    pub async fn download_torrent(torrent_url: &str, host:&str) -> u16 {

        let client = reqwest::Client::new();

        let safe = torrent_url.to_string().clone();
        let form = reqwest::multipart::Form::new().text("urls", safe);

        let outcome = client.post(format!("http://{}{}", host, NEW_TORRENT).as_str())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .multipart(form)
            .send().await
            .unwrap();
        outcome.status().as_u16()
    }

}