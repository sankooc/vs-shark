use pcap::common::concept::{ConversationCriteria, Criteria, DNSRecord, DNSResponse, FrameIndex, FrameInfo, HttpCriteria, HttpMessageDetail, ListResult, TLSConversation, TLSItem, UDPConversation, VConnection, VConversation, VHttpConnection};
use serde::Serialize;
use util::core::FrameResult;
// use anyhow::Result;
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
pub async fn ready(_ctx: tauri::State<'_, GUIContext>, _name: &str) -> Result<String, String> {
    // let engine = ctx.inner();
    Ok("text".to_string())
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
    let filter = host.is_empty().then(|| None).unwrap_or(Some(HttpCriteria { hostname: Some(host) }));
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
