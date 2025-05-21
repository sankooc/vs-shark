use serde::Serialize;

use super::enum_def::PacketStatus;

pub struct Criteria {
    // pub criteria: String,
    pub size: usize,
    pub start: usize,
}

#[derive(Serialize)]
pub struct ProgressStatus {
    pub total: usize,
    pub cursor: usize,
    pub count: usize,
}

impl ProgressStatus {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize)]
pub struct ListResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub start: usize,
}

impl<T> ListResult<T> {
    pub fn new(start: usize, total: usize, items: Vec<T>) -> Self {
        Self { start, total, items }
    }
}

#[derive(Default)]
pub struct FrameInternInfo {
    pub index: u32,
    pub time: u64,
    pub len: u32,
    pub irtt: u16,
    pub status: PacketStatus,
}

#[derive(Serialize, Default, Clone)]
pub struct FrameInfo {
    pub index: u32,
    pub time: u64,
    pub source: String,
    pub dest: String,
    pub protocol: String,
    pub len: u32,
    pub irtt: u16,
    pub info: String,
    pub status: PacketStatus,
}

impl From<&FrameInternInfo> for FrameInfo {
    fn from(value: &FrameInternInfo) -> Self {
        let mut info = FrameInfo::default();
        info.index = value.index;
        info.time = value.time;
        info.len = value.len;
        info.irtt = value.irtt;
        info.status = value.status;
        info
    }
}


#[derive(Default, Clone, Serialize)]
pub struct Field {
    pub start: u64,
    pub size: u64,
    pub summary: String,
    pub children: Option<Vec<Field>>,
}

impl Field {
    pub fn label(summary: String, start: u64, end: u64) -> Field {
        Field {
            start,
            size: end - start,
            summary,
            children: None,
        }
    }
    pub fn with_children(summary: String, start: u64, size: u64) -> Field {
        Field {
            start,
            size,
            summary,
            children: Some(Vec::new()),
        }
    }
    pub fn with_children_reader(reader: &super::io::Reader) -> Field {
        Field::with_children(String::from(""), reader.cursor as u64, 0)
    }
}
