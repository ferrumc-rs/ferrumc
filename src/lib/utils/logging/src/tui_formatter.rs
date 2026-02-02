use log::Level;
use ratatui::prelude::{Line, Stylize};
use tui_logger::{ExtLogRecord, LogFormatter};

use ratatui::style::Style;
use ratatui::text::Span;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
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

        let timestamp = evt.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().dim();
        let file = if let Some(file) = evt.file() {
            format!("{}:{}", file, evt.line.unwrap_or(0)).dim()
        } else {
            "".into()
        };

        let breaker1: Span<'_> = " | ".to_string().into();
        let breaker2: Span<'_> = " | ".to_string().into();
        let breaker3: Span<'_> = " | ".to_string().into();

        let message: Span<'_> = evt.msg().to_string().into();

        let spans = vec![
            level, breaker1, timestamp, breaker2, file, breaker3, message,
        ];

        wrap_spans(spans, width.saturating_sub(1))
    }
}

fn wrap_spans<'a>(spans: Vec<Span<'a>>, width: usize) -> Vec<Line<'a>> {
    let width = width.max(1);

    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut cur_spans: Vec<Span<'a>> = Vec::new();
    let mut cur_w: usize = 0;

    for sp in spans {
        let style: Style = sp.style;
        let s = sp.content.as_ref();

        // Fast path: whole span fits
        let span_w = UnicodeWidthStr::width(s);
        if cur_w + span_w <= width {
            cur_spans.push(sp);
            cur_w += span_w;
            continue;
        }

        // Slow path: split span across lines
        let mut buf = String::new();
        let mut buf_w: usize = 0;

        for ch in s.chars() {
            if ch == '\n' {
                // flush buffer to current line
                if !buf.is_empty() {
                    cur_spans.push(Span::styled(std::mem::take(&mut buf), style));
                    buf_w = 0;
                }
                // end current line
                lines.push(Line::from(std::mem::take(&mut cur_spans)));
                cur_w = 0;
                continue;
            }

            let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);

            // If this char would overflow current line, flush and start a new line
            if cur_w + buf_w + ch_w > width {
                if !buf.is_empty() {
                    cur_spans.push(Span::styled(std::mem::take(&mut buf), style));
                }
                lines.push(Line::from(std::mem::take(&mut cur_spans)));
                cur_w = 0;
                buf_w = 0;
            }

            buf.push(ch);
            buf_w += ch_w;
        }

        // Flush remaining buffer
        if !buf.is_empty() {
            cur_spans.push(Span::styled(buf, style));
            cur_w += buf_w;
        }
    }

    // Flush last line
    if !cur_spans.is_empty() {
        lines.push(Line::from(cur_spans));
    }

    if lines.is_empty() {
        lines.push(Line::from(""));
    }

    lines
}
