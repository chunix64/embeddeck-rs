use ratatui::{Terminal, backend::Backend};

use crate::{models::clock::Clock, ui::screens::default::DefaultUI};

pub struct UIActor<'a, B: Backend> {
    terminal: Terminal<B>,
    clock: &'a Clock,
}

impl<'a, B> UIActor<'a, B>
where
    B: Backend,
{
    pub fn new(terminal: Terminal<B>, clock: &'a Clock) -> Self {
        Self { terminal, clock }
    }

    pub async fn run(&mut self) {
        self.draw().await;
    }

    async fn draw(&mut self) {
        let hour = self.clock.hour().await;
        let minute = self.clock.minute().await;
        let second = self.clock.second().await;

        self.terminal
            .draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();

                DefaultUI::draw(area, buf, hour, minute, second);
            })
            .unwrap();
    }
}
