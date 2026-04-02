use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::theme;
use crate::widgets::spinner::FRAMES;

pub struct StatusBar<'a> {
    pub model: &'a str,
    pub mode: &'a str,
    pub cost: f64,
    pub git_branch: Option<&'a str>,
    pub is_streaming: bool,
    pub spinner_frame: usize,
    pub total_input: u64,
    pub total_output: u64,
    pub scroll_pct: Option<u16>,
}

impl<'a> StatusBar<'a> {
    pub fn new(
        model: &'a str,
        mode: &'a str,
        cost: f64,
        git_branch: Option<&'a str>,
        is_streaming: bool,
        spinner_frame: usize,
    ) -> Self {
        Self {
            model,
            mode,
            cost,
            git_branch,
            is_streaming,
            spinner_frame,
            total_input: 0,
            total_output: 0,
            scroll_pct: None,
        }
    }

    pub fn with_tokens(mut self, input: u64, output: u64) -> Self {
        self.total_input = input;
        self.total_output = output;
        self
    }

    pub fn with_scroll(mut self, pct: Option<u16>) -> Self {
        self.scroll_pct = pct;
        self
    }
}

fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in area.x..area.x + area.width {
            buf[(x, area.y)]
                .set_style(Style::default().bg(theme::SURFACE));
        }

        let sep = Span::styled(
            "  ·  ",
            Style::default().fg(theme::HL_HIGH).bg(theme::SURFACE),
        );

        let mode_span = if self.is_streaming {
            let ch = FRAMES[self.spinner_frame % FRAMES.len()];
            Span::styled(
                format!("{ch} generating"),
                Style::default()
                    .fg(theme::IRIS)
                    .bg(theme::SURFACE)
                    .add_modifier(Modifier::ITALIC),
            )
        } else {
            let (icon, color) = match self.mode {
                "Auto-accept" => ("⚡ ", theme::GOLD),
                "Plan" => ("◆ ", theme::IRIS),
                "Bypass" => ("⚠ ", theme::LOVE),
                _ => ("● ", theme::FOAM),
            };
            Span::styled(
                format!("{icon}{}", self.mode),
                Style::default().fg(color).bg(theme::SURFACE),
            )
        };

        let mut spans = vec![
            Span::styled(" ", Style::default().bg(theme::SURFACE)),
            Span::styled(
                self.model,
                Style::default()
                    .fg(theme::FOAM)
                    .bg(theme::SURFACE)
                    .add_modifier(Modifier::BOLD),
            ),
            sep.clone(),
            mode_span,
        ];

        if self.total_input > 0 || self.total_output > 0 {
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!(
                    "↑{} ↓{}",
                    fmt_tokens(self.total_input),
                    fmt_tokens(self.total_output)
                ),
                Style::default().fg(theme::MUTED).bg(theme::SURFACE),
            ));
        }

        if self.cost > 0.0001 {
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("${:.4}", self.cost),
                Style::default().fg(theme::IRIS).bg(theme::SURFACE),
            ));
        }

        if let Some(branch) = self.git_branch {
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("\u{E0A0} {branch}"),
                Style::default().fg(theme::PINE).bg(theme::SURFACE),
            ));
        }

        if let Some(pct) = self.scroll_pct {
            spans.push(sep);
            let label = match pct {
                0 => "TOP".to_string(),
                100 => "BOT".to_string(),
                p => format!("{p}%"),
            };
            spans.push(Span::styled(
                label,
                Style::default().fg(theme::MUTED).bg(theme::SURFACE),
            ));
        }

        Paragraph::new(Line::from(spans)).render(area, buf);
    }
}
