use std::time::Instant;

use crossterm::event::{self, Event};
use ratatui::{
    Frame,
    style::{Style, Stylize},
};
use tui_big_text::{BigText, PixelSize};

fn main() {
    const INITIAL_REMAINING_SECONDS: u64 = 300;
    // Get the time when the program starts
    let start_time = Instant::now();

    let mut terminal = ratatui::init();
    loop {
        // Check how long the timer has been running
        let elapsed_seconds = start_time.elapsed().as_secs();
        let remaining_seconds = INITIAL_REMAINING_SECONDS - elapsed_seconds;

        terminal
            .draw(|frame| draw(frame, remaining_seconds))
            .expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame, remaining_seconds: u64) {
    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .centered()
        .lines(vec![remaining_seconds.to_string().red().into()])
        .build();
    frame.render_widget(big_text, frame.area());
}
