// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use tokio::runtime::Runtime;
use util::core::UIEngine;

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
}

#[derive(Debug, Default)]
pub struct RecentFiles {
    files: Mutex<VecDeque<RecentFile>>,
    max_count: usize,
}

impl RecentFiles {
    pub fn add_file(&self, file_path: String) {
        let mut files = self.files.lock().unwrap();

        let file_name = std::path::Path::new(&file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&file_path)
            .to_string();

        let recent_file = RecentFile {
            path: file_path.clone(),
            name: file_name,
        };

        files.retain(|f| f.path != file_path);

        files.push_front(recent_file);

        while files.len() > self.max_count {
            files.pop_back();
        }
    }

    pub fn get_files(&self) -> Vec<RecentFile> {
        self.files.lock().unwrap().iter().cloned().collect()
    }
}

#[tauri::command]
async fn open_file_dialog(app_handle: AppHandle) -> Result<Option<String>, String> {
    let file_path = app_handle.dialog().file().add_filter("PCAP Files", &["pcap", "pcapng", "cap"]).blocking_pick_file();

    let context: State<GUIContext> = app_handle.state();
    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            let _ = context.engine().open_file(path_str.clone()).await;
            app_handle.emit("file_opened", &path_str).unwrap();
            let recent_files: State<RecentFiles> = app_handle.state();
            recent_files.add_file(path_str.clone());
            rebuild_menu(&app_handle, Some(path_str.clone())).map_err(|e| e.to_string())?;
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

// #[tauri::command]
// async fn open_recent_file(app_handle: AppHandle, file_path: String) -> Result<(), String> {
//     if !std::path::Path::new(&file_path).exists() {
//         app_handle.dialog().message("File not found!").kind(MessageDialogKind::Error).blocking_show();
//         return Err("File not found".to_string());
//     }

//     let recent_files: State<RecentFiles> = app_handle.state();
//     recent_files.add_file(file_path.clone());

//     rebuild_menu(&app_handle).map_err(|e| e.to_string())?;

//     println!("Opening recent file: {}", file_path);

//     Ok(())
// }

fn rebuild_menu(app_handle: &AppHandle, pcap_file: Option<String>) -> tauri::Result<()> {
    // let recent_files: State<RecentFiles> = app_handle.state();
    // let files = recent_files.get_files();

    // let mut recent_submenu = SubmenuBuilder::new(app_handle, "Open Recent");

    // if files.is_empty() {
    //     let no_recent_item = MenuItemBuilder::new("No recent files").enabled(false).build(app_handle)?;
    //     recent_submenu = recent_submenu.item(&no_recent_item);
    // } else {
    //     for (index, file) in files.iter().enumerate() {
    //         let item = MenuItemBuilder::new(&file.name).id(&format!("recent_{}", index)).build(app_handle)?;
    //         recent_submenu = recent_submenu.item(&item);
    //     }
    // }

    // let recent_menu = recent_submenu.build()?;

    let mut file = SubmenuBuilder::new(app_handle, "File");
    if pcap_file.is_some() {
        let open_item = MenuItemBuilder::new("Close").id("close").build(app_handle)?;
        file = file.item(&open_item);
    } else {
        let open_item = MenuItemBuilder::new("Open").id("open").build(app_handle)?;
        file = file.item(&open_item);
    }
    let file_menu = file.build()?;
    // let file_menu = SubmenuBuilder::new(app_handle, "File").item(&open_item).build()?;

    let edit_item = MenuItemBuilder::new("Edit").build(app_handle)?;

    let menu = MenuBuilder::new(app_handle).items(&[&file_menu, &edit_item]).build()?;
    app_handle.set_menu(menu)?;
    Ok(())
}

pub fn run() {
    let (ui, mut engine) = util::core::build_engine();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            engine.run().await;
        });
    });
    let context = GUIContext::new(ui);

    use command::api::ready;

    // let cmds = tauri::generate_handler![ready, frames, open_file_dialog, open_recent_file];
    tauri::Builder::default()
        .manage(context)
        .setup(|app| {
            app.manage(RecentFiles::default());
            let args: Vec<String> = std::env::args().collect();
            let mut option = None;
            if args.len() > 1 {
                let file_path = args[1].clone();
                option = Some(file_path);
            }
            rebuild_menu(app.handle(), option)?;
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
            // id if id.starts_with("recent_") => {
            //     if let Ok(index) = id.strip_prefix("recent_").unwrap().parse::<usize>() {
            //         let recent_files: State<RecentFiles> = app.state();
            //         let files = recent_files.get_files();

            //         if let Some(file) = files.get(index) {
            //             let app_handle = app.clone();
            //             let file_path = file.path.clone();

            //             tauri::async_runtime::spawn(async move {
            //                 match open_recent_file(app_handle, file_path).await {
            //                     Ok(_) => {
            //                         println!("Opened recent file successfully");
            //                     }
            //                     Err(e) => {
            //                         eprintln!("Error opening recent file: {}", e);
            //                     }
            //                 }
            //             });
            //         }
            //     }
            // }
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
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
