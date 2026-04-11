use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::widgets::spinner::FRAMES;
use crate::theme;

pub struct StatusBar<'a> {
    pub model: &'a str,
    pub model_is_default: bool,
    pub mode: &'a str,
    pub cost: f64,
    pub git_branch: Option<&'a str>,
    pub is_streaming: bool,
    pub spinner_frame: usize,
}

impl<'a> StatusBar<'a> {
    pub fn new(model: &'a str, model_is_default: bool, mode: &'a str, cost: f64, git_branch: Option<&'a str>, is_streaming: bool, spinner_frame: usize) -> Self {
        Self { model, model_is_default, mode, cost, git_branch, is_streaming, spinner_frame }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_style(Style::default().bg(theme::surface()));
        }

        let sep = Span::styled("  ·  ", Style::default().fg(theme::hl_high()).bg(theme::surface()));

        let mode_span = if self.is_streaming {
            let ch = FRAMES[self.spinner_frame % FRAMES.len()];
            Span::styled(format!("{ch} generating"), Style::default().fg(theme::iris()).bg(theme::surface()).add_modifier(Modifier::ITALIC))
        } else {
            let (icon, color) = match self.mode {
                "Auto-accept" => ("⚡ ", theme::gold()),
                "Plan"        => ("◆ ", theme::iris()),
                "Bypass"      => ("⚠ ", theme::love()),
                _             => ("● ", theme::foam()),
            };
            Span::styled(format!("{icon}{}", self.mode), Style::default().fg(color).bg(theme::surface()))
        };

        let model_display = if self.model_is_default { "default" } else { self.model };
        let mut spans = vec![
            Span::styled(" ", Style::default().bg(theme::surface())),
            Span::styled(model_display, Style::default().fg(theme::foam()).bg(theme::surface()).add_modifier(Modifier::BOLD)),
            sep.clone(),
            mode_span,
            sep.clone(),
            Span::styled(format!("${:.4}", self.cost), Style::default().fg(theme::iris()).bg(theme::surface())),
        ];

        if let Some(branch) = self.git_branch {
            spans.push(sep);
            spans.push(Span::styled(
                format!("⎇ {branch}"),
                Style::default().fg(theme::pine()).bg(theme::surface()),
            ));
        }

        Paragraph::new(Line::from(spans)).render(area, buf);
    }
}
