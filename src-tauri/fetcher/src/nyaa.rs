#![feature(option_result_contains)]



pub mod nyaa {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::iter::Map;
    use scraper::{Html, Selector};
    use serde::{Deserialize, Serialize};
	use regex::Regex;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Entry {
        pub category: String,
        pub name: String,
        pub link: String,
        pub size: String,
        pub date: String,
        pub seeders: i32,
        pub leechers: i32,
        pub download_count: i32,
    }

    // impl Display for Entry {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //         f.write_str(self.name.as_str())
    //     }
    // }

    const RESOLUTIONS: [&str; 6] = [
        "1080p",
        "720p",
        "480p",
        "360p",
        "240p",
        "144p",
    ];


	fn make_folder_list(html:&Html) {
		let folder_selector = Selector::parse("body > div > div:nth-child(3) > div.torrent-file-list.panel-body > ul > li").unwrap();
		let folder_select_many = Selector::parse("ul > li").unwrap();
		let folder_name = Selector::parse("a").unwrap();
		let folder_list = html.select(&folder_selector);
		for element in folder_list {
			let folder_name = element.select(&folder_name).next().unwrap().text().next().unwrap();
			let children = element.select(&folder_select_many);
			println!("V {:?} V\n{}",folder_name, children.map(|x| x.text().next().unwrap()).collect::<Vec<_>>().join("\n"));
		}
	}

	pub async fn get_data(id:i32) -> Result<(), Box<dyn Error>> {
		let url = format!("https://nyaa.si/view/{}", id);
		println!("exec : {}", url);
		let resp = reqwest::get(&url).await?
			.text()
			.await?;
		let html = Html::parse_document(&resp);
		make_folder_list(&html);
		// println!("{:#?}", folder_list);
		Ok(())

	}

