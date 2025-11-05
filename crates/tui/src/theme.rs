use pcap::common::concept::FrameInfo;
use ratatui::style::{Color, Style};
// use shark::common::concept::FrameInfo;
// https://github.com/morhetz/gruvbox
pub const GRUVBOX_FG :Color = Color::from_u32(0xebdbb2);
pub const GRUVBOX_FG_0 :Color = Color::from_u32(0xfbf1c7);
pub const GRUVBOX_FG_1 :Color = Color::from_u32(0xebdbb2);
pub const GRUVBOX_FG_2 :Color = Color::from_u32(0xd5c4a1);
pub const GRUVBOX_FG_3 :Color = Color::from_u32(0xbdae93);
pub const GRUVBOX_FG_4 :Color = Color::from_u32(0xa89984);
pub const GRUVBOX_BG_H_0 :Color = Color::from_u32(0x1d2021);
pub const GRUVBOX_BG_S_0 :Color = Color::from_u32(0x32302F);
pub const GRUVBOX_BG_0 :Color = Color::from_u32(0x282828);
pub const GRUVBOX_BG_1 :Color = Color::from_u32(0x3c3836);
pub const GRUVBOX_BG_2 :Color = Color::from_u32(0x504945);
pub const GRUVBOX_BG_3 :Color = Color::from_u32(0x665c54);
pub const GRUVBOX_BG_4 :Color = Color::from_u32(0x7c6f64);


pub const GRUVBOX_D_BLUE :Color = Color::from_u32(0x458588);
pub const GRUVBOX_D_GREEN :Color = Color::from_u32(0x98971a);
pub const GRUVBOX_D_RED :Color = Color::from_u32(0xcc241d);
pub const GRUVBOX_D_AQUA :Color = Color::from_u32(0x689d6a);
pub const GRUVBOX_D_PURPLE :Color = Color::from_u32(0xb16286);
pub const GRUVBOX_D_YELLOW :Color = Color::from_u32(0xd79921);
pub const GRUVBOX_D_GRAY :Color = Color::from_u32(0xa88884);
pub const GRUVBOX_D_ORANGE :Color = Color::from_u32(0xD65D0E);

pub const SELECT_FG: Color = Color::from_u32(0x001219);
pub const SELECT_BG: Color = Color::from_u32(0x94d2bd);
pub const SSDP_C: Color = Color::from_u32(0x739588);
pub const DEF_BG: Color = Color::from_u32(0x1d2021);
pub const DEF_FG: Color = Color::from_u32(0xebdbb2);
pub const ERROR_BG: Color = Color::from_u32(0xC96868);
pub const ERROR_FG: Color = Color::from_u32(0xF5F5F5);
pub const HTTP_BG: Color = GRUVBOX_D_GREEN;
pub const HTTP_FG: Color = GRUVBOX_FG_0;
pub const DEACTIVE_BG: Color = Color::from_u32(0x8d99ae);
pub const DEACTIVE_FG: Color = Color::from_u32(0x001219);
pub const DNS_BG: Color = Color::from_u32(0xC0D6E8);
pub const DNS_FG: Color = Color::from_u32(0xA34343);
pub const ICMP_BG: Color = Color::from_u32(0xb16286);
pub const ICMP_FG: Color = Color::from_u32(0xF5F5F5);
pub const ICMPV6_BG: Color = Color::from_u32(0xb06086);
pub const ICMPV6_FG: Color = Color::from_u32(0xF1F1F5);

pub const PPPOES_BG: Color = Color::from_u32(0x779988);
pub const PPPOES_FG: Color = GRUVBOX_FG_1;

pub const PPPOED_BG: Color = Color::from_u32(0x669988);
pub const PPPOED_FG: Color = GRUVBOX_FG_1;

