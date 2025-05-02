use serde::Serialize;



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


#[derive(Serialize, Default)]
pub struct FrameInfo {
    pub index: u32,
    pub time: u64,
    pub source: &'static str,
    pub dest: &'static str,
    pub protocol: &'static str,
    pub len: u32,
    pub irtt: u16,
    pub info: &'static str,
    pub status: &'static str,
}