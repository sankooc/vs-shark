// use crate::cache::intern;


use std::fmt::Display;

use super::{enum_def::{TCPDetail, TCPFLAG}};


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
        let content = list.iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(",");
        return format!("[{}]", content);
    }
}

impl Display for TcpFlagField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.f_list();
        let content = list.iter().map(|f| format!("{}", f)).collect::<Vec<_>>().join(",");
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
}

impl ConnectState {
    pub fn new(seq: u32, ack: u32, next: u32, len: u16, status: TCPDetail) -> Self {
        Self {seq, ack, next, len, status}
    }
}

#[derive(Default)]
pub struct Endpoint {
    host: &'static str,
    port: u16,
    seq: u32,
    _seq: u32,
    pub next: u32,
    _ack: u32,
    ack: u32,
    _checksum: u16,
    statistic: TCPStatistic,
}
impl Endpoint {
    pub fn new(host: &'static str, port: u16) -> Self {
        let mut _self = Self::default();
        _self.host = host;
        _self.port = port;
        _self
    }
    pub fn confirm(&mut self, stat: &TCPStat) {
        let acknowledge = stat.ack;
        if self._ack == 0 {
            if stat.state.contain(TCPFLAG::SYNC) {
                self._ack = acknowledge;
            } else {
                self._ack = acknowledge - 1;
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
            // self.clear_segment();
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
            // self.clear_segment();
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
            return self.next - self._seq
        }
        0
    }
}

pub struct Connection {
    primary: Endpoint,
    second: Endpoint,
}
impl Connection {
    pub fn new(primary: Endpoint, second: Endpoint) -> Self {
        Self { primary, second }
    }
}

pub struct TmpConnection<'a> {
    conn: &'a mut Connection,
    reverse: bool,
}

impl<'a> TmpConnection<'a> {
    pub fn new(conn: &'a mut Connection, reverse: bool) -> Self {
        Self { conn, reverse }
    }
    pub fn update(&mut self, stat: &TCPStat) -> ConnectState {
        let mut _rs = TCPDetail::KEEPALIVE;
        let mut main = &mut self.conn.second;
        let mut rev = &mut self.conn.primary;
        if self.reverse {
            main = &mut self.conn.primary;
            rev = &mut self.conn.second;
            // _rs = self.conn.primary.update(&stat);
        } 
        let rs: TCPDetail = main.update(&stat);
        rev.confirm(&stat);
        ConnectState::new(main.seq(), rev.ack(), main.next(), stat.payload_len, rs)
        // else {
        //     _rs = self.conn.second.update(&stat);
        // }
        
        // if self.reverse {
        //     self.conn.second.confirm(&stat);
        // } else {
        //     self.conn.primary.confirm(&stat);
        // }
        // if self.reverse {
        //     ConnectState::new(self.conn.primary.seq(), self.conn.primary.ack(), self.conn.primary.next(), _rs)
        // } else {
        //     ConnectState::new(self.conn.second.seq(), self.conn.second.ack(), self.conn.second.next(), _rs)
        // }
        // ConnectState::new(seq, ack, next, status)
        // _rs
    }
}

pub struct TCPStat {
    sequence: u32,
    ack: u32,
    crc: u16,
    state: TcpFlagField,
    // window: u16,
    // urgent: u16,
    payload_len: u16,
}

impl TCPStat {
    pub fn new(sequence: u32, ack: u32, crc: u16, state: TcpFlagField, payload_len: u16) -> Self {
        Self {
            sequence,
            ack,
            crc,
            state,
            payload_len,
        }
    }
}
