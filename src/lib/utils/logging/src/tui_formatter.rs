use log::Level;
use ratatui_core::style::Stylize;
use ratatui_core::text::Line;
use tui_logger::{ExtLogRecord, LogFormatter};

pub struct TuiTracingFormatter;

impl LogFormatter for TuiTracingFormatter {
    fn min_width(&self) -> u16 {
        4
    }

    fn format(&self, width: usize, evt: &ExtLogRecord) -> Vec<Line<'_>> {
        let level = match evt.level {
            Level::Error => "ERROR ".red().bold(),
            Level::Warn => "WARN  ".yellow().bold(),
            Level::Info => "INFO  ".green().bold(),
            Level::Debug => "DEBUG ".blue().bold(),
            Level::Trace => "TRACE ".magenta().bold(),
        };

        let timestamp = evt.timestamp.format("%Y-%m-%d %H:%M:%S ").to_string().dim();
        let file = if let Some(file) = evt.file() {
            format!(" {}:{}", file, evt.line.unwrap_or(0)).dim()
        } else {
            "".into()
        };
        let breaker1 = " | ".to_string().into();
        let breaker2 = " | ".to_string().into();
        let message = evt.msg().to_string().into();
        vec![Line::from(vec![
            level, breaker1, timestamp, breaker2, file, message,
        ])]
    }
}
