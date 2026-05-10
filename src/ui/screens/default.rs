use alloc::format;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct DefaultUI;

impl DefaultUI {
    pub fn draw(area: Rect, buf: &mut Buffer, hour: u32, minute: u32, second: u32) {
        let title = Line::from("ESP Clock".bold());
        let block = Block::bordered()
            .title(title)
            .border_set(ratatui::symbols::border::THICK);

        let clock_text = format!("{:02}:{:02}:{:02}", hour, minute, second);

        let clock = Paragraph::new(clock_text)
            .style(Style::default().bold())
            .centered()
            .block(block);

        clock.render(area, buf);
    }
}
