#![feature(option_result_contains)]




pub mod nyaa {
    #[derive(Debug, Clone)]
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

    const RESOLUTIONS: [&str; 6] = [
        "1080p",
        "720p",
        "480p",
        "360p",
        "240p",
        "144p",
    ];

    use std::error::Error;
    use scraper::{Html, Selector};

    pub async fn search(query: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
        let url = format!("https://nyaa.si/?f=0&c=0_0&q={}&s=seeders&o=desc", query.replace(" ", "+"));
        println!("exec : {}", url);
        let resp = reqwest::get(&url).await?
            .text()
            .await?;
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

    pub async fn search_ep(show:&str, ep:i32) -> Result<Entry,String> {
        let channel_possible = search(format!("{} {}", show, ep).as_str()).await;
        let mut best_item:Option<Entry> = None;
        let mut best_seeders:i32 = 0;
        if let Ok(entries) = channel_possible {
            for item in entries {
                let mut title_without_res = item.name.split(" ").filter(|x| {
                    let num = x.parse::<i32>();
                    if let Ok(n) = num {
                        return n == ep
                    }
                    return true
                }  ).collect::<Vec<&str>>().join(" ");
                for res in RESOLUTIONS.iter() {
                    let new_title = title_without_res.replace(res, "");
                    title_without_res = new_title.to_owned();
                }

                let title_lower = title_without_res.to_lowercase();
                // println!("{} ({}) | searching for {:?} {}", title_lower,item.name,show,ep);
                if title_lower.contains(&show.to_string().to_lowercase())
                    && title_lower.contains(&format!("{}",ep))
                    && item.seeders > best_seeders {
                    println!("{} | found {:?} {}", title_without_res,show,ep);
                    best_item = Some(item.clone());
                    best_seeders = item.seeders;
                }
            }
        } else {
            return Err(channel_possible.unwrap_err().to_string());
        }
        if let Some(item) = best_item {
            return Ok(item);
        }
        Err("No results found".to_string())
    }

}