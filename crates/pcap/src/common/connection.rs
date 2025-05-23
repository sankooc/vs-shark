// use crate::cache::intern;

use std::{
    fmt::{Display, Write},
    ops::Range,
};

use crate::protocol;

use super::{
    enum_def::{Protocol, SegmentStatus, TCPConnectStatus, TCPDetail, TCPFLAG},
    io::{DataSource, Reader}
};

#[derive(Debug)]
pub struct TcpFlagField {
    data: u16,
}

impl TcpFlagField {
    pub fn head_len(&self) -> u16 {
        (self.data >> 12) & 0x0f
    }
    fn f_list(&self) -> Vec<TCPFLAG> {
        let mut list = vec![];
        if self.contain(TCPFLAG::FIN) {
            list.push(TCPFLAG::FIN);
        }
        if self.contain(TCPFLAG::SYNC) {
            list.push(TCPFLAG::SYNC);
        }
        if self.contain(TCPFLAG::RESET) {
            list.push(TCPFLAG::RESET);
        }
        if self.contain(TCPFLAG::PUSH) {
            list.push(TCPFLAG::PUSH);
        }
        if self.contain(TCPFLAG::ACK) {
            list.push(TCPFLAG::ACK);
        }
        list
    }
    pub fn list_str(&self) -> String {
        let list = self.f_list();
        if list.len() == 0 {
            return String::from("");
        }
        // let content = list.iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(",");
        let mut content = String::with_capacity(list.len() * 10);
        let mut iter = list.iter();
        if let Some(first) = iter.next() {
            write!(&mut content, "{}", first).unwrap();
            for item in iter {
                write!(&mut content, ",{}", item).unwrap();
            }
        }
        return format!("[{}]", content);
    }
}

impl Display for TcpFlagField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.f_list();
        // let content = list.iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(",");
        let mut content = String::with_capacity(list.len() * 10);
        let mut iter = list.iter();
        if let Some(first) = iter.next() {
            write!(&mut content, "{}", first).unwrap();
            for item in iter {
                write!(&mut content, ",{}", item).unwrap();
            }
        }
        f.write_fmt(format_args!("Flags: {:#06x} ({})", (self.data & 0x1fff), content))
    }
}

impl From<u16> for TcpFlagField {
    fn from(data: u16) -> Self {
        Self { data }
    }
}
impl TcpFlagField {
    pub fn new(data: u16) -> Self {
        Self { data }
    }
    pub fn contain(&self, flag: TCPFLAG) -> bool {
        // let oodd = 16 - (flag as i32);
        let mask = 1 << (flag as i32) as u16;
        let val = self.data & 0x1fff;
        (val & mask) > 0
    }
    pub fn extact_match(&self, flag: TCPFLAG) -> bool {
        let mask = 1 << (flag as i32) as u16;
        let val = self.data & 0x1fff;
        (val & mask) == mask
    }
}

#[derive(Default)]
pub struct TCPStatistic {
    pub count: u16,
    pub throughput: u32,
    pub retransmission: u16,
    pub invalid: u16,
}

pub struct ConnectState {
    pub seq: u32,
    pub ack: u32,
    pub next: u32,
    pub len: u16,
    pub status: TCPDetail,
    pub flag_bit: u16,
    pub connect_finished: bool,
    pub next_protocol: Protocol,
    pub connection: Option<(usize, bool)>,
}

impl ConnectState {
    pub fn new(seq: u32, ack: u32, next: u32, len: u16, status: TCPDetail) -> Self {
        Self {
            seq,
            ack,
            next,
            len,
            status,
            flag_bit: 0,
            connect_finished: false,
            next_protocol: Protocol::None,
            connection: None,
        }
    }
}

pub struct TCPSegment {
    pub index: u32,
    pub range: Range<usize>,
}

impl TCPSegment {
    pub fn new(index: u32, range: Range<usize>) -> Self {
        Self { index, range }
    }
}

