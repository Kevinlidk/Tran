#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use manager::api;
use manager::api::TransVO;
use resp::Resp;
use tauri::utils::acl::ExecutionContext;

mod clip;
mod common;
mod config;
mod lang;
mod manager;
mod resp;
mod setup;
mod shortcut;
mod tray;
mod util;
mod window;

/// 翻译文本
///
/// Translate text
#[tauri::command]
async fn translate(content: String) -> Resp<TransVO> {
    api::translate(&content).await.into()
}

/// 写入剪贴板
///
/// Write to clipboard
#[tauri::command]
fn copy(content: String) -> Resp<()> {
    clip::set(content).into()
}

/// 打开指定链接
///
/// Open the specified link
#[tauri::command]
async fn open(url: String) -> Resp<()> {
    open::that(url).map_err(anyhow::Error::msg).into()
}

/// 固定窗口标识
#[tauri::command]
async fn pin(state: bool) {
    common::PIN.store(state, std::sync::atomic::Ordering::SeqCst);
}

#[tokio::main]
async fn main() {
    // 全局初始化
    // Global initialization
    common::init().await;

    let mut context = tauri::generate_context!("tauri.conf.json");

    for cmd in [
        "plugin:app|version",
        "plugin:event|listen",
        "plugin:window|show",
        "plugin:window|hide",
        "plugin:window|set_size",
        "plugin:window|set_focus",
        "plugin:window|set_position",
        "plugin:window|internal_on_mousemove",
        "plugin:window|internal_on_mousedown",
        "plugin:window|start_dragging",
    ] {
        context
            .runtime_authority_mut()
            .__allow_command(cmd.to_string(), ExecutionContext::Local);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![copy, open, translate, pin,])
        .run(context)
        .expect("error while running tauri application");
}
