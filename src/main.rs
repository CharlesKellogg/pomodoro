use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use std::time::Instant;
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
        if handle_events().unwrap() {
            break;
        }
    }
    ratatui::restore();
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn draw(frame: &mut Frame, remaining_seconds: u64) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 3]);
    let [left_area, center_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Center"), center_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);

    let big_text = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .centered()
        .lines(vec![remaining_seconds.to_string().red().into()])
        .build();
    // frame.render_widget(big_text, left_area);

    let areas = Layout::vertical([Constraint::Length(1); 4]).split(frame.area());

    let line = Line::from(vec![
        Span::raw("Hello "),
        Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        "!".red().on_light_yellow().italic(),
    ]);
    // frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
    // frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = Paragraph::new("Hello World!").style(Style::new().red().on_white());
    // frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
    // frame.render_widget(paragraph, areas[3]);
}
