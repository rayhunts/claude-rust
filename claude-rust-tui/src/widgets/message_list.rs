use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::state::{ConversationState, ToolUseStatus};
use crate::theme;
use crate::widgets::spinner::FRAMES;

pub struct MessageList<'a> {
    pub state: &'a mut ConversationState,
    pub spinner_frame: usize,
}

impl<'a> MessageList<'a> {
    pub fn new(state: &'a mut ConversationState, spinner_frame: usize) -> Self {
        Self { state, spinner_frame }
    }
}

fn parse_inline(text: &str) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut buf = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if !buf.is_empty() {
                spans.push(Span::styled(
                    std::mem::take(&mut buf),
                    Style::default().fg(theme::TEXT),
                ));
            }
            i += 2;
            while i < chars.len()
                && !(i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*')
            {
                buf.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(
                std::mem::take(&mut buf),
                Style::default()
                    .fg(theme::ROSE)
                    .add_modifier(Modifier::BOLD),
            ));
            if i + 1 < chars.len() {
                i += 2;
            }
        } else if chars[i] == '`' {
            if !buf.is_empty() {
                spans.push(Span::styled(
                    std::mem::take(&mut buf),
                    Style::default().fg(theme::TEXT),
                ));
            }
            i += 1;
            while i < chars.len() && chars[i] != '`' {
                buf.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(
                format!(" {} ", std::mem::take(&mut buf)),
                Style::default().fg(theme::FOAM).bg(theme::OVERLAY),
            ));
            if i < chars.len() {
                i += 1;
            }
        } else if chars[i] == '*' || chars[i] == '_' {
            let delim = chars[i];
            if !buf.is_empty() {
                spans.push(Span::styled(
                    std::mem::take(&mut buf),
                    Style::default().fg(theme::TEXT),
                ));
            }
            i += 1;
            while i < chars.len() && chars[i] != delim {
                buf.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(
                std::mem::take(&mut buf),
                Style::default()
                    .fg(theme::SUBTLE)
                    .add_modifier(Modifier::ITALIC),
            ));
            if i < chars.len() {
                i += 1;
            }
        } else {
            buf.push(chars[i]);
            i += 1;
        }
    }
    if !buf.is_empty() {
        spans.push(Span::styled(buf, Style::default().fg(theme::TEXT)));
    }
    spans
}

fn render_md_line(raw: &str, in_code: bool) -> Line<'static> {
    let trimmed = raw.trim_end();
    if in_code {
        return Line::from(Span::styled(
            format!("  │ {trimmed}"),
            Style::default().fg(theme::SUBTLE),
        ));
    }
    if let Some(r) = trimmed.strip_prefix("### ") {
        return Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                r.to_string(),
                Style::default()
                    .fg(theme::FOAM)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
    }
    if let Some(r) = trimmed.strip_prefix("## ") {
        return Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                r.to_string(),
                Style::default()
                    .fg(theme::ROSE)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
    }
    if let Some(r) = trimmed.strip_prefix("# ") {
        return Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                r.to_string(),
                Style::default()
                    .fg(theme::GOLD)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
    }
    if trimmed.chars().all(|c| c == '-') && trimmed.len() >= 3 {
        return Line::from(Span::styled(
            format!("  {}", "─".repeat(trimmed.len().min(60))),
            Style::default().fg(theme::HL_MED),
        ));
    }
    let (pre, body) = if let Some(r) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
    {
        ("  • ".to_string(), r)
    } else if let Some(r) = trimmed
        .strip_prefix("  - ")
        .or_else(|| trimmed.strip_prefix("  * "))
    {
        ("    ◦ ".to_string(), r)
    } else if let Some(r) = trimmed
        .strip_prefix("    - ")
        .or_else(|| trimmed.strip_prefix("    * "))
    {
        ("      ▪ ".to_string(), r)
    } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed.contains(". ") {
        let dot_pos = trimmed.find(". ").unwrap();
        let num = &trimmed[..dot_pos + 2];
        let rest = &trimmed[dot_pos + 2..];
        let mut spans = vec![Span::styled(
            format!("  {num}"),
            Style::default()
                .fg(theme::PINE)
                .add_modifier(Modifier::BOLD),
        )];
        spans.extend(parse_inline(rest));
        return Line::from(spans);
    } else {
        ("  ".to_string(), trimmed)
    };
    let mut spans = vec![Span::styled(pre, Style::default().fg(theme::PINE))];
    spans.extend(parse_inline(body));
    Line::from(spans)
}

