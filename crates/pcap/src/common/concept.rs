use serde::Serialize;

use super::NString;




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
    pub source: NString,
    pub dest: NString,
    pub protocol: String,
    pub len: u32,
    pub irtt: u16,
    pub info: NString,
    pub status: NString,
}

pub enum TCPFLAG {
    FIN = 0,
    SYN,
    RESET,
    PUSH,
    ACKNOWLEDGMENT,
    URGENT,
    ECN,
    CWR,
    AccurateEcn,
    REVERVED,
}
pub struct TcpFlagField{
    data: u16
}

impl TcpFlagField {
    pub fn new(data: u16) -> Self {
        Self { data }
    }
}


#[derive(Default, Clone, Serialize)]
pub struct Field {
    pub start: u64,
    pub size: u64,
    pub summary: NString,
    pub children: Option<Vec<Field>>,
}

impl Field {
    pub fn label(summary: NString, start: u64, end: u64) -> Field {
        Field {
            start,
            size: end - start,
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
    pub fn with_children(summary: NString, start: u64, size: u64) -> Field {
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



pub struct Endpoint {
    host: String,
    port: u16
}
impl Endpoint {
    pub fn update(&self, stat: TCPStat) {
        // todo
    }
}

pub struct Connection {
    primary:  Endpoint,
    second:  Endpoint,
}

pub struct TmpConnection{
    conn: Connection,
    reverse: bool,
}

impl TmpConnection {
    pub fn update(&self, stat: TCPStat) {
        if self.reverse {
            self.conn.primary.update(stat);
        } else {
            self.conn.second.update(stat);
        }
    }
}

pub struct TCPStat{
    sequence: u32,
    ack: u32,
    state: TcpFlagField,
    window: u16,
    urgent: u16
}