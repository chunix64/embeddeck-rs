use core::marker::PhantomData;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use ratatui::{Frame, Terminal, prelude::Backend};

pub struct DefaultUI<B: Backend> {
    _marker: PhantomData<B>,
}

impl<B: Backend> DefaultUI<B> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub async fn run(&mut self, terminal: &mut Terminal<B>, delay: &mut Delay)
    where
        B::Error: 'static,
    {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            delay.delay_ms(33).await;
        }
    }

    fn draw(&mut self, frame: &mut Frame) {}
}
