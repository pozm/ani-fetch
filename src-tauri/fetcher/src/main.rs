use fetcher::nyaa::nyaa::{get_data, search_ep, search_show};
use fetcher::qbit::qbit::{download_torrent, download_torrents, download_torrents_str};

#[tokio::main]
async fn main() {

    // let ep_pos = search_ep("sono bisque doll wa koi wo suru",7,None).await;
    
    // if let Ok(ep) = ep_pos {
    //     println!("{:?}", ep);
    //     let status = download_torrent(ep.link.as_str(),"localhost:8080").await;
    //     println!("{:?}", status);
    // } else {
    //     println!("{:?}", ep_pos);
    // }

    let eps = search_show("Summertime rendering", 7,None).await;
    if let Ok(eps) = eps {
        // let links = eps.iter().map(|ep| ep.1.link.as_str()).collect::<Vec<&str>>();
        let names = eps.iter().map(|ep| (ep.1.name.as_str(),ep.1.seeders)).collect::<Vec<(&str,i32)>>();
        println!("{:#?}",names);
        // download_torrents_str(links, "localhost:1112").await;
    } else {
        println!("{:?}", eps);
    }

	// let data = get_data(1421679).await;
    // println!("da {:?}",data);
}
