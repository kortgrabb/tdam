// Prevents additional console window on Windows in release, DO NOT REMOVE!!

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
