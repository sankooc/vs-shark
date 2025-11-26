// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use tokio::runtime::Runtime;
use util::core::{EngineCommand, UIEngine};
use util::PFile;

use crate::command::api::*;
mod command;
// mod core;
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecentFile {
    path: String,
    name: String,
}

pub struct GUIContext {
    pub ui: UIEngine,
}

impl GUIContext {
    fn new(ui: UIEngine) -> Self {
        Self { ui }
    }
    pub fn engine(&self) -> &UIEngine {
        &self.ui
    }
    // pub fn has_file(&self) -> bool {
    //     self.file.is_some()
    // }
    // pub fn set_file(&mut self, file: Option<String>) {
    //     self.file = file;
    // }
}
#[tauri::command]
fn frontend_ready(app: AppHandle) {
    let splash = app.get_webview_window("splashscreen").unwrap();
    let main = app.get_webview_window("main").unwrap();
    splash.close().unwrap();
    main.show().unwrap();
}

#[tauri::command]
async fn open_file_dialog(app_handle: AppHandle) -> Result<Option<String>, String> {
    println!("open_file_dialog called");
    let file_path = app_handle.dialog().file().add_filter("PCAP Files", &["pcap", "pcapng", "cap"]).blocking_pick_file();
    let context: State<GUIContext> = app_handle.state();
    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            if let Some(pf) = PFile::new(&path_str) {
                app_handle.emit("file_touch", &pf).unwrap();
            }
            if context.engine().open_file(path_str.clone()).await.is_ok() {
                app_handle.emit("parse_complete", true).unwrap();
                rebuild_menu(&app_handle, Some(path_str.clone())).map_err(|e| e.to_string())?;
                Ok(Some(path_str))
            } else {
                app_handle.emit("parse_complete", false).unwrap();
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

fn rebuild_menu(app_handle: &AppHandle, pcap_file: Option<String>) -> tauri::Result<()> {

    let mut file = SubmenuBuilder::new(app_handle, "File");
    if pcap_file.is_some() {
        let open_item = MenuItemBuilder::new("Close").id("close").build(app_handle)?;
        file = file.item(&open_item);
    } else {
        let open_item = MenuItemBuilder::new("Open").id("open").build(app_handle)?;
        file = file.item(&open_item);
    }
    let file_menu = file.build()?;

    let edit_item = MenuItemBuilder::new("Edit").build(app_handle)?;

    let menu = MenuBuilder::new(app_handle).items(&[&file_menu, &edit_item]).build()?;
    app_handle.set_menu(menu)?;
    Ok(())
}

pub fn run() {
    let (ui, mut engine, mut rx) = util::core::build_engine();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            engine.run().await;
        });
    });
    let context = GUIContext::new(ui);

    use command::api::ready;
    
    tauri::Builder::default()
        .manage(context)
        .setup(|app| {
            
            // app.manage(RecentFiles::default());
            let args: Vec<String> = std::env::args().collect();
            let mut option = None;
            if args.len() > 1 {
                let file_path = args[1].clone();
                option = Some(file_path);
            }
            rebuild_menu(app.handle(), option)?;
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(msg) = rx.recv().await {
                        match msg {
                            EngineCommand::Quit => break,
                            EngineCommand::None => {}
                            EngineCommand::Error(err) => {
                                app_handle.emit("error", &err).unwrap();
                            }
                            EngineCommand::Progress(progress) => {
                                app_handle.emit("progress", &progress).unwrap();
                            }
                        }
                    } else {
                        println!("waiting next");
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            });
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => {
                let app_handle = app.clone();
                tauri::async_runtime::spawn(async move { open_file_dialog(app_handle).await });
            }
            "close" => {
                let app_handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    let context: State<GUIContext> = app_handle.state();
                    let _ = context.engine().close_file().await;
                    app_handle.emit("file_close", ()).unwrap();
                    rebuild_menu(&app_handle, None).unwrap();
                });
            }
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            frontend_ready,
            ready,
            frames,
            frame,
            stat,
            tcp_list,
            tcp_conv_list,
            udp_list,
            http_list,
            http_detail,
            dns_records,
            dns_record,
            tls_list,
            tls_conv_list,
            open_file_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
