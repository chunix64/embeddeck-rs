use alloc::{format, vec};

use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget};
use tui_big_text::BigText;

use crate::models::clock::Clock;

pub struct DigitalClockWidget<'a> {
    clock: &'a Clock,
}

impl<'a> DigitalClockWidget<'a> {
    pub fn new(clock: &'a Clock) -> Self {
        Self { clock }
    }
}

impl Widget for DigitalClockWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let clock_text = Line::from(format!(
            "{:02}:{:02}:{:02}",
            self.clock.hour(),
            self.clock.minute(),
            self.clock.second()
        ));

        let clock = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Quadrant)
            .style(Style::new().white())
            .lines(vec![clock_text])
            .centered()
            .build();

        clock.render(area, buf);
    }
}