pub fn get_protocol_color(protocol: &str) -> Style {
    match protocol.to_lowercase().as_str() {
        "tcp" => Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_BLUE),
        "arp" => Style::new().fg(GRUVBOX_BG_H_0).bg(Color::from_u32(0xb8bb26)),
        "tls" => Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_AQUA),
        "udp" => Style::new().fg(GRUVBOX_BG_H_0).bg(GRUVBOX_D_YELLOW),
        "ssdp" => Style::new().fg(GRUVBOX_FG_0).bg(SSDP_C),
        "error" => Style::new().fg(ERROR_FG).bg(ERROR_BG),
        "http" => Style::new().fg(HTTP_FG).bg(HTTP_BG),
        "deactive" => Style::new().fg(DEACTIVE_FG).bg(DEACTIVE_BG),
        "dns" => Style::new().fg(DNS_FG).bg(DNS_BG),
        "icmp" => Style::new().fg(ICMP_FG).bg(ICMP_BG),
        "icmpv6" => Style::new().fg(ICMPV6_FG).bg(ICMPV6_BG),
        "pppoes" => Style::new().fg(PPPOES_FG).bg(PPPOES_BG),
        "pppoed" => Style::new().fg(PPPOED_FG).bg(PPPOED_BG),
        _ => Style::new().fg(DEF_FG).bg(DEF_BG)
    }
}

pub fn reverse_protocol(protocol: &str) -> Style {
    let mut s = get_protocol_color(protocol);
    let b = s.fg.take().unwrap();
    let f = s.bg.take().unwrap();
    Style::new().fg(f).bg(b)
}


pub fn get_frame_color(frame: &FrameInfo) -> Style {
    get_protocol_color(frame.protocol.as_str())
}


pub const HEAD_FG: Color = Color::from_u32(0xebdbb2);
pub const HEAD_BG: Color = Color::from_u32(0x3c3836);
pub fn get_header_style() -> Style {
    Style::new().fg(HEAD_FG).bg(HEAD_BG)
}

pub fn get_select() -> Style {
    Style::default().fg(SELECT_FG).bg(SELECT_BG)

}

pub const ACTIVE_TAB_COLOR: Color = GRUVBOX_D_AQUA;

pub fn get_active_tab_color() -> Style {
    Style::new().fg(HEAD_FG).bg(ACTIVE_TAB_COLOR)
}
pub fn get_color(_class_name: &'static str) -> Style {
//   match class_name {
//     _ => Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_BLUE)
//   } 
Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_BLUE) 
}
pub fn panel_color() -> Style {
    Style::new().fg(GRUVBOX_FG)
}
pub fn title_color() -> Style {
    Style::new().bg(GRUVBOX_BG_0).fg(GRUVBOX_FG)
}

pub const BACK_COLOR: Color = GRUVBOX_BG_0;
pub const FRONT_COLOR: Color = GRUVBOX_FG_0;


pub const POSITIVE_STYLE: Style = Style::new().fg(HEAD_FG).bg(ACTIVE_TAB_COLOR);
pub const DESELECT_BG: Color =  GRUVBOX_BG_4;
pub const NAGETIVE_STYLE: Style = Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_BLUE);

pub const BLANK: Style = Style::new().fg(FRONT_COLOR).bg(GRUVBOX_BG_H_0);
pub const BLANK_FROZEN: Style = Style::new().fg(FRONT_COLOR).bg(GRUVBOX_FG_4);


pub const REVERT_STYLE: Style = Style::new().fg(ACTIVE_TAB_COLOR).bg(GRUVBOX_BG_H_0);


pub const REVERT_STYLE2: Style = Style::new().fg(ACTIVE_TAB_COLOR).bg(GRUVBOX_BG_S_0);

pub const STATUS_HINT_STYLE: Style = Style::new().fg(GRUVBOX_FG_4).bg(GRUVBOX_BG_3);

pub const STATUS_PROGS_STYLE: Style = Style::new().fg(GRUVBOX_FG).bg(HEAD_FG);