use crate::GUIContext;

#[tauri::command]
pub async fn ready(ctx: tauri::State<'_, GUIContext>, name: &str) -> Result<String, String> {
    let engine = ctx.inner();
    Ok("text".to_string())
}