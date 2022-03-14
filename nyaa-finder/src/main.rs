use nyaa_finder::nyaa::{search, search_ep};

#[tokio::main]
async fn main() {

    let ep_pos = search_ep("sono bisque doll wa koi wo suru",7).await;

    if let Ok(ep) = ep_pos {
        println!("{:?}", ep);
        let status = qbittorrent::qbit::download_torrent(ep.link.as_str(),"localhost:8080").await;
        println!("{:?}", status);
    } else {
        println!("{:?}", ep_pos);
    }

    println!("Hello, world!");
}
