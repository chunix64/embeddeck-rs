use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

use crate::{models::clock::Clock, ui::widgets::digital_clock::DigitalClockWidget};

pub struct DefaultUI<'a> {
    clock: &'a Clock,
}

impl<'a> DefaultUI<'a> {
    pub fn new(clock: &'a Clock) -> Self {
        Self { clock }
    }
}

impl Widget for DefaultUI<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("ESP Clock".bold());

        let block = Block::bordered()
            .title(title)
            .border_set(ratatui::symbols::border::THICK);
        let inner = block.inner(area);
        let center = inner.centered(Constraint::Fill(1), Constraint::Length(4));
        block.render(area, buf);

        let digital_clock = DigitalClockWidget::new(self.clock);
        digital_clock.render(center, buf);
    }
}
