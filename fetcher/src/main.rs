use fetcher::nyaa::nyaa::{search_ep, search_show};
use fetcher::qbit::qbit::{download_torrent, download_torrents, download_torrents_str};

#[tokio::main]
async fn main() {

    // let ep_pos = search_ep("sono bisque doll wa koi wo suru",7).await;
    //
    // if let Ok(ep) = ep_pos {
    //     println!("{:?}", ep);
    //     let status = download_torrent(ep.link.as_str(),"localhost:8080").await;
    //     println!("{:?}", status);
    // } else {
    //     println!("{:?}", ep_pos);
    // }

    let eps = search_show("sono bisque doll wa koi wo suru", 10).await;
    if let Ok(eps) = eps {
        let links = eps.iter().map(|ep| ep.1.link.as_str()).collect::<Vec<&str>>();
        println!("{:#?}", links);
        download_torrents_str(links, "localhost:8080").await;
    } else {
        println!("{:?}", eps);
    }
}
