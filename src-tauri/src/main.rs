#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use tauri::{generate_handler, Manager, Window};
use fetcher::{json_struct::torrent_info::TorrentsInfo, nyaa::nyaa::{search_ep, Entry, search_show}};

fn main() {
  tauri::Builder::default()
	  .setup(|app| {
		  let window = app.get_window("main").unwrap();
		//   #[cfg(target_os = "macos")]
		//   apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).unwrap();

		//   #[cfg(target_os = "windows")]
		//   apply_acrylic(&window, None).unwrap();
		  Ok(())
	  })
	  .plugin(tauri_plugin_store::PluginBuilder::default().build())
	  .invoke_handler(generate_handler![get_torrents,open_in_mpv,search_torrent,download_torrents])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn get_torrents() -> Result<TorrentsInfo,Vec<()>> {
	let t_data = fetcher::qbit::QBitClient::instance.fetch_torrent_data().await;
	if t_data.is_err() {
		Err(vec![])
	} else { Ok(t_data.unwrap()) }
}

#[tauri::command(async)]
async fn download_torrents(torrents: Vec<String>,download_to:String) -> u16 {
	fetcher::qbit::QBitClient::instance.download_torrents(torrents, Some(download_to)).await
}

#[tauri::command(async)]
async fn search_torrent(window:Window,series: String,ep:i32,season:Option<i32>,search_type:i32)-> Result<Vec<Entry>,String> {
	// std::thread::spawn(move || async {
	// 	window.emit("search_torrent_result",search_ep(&series, ep).await);
	// });

	if search_type == 0 {
		println!("searching ep {ep} of {series} | {season:?}");
		if let Ok(fep) = search_ep(&series, ep,season).await {
			Ok(vec![fep])
		} else {
			Err("error".to_string())
		}

	} else if search_type == 1 {
		if let Ok(feps) = search_show(&series,ep,season).await {
			println!("search many eps of {series}:{ep} | {season:?} -> {}",feps.len());
			Ok(feps.values().cloned().collect())
		} else {
			Err("error".to_string())
		}
	} else {
		Err("error".to_string())
	}

}

#[tauri::command(async)]
fn open_in_mpv(path: String) {
  let path = std::path::PathBuf::from(path);
  let path = path.to_str().unwrap();
  let path = std::ffi::OsString::from(path);
  std::process::Command::new("mpv").arg(path).spawn().expect("Failed to open in mpv");
}