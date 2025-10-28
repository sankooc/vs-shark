use std::sync::LazyLock;

use pcap::common::concept::Language;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use syntect::{
    easy::HighlightLines,
    highlighting::{ThemeSet, Style as SyntectStyle},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};


static THEMES: LazyLock<ThemeSet> = LazyLock::new(|| {
    ThemeSet::load_defaults()
});

static SYNTAX: LazyLock<SyntaxSet> = LazyLock::new(|| {
    SyntaxSet::load_defaults_newlines()
});

pub struct CodeView {
    content: String,
    language: Language,
    scroll: u16,
}

impl CodeView {
    pub fn new(content: String, language: Language) -> Self {
        Self {
            content,
            language,
            scroll: 0,
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }
}

impl Widget for &CodeView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let syntax = match self.language {
            Language::Json => SYNTAX.find_syntax_by_extension("json").unwrap_or_else(|| SYNTAX.find_syntax_plain_text()),
            Language::JavaScript => SYNTAX.find_syntax_by_extension("js").unwrap_or_else(|| SYNTAX.find_syntax_plain_text()),
            Language::Css => SYNTAX.find_syntax_by_extension("css").unwrap_or_else(|| SYNTAX.find_syntax_plain_text()),
            Language::Html => SYNTAX.find_syntax_by_extension("html").unwrap_or_else(|| SYNTAX.find_syntax_plain_text()),
            Language::Xml => SYNTAX.find_syntax_by_extension("xml").unwrap_or_else(|| SYNTAX.find_syntax_plain_text()),
            Language::Yaml => SYNTAX.find_syntax_by_extension("yaml").or_else(|| SYNTAX.find_syntax_by_extension("yml")).unwrap_or_else(|| SYNTAX.find_syntax_plain_text()),
            Language::Csv | Language::Text => SYNTAX.find_syntax_plain_text(),
            _ => {return}
        };
        let theme = &THEMES.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);

        let mut lines = Vec::new();
        for line in LinesWithEndings::from(&self.content) {
            let ranges: Vec<(SyntectStyle, &str)> = h.highlight_line(line, &SYNTAX).unwrap();
            let spans: Vec<Span> = ranges
                .into_iter()
                .map(|(style, content)| {
                    let color = style.foreground;
                    Span::styled(
                        content.to_string(),
                        Style::default().fg(Color::Rgb(color.r, color.g, color.b)),
                    )
                })
                .collect();
            lines.push(Line::from(spans));
        }

        let paragraph = Paragraph::new(lines).scroll((self.scroll, 0));

        paragraph.render(area, buf);
    }
}