    pub async fn search(query: &str) -> Result<Vec<Entry>, ()> {
        let url = format!("https://nyaa.si/?f=0&c=1_2&q={}&s=seeders&o=desc", query.replace(" ", "+"));
        println!("exec : {}", url);
        let resp = reqwest::get(&url).await.unwrap()
            .text()
            .await.unwrap();
        let html = Html::parse_document(&resp);
        let mut entries = vec![];

        let table_selector = Selector::parse("body > div > div.table-responsive > table > tbody > tr").unwrap();

        let select_items = html.select(&table_selector);
        let name_selector  = Selector::parse("td:nth-child(2) > a:not(.comments)").unwrap();
        let magnet_selector  = Selector::parse("td:nth-child(3) > a:last-child").unwrap();
        let size_selector  = Selector::parse("td:nth-child(4)").unwrap();
        let date_selector  = Selector::parse("td:nth-child(5)").unwrap();
        let seeds_selector  = Selector::parse("td:nth-child(6)").unwrap();
        let leeches_selector  = Selector::parse("td:nth-child(7)").unwrap();
        let downloads_selector  = Selector::parse("td:nth-child(8)").unwrap();


        for element in select_items {
            let name = element.select(&name_selector).next().unwrap().text().next().unwrap();
            let magnet = element.select(&magnet_selector).next().unwrap().value().attr("href").unwrap();
            let size = element.select(&size_selector).next().unwrap().text().next().unwrap();
            let date = element.select(&date_selector).next().unwrap().text().next().unwrap();
            let seeds = element.select(&seeds_selector).next().unwrap().text().next().unwrap();
            let leeches = element.select(&leeches_selector).next().unwrap().text().next().unwrap();
            let downloads = element.select(&downloads_selector).next().unwrap().text().next().unwrap();
            let entry = Entry {
                date:date.to_string(),
                name:name.to_string(),
                link:magnet.to_string(),
                size:size.to_string(),
                seeders:seeds.parse().unwrap(),
                leechers:leeches.parse().unwrap(),
                download_count:downloads.parse().unwrap(),
                category: "".to_string(),
            };
            entries.push(entry);
        }

        Ok(entries)
    }
    // pub fn parse_entry_name(name:String, ep:i32,season:Option<i32>) -> String {
    // //     let mut title_without_res = name.split(" ").filter(|x| {
    // //         let num = x.parse::<i32>();
    // //         if let Ok(n) = num {
    // //             return n == ep
    // //         }
    // //         return true
    // //     }  ).collect::<Vec<&str>>().join(" ");
    // //     for res in RESOLUTIONS.iter() {
    // //         let new_title = title_without_res.replace(res, "");
    // //         title_without_res = new_title.to_owned(); 
    // //     }
    // //    title_without_res.to_lowercase()
	// 	lazy_static! {
	// 		static ref RE: Regex = Regex::new(r#"(?mi)^(?<uploader> ?\[.*?\])? ?(?<series>[a-zA-Z-\s()]+?) (?<seasonep>(?<season>s\d+|season \d+)? ?-? ?(?<ep>e\d+|\d+))"#).unwrap();
	// 	}
	// 	let caps = RE.captures(&name).unwrap();
	// 	let series = caps.name("series").unwrap().as_str();
	// 	let ep = caps.name("ep").unwrap().as_str();
	// 	let season = caps.name("season").unwrap().as_str();
	// 	series.to_string()
    // }
    pub async fn search_ep_with_entries(entries:Vec<Entry>,show:&str,ep:i32,season:Option<i32>) -> Result<Entry,String> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r#"(?mi)^(?P<uploader> ?\[.*?\])? ?(?P<series>[a-zA-Z-\s()]+?) (?P<seasonep>(?P<season>s\d+|season \d+)? ?-? ?(?P<ep>e\d+|\d+))"#).unwrap();
		}
        let mut best_item:Option<Entry> = None;
        let mut best_seeders:i32 = 0;
        for item in entries {
			let mut title_lower = "".to_string();
			let mut got_ep = -1;
			let mut got_sea = -1;
            if let Some(caps) = RE.captures(&item.name) {
				title_lower = (&caps)["series"].to_lowercase();
				got_ep = i32::from_str_radix(&caps["ep"].chars().filter(|x| x.is_digit(10)).collect::<String>(),10).unwrap_or(-1);
				// let got_sea = i32::from_str_radix(&caps["season"].chars().filter(|x| x.is_digit(10)).collect::<String>(),10).unwrap_or(-1);
				if season.is_some() && let Some(m_got_season) = caps.name("season") {
					let digits = m_got_season.as_str().chars().filter(|x| x.is_digit(10)).collect::<String>();
					got_sea = i32::from_str_radix(&digits,10).unwrap_or(-1);
				}
				// println!("{} ({}) | searching for {:?} {}", title_lower,item.name,show,ep);
			}
            if title_lower.contains(&show.to_string().to_lowercase())
                && got_ep == ep
				&& if season.is_some() {got_sea == season.unwrap()} else {true}
                && item.seeders > best_seeders {
                println!("{} | found {:?} {}", title_lower,show,ep);
                best_item = Some(item.clone());
                best_seeders = item.seeders;
            }
        }
        if let Some(item) = best_item {
            return Ok(item);
        }
        Err("No results found".to_string())
    }
    pub async fn search_ep(show:&str, ep:i32,season:Option<i32>) -> Result<Entry,String> {
        let channel_possible = search(format!("{} {}", show, ep).as_str()).await;
        if let Ok(entries) = channel_possible {
            search_ep_with_entries(entries, show, ep,season).await
        } else {
            Err("No results found".to_string())
        }
    }

    pub async fn search_show(show:&str,total_eps:i32,season:Option<i32>) -> Result<HashMap<i32,Entry>,String> {

        let results_pos = search(show).await;
		lazy_static! {
			static ref RE: Regex = Regex::new(r#"(?mi)^(?P<uploader> ?\[.*?\])? ?(?P<series>[a-zA-Z-\s()]+?) (?P<seasonep>(?P<season>s\d+|season \d+)? ?-? ?(?P<ep>e\d+|\d+))"#).unwrap();
		}
        let mut eps:HashMap<i32,Entry> = HashMap::new();
        if let Ok(results) = results_pos {
            for entry in results {
				let mut title_lower = "".to_string();
				let mut got_ep = -1;
				let mut got_sea = -1;
				if let Some(caps) = RE.captures(&item.name) {
					title_lower = (&caps)["series"].to_lowercase();
					got_ep = i32::from_str_radix(&caps["ep"].chars().filter(|x| x.is_digit(10)).collect::<String>(),10).unwrap_or(-1);
					// let got_sea = i32::from_str_radix(&caps["season"].chars().filter(|x| x.is_digit(10)).collect::<String>(),10).unwrap_or(-1);
					if season.is_some() && let Some(m_got_season) = caps.name("season") {
						let digits = m_got_season.as_str().chars().filter(|x| x.is_digit(10)).collect::<String>();
						got_sea = i32::from_str_radix(&digits,10).unwrap_or(-1);
					}
					// println!("{} ({}) | searching for {:?} {}", title_lower,item.name,show,ep);
				}
				if let Some(ep) = ep {
                    let ep = ep.parse::<i32>().unwrap();
                    println!("{} | {}", entry.name, ep);
                    let entry_there = eps.get(&ep);
                    if let Some(existing_entry) = entry_there {
                        if existing_entry.seeders > entry.seeders || ep > total_eps {
                            continue;
                        }
                    }
                    eps.insert(ep,entry);
                } else {
					println!("{} | no ep (attempting to all in 1 😎)", entry.name);
					let entry_there = eps.get(&0);
					if let Some(existing_entry) = entry_there {
						if existing_entry.seeders > entry.seeders {
							continue;
						}
					}

				}
            }
        }

        Ok(eps)
    }

}