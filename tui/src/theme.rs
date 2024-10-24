use ratatui::style::{palette::tailwind, Color, Style};
use shark::common::concept::FrameInfo;
//c200: Color::from_u32(0xe2e8f0),
// $c-fg-0: #fbf1c7;

// $c-bg-h-0: #1d2021;
// $c-bg-0: #282828;
// $c-bg-1: #3c3836;

// $c-blue: #458588;
// $c-green: #98971a;
// $c-red: #cc241d;
// $c-aqua: #689d6a;
// $c-purple: #b16286;
// $c-yellow: #d79921;
pub const GRUVBOX_FG_0 :Color = Color::from_u32(0xfbf1c7);
pub const GRUVBOX_BG_H_0 :Color = Color::from_u32(0x1d2021);
pub const GRUVBOX_BG_0 :Color = Color::from_u32(0x282828);
pub const GRUVBOX_BG_1 :Color = Color::from_u32(0x3c3836);

pub const GRUVBOX_D_BLUE :Color = Color::from_u32(0x458588);
pub const GRUVBOX_D_GREEN :Color = Color::from_u32(0x98971a);
pub const GRUVBOX_D_RED :Color = Color::from_u32(0xcc241d);
pub const GRUVBOX_D_AQUA :Color = Color::from_u32(0x689d6a);
pub const GRUVBOX_D_PURPLE :Color = Color::from_u32(0xb16286);
pub const GRUVBOX_D_YELLOW :Color = Color::from_u32(0xd79921);


pub const SELECT_FG: Color = Color::from_u32(0x001219);
pub const SELECT_BG: Color = Color::from_u32(0x94d2bd);

pub const SSDP_C: Color = Color::from_u32(0x739588);


pub fn get_protocol_color(protocol: &str) -> Style {
    match protocol.to_lowercase().as_str() {
        "tcp" => Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_BLUE),
        "arp" => Style::new().fg(GRUVBOX_BG_H_0).bg(GRUVBOX_D_GREEN),
        "tls" => Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_AQUA),
        "udp" => Style::new().fg(GRUVBOX_BG_H_0).bg(GRUVBOX_D_YELLOW),
        "ssdp" => Style::new().fg(GRUVBOX_FG_0).bg(SSDP_C),
        _ => Style::new().fg(tailwind::SLATE.c200).bg(tailwind::SLATE.c950)
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
pub fn get_color(class_name: &'static str) -> Style {
  match class_name {
    _ => Style::new().fg(GRUVBOX_FG_0).bg(GRUVBOX_D_BLUE)
  }  
}
pub fn panel_color() -> Style {
    Style::new().fg(GRUVBOX_BG_H_0)
}