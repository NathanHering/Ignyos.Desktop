// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::system::get_os_info,
            commands::window::minimize,
            commands::window::toggle_maximize,
            commands::window::close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
