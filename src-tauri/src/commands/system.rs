#[tauri::command]
pub fn get_os_info() -> String {
    let os_info = os_info::get();
    format!("{} {}", os_info.os_type(), os_info.version())
}