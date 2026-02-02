use crate::shutdown_handler;
use crossterm::event;
use crossterm::event::Event;
use ferrumc_state::GlobalState;
use ratatui::{DefaultTerminal, Frame};
use std::thread::sleep;
use std::time::Duration;

pub fn run_tui(state: GlobalState) {
    std::thread::Builder::new()
        .name("tui".into())
        .spawn(|| {
            ratatui::run(|term| {
                tui_main(term, state).expect("TUI encountered an unrecoverable error")
            });
        })
        .unwrap();
}

fn tui_main(terminal: &mut DefaultTerminal, state: GlobalState) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            render(frame);
        })?;
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == event::KeyCode::Char('c')
                    && key_event.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    shutdown_handler(state.clone());
                    break;
                }
            }
        }
        sleep(Duration::from_millis(50))
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    let area = frame.area();
    frame.render_widget(tui_logger::TuiLoggerSmartWidget::default(), area)
}