fn render_welcome(lines: &mut Vec<Line<'static>>, area_height: u16) {
    let pad = (area_height as usize).saturating_sub(8) / 2;
    for _ in 0..pad {
        lines.push(Line::from(""));
    }
    lines.push(Line::from(vec![Span::styled(
        "  ◆  Claude Code",
        Style::default()
            .fg(theme::FOAM)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Type a message to get started",
        Style::default().fg(theme::MUTED),
    )));
    lines.push(Line::from(Span::styled(
        "  Use /help for available commands",
        Style::default().fg(theme::MUTED),
    )));
    lines.push(Line::from(""));
}

impl<'a> Widget for MessageList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect {
            y: area.y + 1,
            height: area.height.saturating_sub(1),
            ..area
        };
        let mut lines: Vec<Line<'static>> = Vec::new();

        if self.state.messages.is_empty() {
            render_welcome(&mut lines, area.height);
        }

        for (msg_idx, msg) in self.state.messages.iter().enumerate() {
            if msg_idx > 0 {
                lines.push(Line::from(Span::styled(
                    format!("  {}", "·".repeat(3)),
                    Style::default().fg(theme::HL_MED),
                )));
                lines.push(Line::from(""));
            }

            match msg.role.as_str() {
                "user" => {
                    for (i, raw) in msg.content.lines().enumerate() {
                        let line = if i == 0 {
                            Line::from(vec![
                                Span::styled(
                                    "  ❯ ",
                                    Style::default()
                                        .fg(theme::FOAM)
                                        .add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(
                                    raw.trim_end().to_string(),
                                    Style::default().fg(theme::TEXT),
                                ),
                            ])
                        } else {
                            Line::from(Span::styled(
                                format!("    {}", raw.trim_end()),
                                Style::default().fg(theme::TEXT),
                            ))
                        };
                        lines.push(line);
                    }
                }
                "error" => {
                    for (i, raw) in msg.content.lines().enumerate() {
                        let line = if i == 0 {
                            Line::from(vec![
                                Span::styled(
                                    "  ✗ ",
                                    Style::default()
                                        .fg(theme::LOVE)
                                        .add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(
                                    raw.trim_end().to_string(),
                                    Style::default().fg(theme::LOVE),
                                ),
                            ])
                        } else {
                            Line::from(Span::styled(
                                format!("    {}", raw.trim_end()),
                                Style::default().fg(theme::LOVE),
                            ))
                        };
                        lines.push(line);
                    }
                }
                "system" => {
                    for raw in msg.content.lines() {
                        lines.push(Line::from(Span::styled(
                            format!("  · {}", raw.trim_end()),
                            Style::default()
                                .fg(theme::MUTED)
                                .add_modifier(Modifier::ITALIC),
                        )));
                    }
                }
                _ => {
                    if !msg.thinking.is_empty() {
                        let think_lines: Vec<&str> = msg.thinking.lines().collect();
                        let show = if msg.is_streaming {
                            &think_lines[think_lines.len().saturating_sub(3)..]
                        } else {
                            let max_show = 5;
                            if think_lines.len() > max_show {
                                lines.push(Line::from(Span::styled(
                                    format!(
                                        "  ◈ thinking ({} lines, showing last {max_show})",
                                        think_lines.len()
                                    ),
                                    Style::default()
                                        .fg(theme::MUTED)
                                        .add_modifier(Modifier::ITALIC),
                                )));
                                &think_lines[think_lines.len().saturating_sub(max_show)..]
                            } else {
                                lines.push(Line::from(Span::styled(
                                    "  ◈ thinking",
                                    Style::default()
                                        .fg(theme::MUTED)
                                        .add_modifier(Modifier::ITALIC),
                                )));
                                &think_lines[..]
                            }
                        };
                        if msg.is_streaming {
                            let ch = FRAMES[self.spinner_frame % FRAMES.len()];
                            lines.push(Line::from(Span::styled(
                                format!("  {ch} thinking..."),
                                Style::default()
                                    .fg(theme::IRIS)
                                    .add_modifier(Modifier::ITALIC),
                            )));
                        }
                        for raw in show {
                            lines.push(Line::from(Span::styled(
                                format!("    {}", raw.trim_end()),
                                Style::default()
                                    .fg(theme::MUTED)
                                    .add_modifier(Modifier::DIM),
                            )));
                        }
                        if !msg.content.is_empty() {
                            lines.push(Line::from(""));
                        }
                    }
                    let mut in_code = false;
                    let mut in_mermaid = false;
                    for raw in msg.content.lines() {
                        let trimmed_start = raw.trim_start();
                        if trimmed_start.starts_with("```mermaid") {
                            in_mermaid = true;
                            in_code = true;
                            lines.push(Line::from(Span::styled(
                                "  ╭─ mermaid ",
                                Style::default()
                                    .fg(theme::IRIS)
                                    .add_modifier(Modifier::BOLD),
                            )));
                        } else if in_mermaid && trimmed_start.starts_with("```") {
                            in_mermaid = false;
                            in_code = false;
                            lines.push(Line::from(Span::styled(
                                "  ╰──────────",
                                Style::default().fg(theme::IRIS),
                            )));
                        } else if in_mermaid {
                            lines.push(Line::from(vec![
                                Span::styled("  │ ", Style::default().fg(theme::IRIS)),
                                Span::styled(
                                    raw.trim_end().to_string(),
                                    Style::default().fg(theme::GOLD),
                                ),
                            ]));
                        } else if trimmed_start.starts_with("```") {
                            in_code = !in_code;
                            if in_code {
                                let lang = trimmed_start.strip_prefix("```").unwrap_or("");
                                let label = if lang.is_empty() {
                                    "  ╭─ code ".to_string()
                                } else {
                                    format!("  ╭─ {lang} ")
                                };
                                lines.push(Line::from(Span::styled(
                                    label,
                                    Style::default()
                                        .fg(theme::PINE)
                                        .add_modifier(Modifier::BOLD),
                                )));
                            } else {
                                lines.push(Line::from(Span::styled(
                                    "  ╰──────────",
                                    Style::default().fg(theme::PINE),
                                )));
                            }
                        } else {
                            lines.push(render_md_line(raw, in_code));
                        }
                    }

                    for tool in &msg.tool_uses {
                        let (icon, name_str, col) = match tool.status {
                            ToolUseStatus::Running => {
                                let ch =
                                    FRAMES[self.spinner_frame % FRAMES.len()].to_string();
                                let cap = tool
                                    .name
                                    .chars()
                                    .next()
                                    .map(|c| {
                                        c.to_uppercase().to_string()
                                            + &tool.name[c.len_utf8()..]
                                    })
                                    .unwrap_or_else(|| tool.name.clone());
                                (ch, format!("{cap}..."), theme::GOLD)
                            }
                            ToolUseStatus::Completed => {
                                ("✓".into(), tool.name.clone(), theme::FOAM)
                            }
                            ToolUseStatus::Error => {
                                ("✗".into(), tool.name.clone(), theme::LOVE)
                            }
                        };
                        let preview = if tool.output_preview.is_empty() {
                            String::new()
                        } else {
                            format!("  {}", tool.output_preview)
                        };
                        lines.push(Line::from(vec![
                            Span::styled(
                                format!("  {icon} "),
                                Style::default()
                                    .fg(col)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                name_str,
                                Style::default()
                                    .fg(col)
                                    .add_modifier(Modifier::ITALIC),
                            ),
                            Span::styled(
                                preview,
                                Style::default()
                                    .fg(theme::MUTED)
                                    .add_modifier(Modifier::DIM),
                            ),
                        ]));
                    }
                }
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(""));

        let total = lines.len();
        let visible = area.height as usize;
        self.state.total_lines = total;
        let offset = if self.state.auto_scroll {
            let off = total.saturating_sub(visible);
            self.state.scroll_offset = off;
            off
        } else {
            self.state.scroll_offset
        };

        Paragraph::new(lines)
            .scroll((offset as u16, 0))
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
