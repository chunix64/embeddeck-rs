use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use ratatui::{Terminal, backend::Backend};

pub struct UIActor<B: Backend> {
    termimal: Terminal<B>,
}

impl<B> UIActor<B>
where
    B: Backend,
{
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { termimal: terminal }
    }

    pub async fn run(&mut self) {
        loop {
            Delay.delay_ms(500).await;
        }
    }
}
