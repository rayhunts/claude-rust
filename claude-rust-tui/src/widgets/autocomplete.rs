use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

use crate::theme;

pub struct AutocompleteDropdown<'a> {
    pub options: &'a [String],
    pub selected: usize,
}

impl<'a> AutocompleteDropdown<'a> {
    pub fn new(options: &'a [String], selected: usize) -> Self {
        Self { options, selected }
    }
}

impl<'a> Widget for AutocompleteDropdown<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem<'_>> = self.options.iter().enumerate().map(|(i, opt)| {
            let style = if i == self.selected {
                Style::default().fg(theme::base()).bg(theme::pine()).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::text())
            };
            ListItem::new(Line::from(Span::styled(opt.as_str(), style)))
        }).collect();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::overlay()))
            .style(Style::default().bg(theme::surface()));

        List::new(items).block(block).render(area, buf);
    }
}
