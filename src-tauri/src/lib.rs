// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod user;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![user::user_list::user_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
