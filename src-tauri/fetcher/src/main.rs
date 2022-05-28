use fetcher::nyaa::nyaa::{get_data, search_ep, search_show};
use fetcher::qbit::qbit::{download_torrent, download_torrents, download_torrents_str};

#[tokio::main]
async fn main() {

    let ep_pos = search_ep("spy x family",8,None).await;
    
    if let Ok(ep) = ep_pos {
        println!("{:#?}", ep);
        // let status = download_torrent(ep.link.as_str(),"localhost:8080",Some(format!("D:\\pog\\anime\\{}\\Season {}",ep.matched_title,1))).await;
        // println!("{:?}", status);
    } else {
        println!("err {:?}", ep_pos);
    }
	// let show_name = "Spy x Family";
	// let show_eps = 7;
	// let show_season: Option<i32> = None;

    // let eps = search_show(show_name, show_eps,show_season).await;
    // if let Ok(eps) = eps {
    //     let links = eps.iter().map(|ep| ep.1.link.as_str()).collect::<Vec<&str>>();
    //     let names = eps.iter().map(|ep| (ep.1.name.as_str(),ep.1.seeders,ep.1.matched_title.as_str())).collect::<Vec<(&str,i32,&str)>>();
    //     println!("{:#?}",names);
    //     download_torrents_str(links, "localhost:1112",Some(format!("D:\\pog\\anime\\{}\\Season {}",names.first().unwrap().2,show_season.unwrap_or(1)))).await;
    // } else {
    //     println!("{:?}", eps);
    // }

	// let data = get_data(1421679).await;
    // println!("da {:?}",data);
}
