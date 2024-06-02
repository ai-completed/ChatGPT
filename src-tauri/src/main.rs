// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod auto_dev_plugin; 

use core::{cmd, setup};
use auto_dev_plugin::init as auto_dev_plugin_init; 

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(auto_dev_plugin_init())
        .invoke_handler(tauri::generate_handler![
            cmd::view_reload,
            cmd::view_url,
            cmd::view_go_forward,
            cmd::view_go_back,
            cmd::set_view_ask,
            cmd::get_app_conf,
            cmd::window_pin,
            cmd::ask_sync,
            cmd::ask_send,
            cmd::set_theme,
            auto_dev_plugin::init_auto_dev,
        ])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running lencx/ChatGPT application");
}
