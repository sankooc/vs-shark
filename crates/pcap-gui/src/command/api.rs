use pcap::common::{concept::{ConversationCriteria, Criteria, DNSRecord, DNSResponse, FrameIndex, FrameInfo, HttpCriteria, HttpMessageDetail, ListResult, ProgressStatus, TLSConversation, TLSItem, UDPConversation, VConnection, VConversation, VHttpConnection}, file::Metadata};
use serde::Serialize;
use util::{PFile, core::FrameResult};
use crate::GUIContext;

#[derive(Serialize)]
pub struct HttpD {
    pub headers: Vec<String>,
    pub raw: Vec<u8>,
    pub plaintext: Option<String>,
    pub content_type: Option<String>,
}

impl From<&HttpMessageDetail> for HttpD {
    fn from(value: &HttpMessageDetail) -> Self {
        let headers = value.headers.clone();
        let raw = value.raw_content().to_vec();
        let plaintext = value.get_text_content();
        let content_type = value.content_type();
        Self {
            headers,
            raw,
            plaintext,
            content_type,
        }
    }
}

#[tauri::command]
pub async fn touch(ctx: tauri::State<'_, GUIContext>) -> Result<(PFile, ProgressStatus), ()> {
    let ctx = ctx.inner();
    let file = ctx.engine().touch_file().await;
    file.ok_or(())
}
#[tauri::command]
pub async fn metadata(ctx: tauri::State<'_, GUIContext>) -> Result<Metadata, String> {
    let engine = ctx.inner();
    let metadata = engine.engine().metadata().await;
    metadata.ok_or("read_metadata_failed".to_string())
}

#[tauri::command]
pub async fn frames(state: tauri::State<'_, GUIContext>, start: usize, size: usize) -> Result<ListResult<FrameInfo>, String> {
    let cri = Criteria { start, size };
    let context = state.inner();
    Ok(context.engine().frames(cri).await)
}

#[tauri::command]
pub async fn frame(state: tauri::State<'_, GUIContext>, index: usize) -> Result<FrameResult, String> {
    let context = state.inner();
    Ok(context.engine().frame(index as FrameIndex).await)
}
#[tauri::command]
pub async fn stat(state: tauri::State<'_, GUIContext>, field: String) -> Result<String, String> {
    let context = state.inner();
    Ok(context.engine().stat(field).await)
}

#[tauri::command]
pub async fn tcp_list(state: tauri::State<'_, GUIContext>, start: usize, size: usize, ip: Option<String>) -> Result<ListResult<VConversation>, String> {
    let context = state.inner();
    let cri = Criteria { start, size };
    let filter = ConversationCriteria { ip };
    Ok(context.engine().conversations(cri, filter).await)
}

#[tauri::command]
pub async fn tcp_conv_list(state: tauri::State<'_, GUIContext>, start: usize, size: usize, index: usize) -> Result<ListResult<VConnection>, String> {
    let context = state.inner();
    let cri = Criteria { start, size };
    Ok(context.engine().connections(index, cri).await)
}

#[tauri::command]
pub async fn udp_list(state: tauri::State<'_, GUIContext>, start: usize, size: usize, ip: Option<String>, asc: bool) -> Result<ListResult<UDPConversation>, String> {
    let context = state.inner();
    let cri = Criteria { start, size };
    Ok(context.engine().udp_list(cri, ip, asc).await)
}

#[tauri::command]
pub async fn http_list(state: tauri::State<'_, GUIContext>, start: usize, size: usize, host: String, asc: bool) -> Result<ListResult<VHttpConnection>, String> {
    let context = state.inner();
    let cri = Criteria { start, size };
    let filter = if host.is_empty() { None } else { Some(HttpCriteria { hostname: Some(host) }) };
    Ok(context.engine().http_list(cri, filter, asc).await)
}

#[tauri::command]
pub async fn dns_records(state: tauri::State<'_, GUIContext>, start: usize, size: usize, asc: bool) -> Result<ListResult<DNSResponse>, String> {
    let cri = Criteria { start, size };
    let context = state.inner();
    Ok(context.engine().dns_records(cri, asc).await)
}

#[tauri::command]
pub async fn dns_record(state: tauri::State<'_, GUIContext>, index: usize, start: usize, size: usize) -> Result<ListResult<DNSRecord>, String> {
    let cri = Criteria { start, size };
    let context = state.inner();
    Ok(context.engine().dns_record(index, cri).await)
}

#[tauri::command]
pub async fn tls_list(state: tauri::State<'_, GUIContext>, start: usize, size: usize) -> Result<ListResult<TLSConversation>, String> {
    let cri = Criteria { start, size };
    let context = state.inner();
    Ok(context.engine().tls_list(cri).await)
}

#[tauri::command]
pub async fn tls_conv_list(state: tauri::State<'_, GUIContext>, index: usize, start: usize, size: usize) -> Result<ListResult<TLSItem>, String> {
    let cri = Criteria { start, size };
    let context = state.inner();
    Ok(context.engine().tls_detail(index, cri).await)
}

#[tauri::command]
pub async fn http_detail(state: tauri::State<'_, GUIContext>, index: usize) -> Result<Option<Vec<HttpD>>, String> {
    let context = state.inner();
    let rs = context.engine().http_detail(index).await;
    let rs2: Option<Vec<HttpD>> = rs.map(|f| f.iter().map(|e| e.into()).collect());
    Ok(rs2)
}
