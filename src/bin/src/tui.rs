use crate::shutdown_handler;
use crossterm::event;
use crossterm::event::Event;
use ferrumc_state::GlobalState;
use ratatui::{DefaultTerminal, Frame};

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use ratatui::prelude::{Line, Modifier, Span};
use std::thread::sleep;
use std::time::Duration;
use tracing::debug;

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use tui_logger::{TuiWidgetEvent, TuiWidgetState};

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
    let mut input = Input::default();

    // This enables scrollback + other tui-logger interactions
    let log_state = TuiWidgetState::new();

    loop {
        terminal.draw(|frame| {
            render(frame, &input, &log_state);
        })?;

        if event::poll(Duration::from_millis(10))? {
            let ev = event::read()?;

            if let Event::Key(key_event) = ev {
                // Ctrl+C exits
                if key_event.code == event::KeyCode::Char('c')
                    && key_event.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    shutdown_handler(state.clone());
                    break;
                }

                // Log scrolling controls (PageUp/PageDown/Esc)
                match key_event.code {
                    event::KeyCode::PageUp => {
                        log_state.transition(TuiWidgetEvent::PrevPageKey);
                        continue;
                    }
                    event::KeyCode::PageDown => {
                        log_state.transition(TuiWidgetEvent::NextPageKey);
                        continue;
                    }
                    event::KeyCode::Esc => {
                        log_state.transition(TuiWidgetEvent::EscapeKey);
                        continue;
                    }
                    _ => {}
                }

                // Enter runs command
                if key_event.code == event::KeyCode::Enter {
                    let cmd = input.value().trim().to_string();
                    if !cmd.is_empty() {
                        debug!("Command ran: {}", cmd);
                    }
                    input.reset();
                    continue;
                }

                // Everything else edits the command line
                input.handle_event(&ev);
            }
        }

        sleep(Duration::from_millis(50))
    }

    ratatui::restore();
    Ok(())
}

fn render(frame: &mut Frame, input: &Input, log_state: &TuiWidgetState) {
    let area = frame.area();

    // One outer box
    let outer = Block::default()
        .title(" FerrumC (Press Ctrl+C to exit)  (PgUp/PgDn to scroll logs) ")
        .borders(Borders::ALL);

    frame.render_widget(outer.clone(), area);
    let inner = outer.inner(area);

    // Split inside box: logs + 1 command line
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);

    // Logger (no extra borders)
    let log_widget = tui_logger::TuiLoggerWidget::default()
        .formatter(Box::new(
            ferrumc_logging::tui_formatter::TuiTracingFormatter,
        ))
        .state(log_state);

    frame.render_widget(log_widget, chunks[0]);

    // Command line with dimming
    let prompt = "> ";
    let value = input.value();
    let is_empty = value.is_empty();

    let prompt_style = if is_empty {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Gray)
    };

    let value_style = if is_empty {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let shown_text = if is_empty { "type a command…" } else { value };

    // Handle horizontal scrolling for real input (not placeholder)
    let available_width = chunks[1].width.saturating_sub(prompt.len() as u16) as usize;
    let scroll = if is_empty {
        0
    } else {
        input.visual_scroll(available_width)
    };

    let line = Line::from(vec![
        Span::styled(prompt, prompt_style),
        Span::styled(shown_text, value_style),
    ]);

    let cmd_widget = Paragraph::new(line).scroll((0, scroll as u16));
    frame.render_widget(cmd_widget, chunks[1]);

    // Cursor: keep it after prompt, and track scroll
    let cursor_x = if is_empty {
        // just after "> "
        prompt.len() as u16
    } else {
        let cur = input.visual_cursor();
        (prompt.len() + cur.saturating_sub(scroll)) as u16
    };

    frame.set_cursor_position((chunks[1].x + cursor_x, chunks[1].y));
}
