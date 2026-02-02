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
        let mut spans: Vec<Span<'_>> = Vec::new();
        spans.push(match evt.level {
            Level::Error => "ERROR ".red().bold(),
            Level::Warn => "WARN  ".yellow().bold(),
            Level::Info => "INFO  ".green().bold(),
            Level::Debug => "DEBUG ".blue().bold(),
            Level::Trace => "TRACE ".magenta().bold(),
        });
        spans.push(" | ".dim());

        spans.push(
            evt.timestamp
                .format("%Y-%m-%d %H:%M:%S | ")
                .to_string()
                .dim(),
        );
        #[cfg(debug_assertions)]
        spans.push(if let Some(file) = evt.file() {
            format!("{}:{} | ", file, evt.line.unwrap_or(0)).dim()
        } else {
            " ".into()
        });

        let split_str = split_target(evt.msg().to_string());
        split_str.iter().enumerate().for_each(|(i, part)| {
            if i == split_str.len() - 1 {
                spans.push(part.to_string().into());
            } else {
                spans.push(format!("{} | ", part).dim());
            }
        });

        wrap_spans(spans, width.saturating_sub(1))
    }
}

fn wrap_spans<'a>(spans: Vec<Span<'a>>, width: usize) -> Vec<Line<'a>> {
    let width = width.max(1);

    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut cur_spans: Vec<Span<'a>> = Vec::new();
    let mut cur_w: usize = 0;

    let push_token =
        |cur_spans: &mut Vec<Span<'a>>, cur_w: &mut usize, tok: String, style: Style| {
            if !tok.is_empty() {
                *cur_w += UnicodeWidthStr::width(tok.as_str());
                cur_spans.push(Span::styled(tok, style));
            }
        };

    let flush_line =
        |lines: &mut Vec<Line<'a>>, cur_spans: &mut Vec<Span<'a>>, cur_w: &mut usize| {
            lines.push(Line::from(std::mem::take(cur_spans)));
            *cur_w = 0;
        };

    let split_long_token = |lines: &mut Vec<Line<'a>>,
                            cur_spans: &mut Vec<Span<'a>>,
                            cur_w: &mut usize,
                            tok: &str,
                            style: Style| {
        let mut chunk = String::new();
        let mut chunk_w = 0usize;

        for ch in tok.chars() {
            let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);

            if *cur_w + chunk_w + ch_w > width {
                if !chunk.is_empty() {
                    cur_spans.push(Span::styled(std::mem::take(&mut chunk), style));
                }
                flush_line(lines, cur_spans, cur_w);
                chunk_w = 0;
            }

            chunk.push(ch);
            chunk_w += ch_w;
        }

        if !chunk.is_empty() {
            cur_spans.push(Span::styled(chunk, style));
            *cur_w += chunk_w;
        }
    };

    for sp in spans {
        let style = sp.style;
        let s = sp.content.as_ref();

        // Fast path if whole span fits
        let span_w = UnicodeWidthStr::width(s);
        if cur_w + span_w <= width {
            cur_spans.push(sp);
            cur_w += span_w;
            continue;
        }

        // Tokenize span into: words, whitespace, and '\n' boundaries
        let mut tok = String::new();
        let mut tok_is_ws: Option<bool> = None;

        let flush_tok = |lines: &mut Vec<Line<'a>>,
                         cur_spans: &mut Vec<Span<'a>>,
                         cur_w: &mut usize,
                         tok: &mut String,
                         tok_is_ws: &mut Option<bool>| {
            if tok.is_empty() {
                *tok_is_ws = None;
                return;
            }

            let is_ws = tok_is_ws.unwrap_or(false);
            let tok_w = UnicodeWidthStr::width(tok.as_str());

            // If token fits on this line, push it
            if *cur_w + tok_w <= width {
                push_token(cur_spans, cur_w, std::mem::take(tok), style);
                *tok_is_ws = None;
                return;
            }

            // If it's whitespace and doesn't fit, drop it
            if is_ws {
                tok.clear();
                *tok_is_ws = None;
                return;
            }

            // It's a word that doesn't fit.
            // If the line already has content, wrap to next line first, then try again.
            if *cur_w > 0 {
                flush_line(lines, cur_spans, cur_w);
            }

            // Now if the word is still too long, hard-split it.
            if tok_w > width {
                let word = std::mem::take(tok);
                split_long_token(lines, cur_spans, cur_w, &word, style);
            } else {
                // Fits on empty line
                push_token(cur_spans, cur_w, std::mem::take(tok), style);
            }

            *tok_is_ws = None;
        };

        for ch in s.chars() {
            if ch == '\n' {
                flush_tok(
                    &mut lines,
                    &mut cur_spans,
                    &mut cur_w,
                    &mut tok,
                    &mut tok_is_ws,
                );
                flush_line(&mut lines, &mut cur_spans, &mut cur_w);
                continue;
            }

            let is_ws = ch.is_whitespace();

            match tok_is_ws {
                None => {
                    tok_is_ws = Some(is_ws);
                    tok.push(ch);
                }
                Some(prev_is_ws) if prev_is_ws == is_ws => {
                    tok.push(ch);
                }
                Some(_) => {
                    flush_tok(
                        &mut lines,
                        &mut cur_spans,
                        &mut cur_w,
                        &mut tok,
                        &mut tok_is_ws,
                    );
                    tok_is_ws = Some(is_ws);
                    tok.push(ch);
                }
            }

            if tok.len() > 4096 {
                flush_tok(
                    &mut lines,
                    &mut cur_spans,
                    &mut cur_w,
                    &mut tok,
                    &mut tok_is_ws,
                );
            }
        }

        flush_tok(
            &mut lines,
            &mut cur_spans,
            &mut cur_w,
            &mut tok,
            &mut tok_is_ws,
        );
    }

    if !cur_spans.is_empty() {
        lines.push(Line::from(cur_spans));
    }

    if lines.is_empty() {
        lines.push(Line::from(""));
    }

    lines
}

// Filtering out name: and addy: prefixes from targets
fn split_target(input: String) -> Vec<String> {
    let name_regex = regex::Regex::new(r#"(name:\s\S*)(.*)"#).unwrap();
    let address_regex = regex::Regex::new(r#"(addy:\s[\d\\.:]*)(.*)"#).unwrap();
    let mut parts = Vec::new();
    if let Some(caps) = name_regex.captures(&input) {
        if let Some(rest) = caps.get(2) {
            parts.push(rest.as_str().to_string());
        }
    } else if let Some(caps) = address_regex.captures(&input) {
        parts.push(caps.get(1).unwrap().as_str().to_string());
        if let Some(rest) = caps.get(2) {
            parts.push(rest.as_str().to_string());
        }
    } else {
        parts.push(input);
    }
    parts
}