#[derive(Default)]
pub struct Endpoint {
    host: String,
    port: u16,
    pub status: TCPConnectStatus,
    seq: u32,
    _seq: u32,
    next: u32,
    _ack: u32,
    ack: u32,
    _checksum: u16,
    statistic: TCPStatistic,
    pub segment_status: SegmentStatus,
    _segments: Option<Vec<TCPSegment>>,
}
impl Endpoint {
    pub fn new(host: String, port: u16) -> Self {
        let mut _self = Self::default();
        _self.host = host;
        _self.port = port;
        _self
    }
    pub fn clear_segment(&mut self) {
        self._segments = None;
    }
    pub fn add_segment(&mut self, index: u32, range: Range<usize>) {
        if let None = self._segments {
            self._segments = Some(vec![]);
        }
        let _segs = self._segments.as_mut().unwrap();
        _segs.push(TCPSegment::new(index, range));
    }
    pub fn confirm(&mut self, stat: &TCPStat) {
        let acknowledge = stat.ack;
        if self._ack == 0 {
            if stat.state.contain(TCPFLAG::SYNC) {
                self._ack = acknowledge;
            } else {
                if acknowledge >= 1 {
                    self._ack = acknowledge - 1;
                } else {
                    self._ack = 0;
                }
            }
        }

        if self.ack > acknowledge {
            // return (false, false);
            // TODO false
        }
        if self.seq < acknowledge {
            // TODO
        }
        // let same = acknowledge == self.ack;
        self.ack = acknowledge;
    }
    pub fn update(&mut self, stat: &TCPStat) -> TCPDetail {
        let sequence = stat.sequence;
        let statistic = &mut self.statistic;
        statistic.count = statistic.count + 1;
        statistic.throughput += stat.payload_len as u32;
        if self.seq == sequence && stat.payload_len == 0 {
            return TCPDetail::NEXT;
        }
        if stat.state.contain(TCPFLAG::RESET) {
            self.clear_segment();
            self.status = TCPConnectStatus::CLOSED;
            return TCPDetail::RESET;
        }
        let mut _tcp_len = 0;
        if stat.state.contain(TCPFLAG::SYNC) {
            _tcp_len = 1;
        } else if stat.state.contain(TCPFLAG::FIN) {
            _tcp_len = 1;
        } else {
            _tcp_len = stat.payload_len as u32;
        }
        if self.seq == 0 {
            if stat.state.contain(TCPFLAG::SYNC) {
                self._seq = sequence;
                // self.status = TCPConnectStatus::SYN_SENT;
            } else {
                self._seq = sequence - 1;
            }
            // self._seq = sequence;
            self.seq = sequence;
            self.next = sequence + _tcp_len;
            self._checksum = stat.crc;
            return TCPDetail::NEXT;
        }
        if sequence > self.next {
            self.seq = sequence;
            self.next = sequence + _tcp_len;
            self._checksum = stat.crc;
            statistic.invalid += 1;
            self.clear_segment();
            return TCPDetail::NOPREVCAPTURE;
        } else if sequence == self.next {
            self.seq = sequence;
            self._checksum = stat.crc;
            if _tcp_len == 0 {
                return TCPDetail::NEXT;
            }
            self.next = sequence + _tcp_len;
            return TCPDetail::NEXT;
        } else {
            if sequence == self.next - 1 && (_tcp_len == 1 || _tcp_len == 0) && stat.state.extact_match(TCPFLAG::ACK) {
                self._checksum = stat.crc;
                return TCPDetail::KEEPALIVE;
            }
            if self.seq == sequence + _tcp_len {
                statistic.retransmission += 1;
                return TCPDetail::RETRANSMISSION;
            }
            statistic.invalid += 1;
            return TCPDetail::DUMP;
        }
        // todo
    }

    pub fn ack(&self) -> u32 {
        if self._ack >= self.ack {
            return 0;
        }
        self.ack - self._ack
    }
    pub fn seq(&self) -> u32 {
        if self._seq >= self.seq {
            return 0;
        }
        self.seq - self._seq
    }
    pub fn next(&self) -> u32 {
        if self.next > self._seq {
            return self.next - self._seq;
        }
        0
    }
    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub struct Connection {
    primary: Endpoint,
    second: Endpoint,
    pub protocol: Protocol,
}
impl Connection {
    pub fn new(primary: Endpoint, second: Endpoint) -> Self {
        Self { primary, second, protocol: Protocol::None }
    }
}

pub struct TmpConnection<'a> {
    conn: &'a mut Connection,
    // pub frame: &'a mut Frame, //todo
    reverse: bool,
}

impl<'a> TmpConnection<'a> {
    pub fn new(conn: &'a mut Connection, reverse: bool) -> Self {
        Self { conn, reverse }
    }

    pub fn source_endpoint(&mut self) -> &mut Endpoint{
        match self.reverse {
            true => &mut self.conn.primary,
            false => &mut self.conn.second,
        }
    }
    pub fn target_endpoint(&mut self) -> &mut Endpoint{
        match self.reverse {
            true => &mut self.conn.second,
            false => &mut self.conn.primary,
        }
    }
    pub fn update(&mut self, stat: &TCPStat, data_source: &DataSource, range: Range<usize>) -> anyhow::Result<ConnectState> {
        let mut _rs = TCPDetail::KEEPALIVE;
        let mut main = &mut self.conn.second;
        let mut rev = &mut self.conn.primary;
        if self.reverse {
            main = &mut self.conn.primary;
            rev = &mut self.conn.second;
        }

        let status: TCPDetail = main.update(&stat);
        rev.confirm(&stat);
        let mut rs = ConnectState::new(main.seq(), rev.ack(), main.next(), stat.payload_len, status);
        match &rs.status {
            TCPDetail::RESET => {
                main.status = TCPConnectStatus::CLOSED;
                rev.status = TCPConnectStatus::CLOSED;
                rs.connect_finished = true;
            }
            TCPDetail::RETRANSMISSION | TCPDetail::NOPREVCAPTURE | TCPDetail::DUMP => {
                // TODO
            }
            _ => {
                if rs.status == TCPDetail::NEXT {
                    if rs.len > 0 {
                        let reader = Reader::new_sub(data_source, range.clone())?;
                        match self.conn.protocol {
                            Protocol::None => {
                                if protocol::application::http::detect(&reader) {
                                    self.conn.protocol = Protocol::HTTP;
                                    main.segment_status = SegmentStatus::Init;
                                }
                            }
                            _ => {}
                        }
                        rs.next_protocol = self.conn.protocol;
                    }
                }
                // // process
                if stat.state.contain(TCPFLAG::FIN) {
                    main.status = TCPConnectStatus::CLOSE_WAIT;
                }
                if stat.state.extact_match(TCPFLAG::ACK) && rev.status == TCPConnectStatus::CLOSE_WAIT {
                    rev.status = TCPConnectStatus::CLOSED;
                    if main.status == TCPConnectStatus::CLOSED {
                        rs.connect_finished = true;
                    }
                }
            }
        }
        Ok(rs)
    }
}

pub struct TCPStat {
    pub index: u32,
    sequence: u32,
    ack: u32,
    crc: u16,
    state: TcpFlagField,
    // window: u16,
    // urgent: u16,
    payload_len: u16,
    // pub data_range: Range<usize>,
}

impl TCPStat {
    pub fn new(index: u32, sequence: u32, ack: u32, crc: u16, state: TcpFlagField, payload_len: u16) -> Self {
        Self {
            index,
            sequence,
            ack,
            crc,
            state,
            payload_len,
            // data_range,
        }
    }
}
