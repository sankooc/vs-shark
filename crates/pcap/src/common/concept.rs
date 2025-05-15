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
    pub protocol: String,
    pub len: u32,
    pub irtt: u16,
    pub info: &'static str,
    pub status: &'static str,
}

#[derive(Default, Clone, Serialize)]
pub struct Field {
    pub start: u64,
    pub size: u64,
    pub summary: &'static str,
    pub children: Option<Vec<Field>>,
}

impl Field {
    pub fn label(summary: &'static str, start: u64, size: u64) -> Field {
        Field {
            start,
            size,
            summary,
            children: None,
        }
    }
    pub fn empty() -> Field {
        Field {
            start: 0,
            size: 0,
            summary: "",
            children: None,
        }
    }
    pub fn with_children(summary: &'static str, start: u64, size: u64) -> Field {
        Field {
            start,
            size,
            summary,
            children: Some(Vec::new()),
        }
    }
    pub fn with_children_reader(reader: &super::io::Reader) -> Field {
        Field::with_children("", reader.cursor as u64, 0)
    }
}
