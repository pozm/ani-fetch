#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use tauri::{generate_handler, Manager, Window};
use window_vibrancy::apply_acrylic;
use fetcher::{json_struct::torrent_info::TorrentsInfo, nyaa::nyaa::{search_ep, Entry}};

fn main() {
  tauri::Builder::default()
	  .setup(|app| {
		  let window = app.get_window("main").unwrap();
		  #[cfg(target_os = "macos")]
		  apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).unwrap();

		//   #[cfg(target_os = "windows")]
		//   apply_acrylic(&window, None).unwrap();
		  Ok(())
	  })
	  .plugin(tauri_plugin_store::PluginBuilder::default().build())
	  .invoke_handler(generate_handler![get_torrents,open_in_mpv,search_torrent])
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
async fn search_torrent(window:Window,series: String,ep:i32)-> Result<Entry,String> {
	// std::thread::spawn(move || async {
	// 	window.emit("search_torrent_result",search_ep(&series, ep).await);
	// });
	search_ep(&series, ep,None).await
}

#[tauri::command(async)]
fn open_in_mpv(path: String) {
  let path = std::path::PathBuf::from(path);
  let path = path.to_str().unwrap();
  let path = std::ffi::OsString::from(path);
  std::process::Command::new("mpv").arg(path).spawn().expect("Failed to open in mpv");
}