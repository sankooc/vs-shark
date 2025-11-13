// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use tauri::menu::{MenuItemBuilder, MenuBuilder, SubmenuBuilder};



// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let sub_item_1 = MenuItemBuilder::new("Open")
                .id("open")
                .build(app)?;

            let sub_item_2 = MenuItemBuilder::new("Open Recent")
                .id("open_recent")
                .build(app)?;

            let file_item = SubmenuBuilder::new(app, "File")
                .item(&sub_item_1)
                .item(&sub_item_2)
                .build()?;

            let edit_item = MenuItemBuilder::new("Edit")
                .build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&file_item, &edit_item])
                .build()?;
            app.set_menu(menu)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
