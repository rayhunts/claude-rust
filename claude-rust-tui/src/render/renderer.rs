use ratatui::Frame;
use ratatui::style::Style;
use ratatui::widgets::Block;

use crate::layout::MainLayout;
use crate::state::{AppState, ModalKind};
use crate::theme;
use crate::widgets::{InputBox, MessageList, PermissionDialog, StatusBar};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(frame: &mut Frame, state: &mut AppState) {
        let full = frame.area();
        frame.render_widget(
            Block::default().style(Style::default().bg(theme::BASE)),
            full,
        );
        let [msg, inp, stat] = MainLayout::split(full);

        frame.render_widget(
            MessageList::new(&mut state.conversation, state.spinner_frame),
            msg,
        );
        frame.render_widget(
            InputBox::new(&state.input, !state.is_streaming),
            inp,
        );

        let scroll_pct = if state.conversation.auto_scroll {
            None
        } else {
            let total = state.conversation.total_lines;
            let visible = msg.height as usize;
            if total <= visible {
                None
            } else {
                let max_off = total.saturating_sub(visible);
                let pct = (state.conversation.scroll_offset as f64 / max_off as f64 * 100.0) as u16;
                Some(pct.min(100))
            }
        };

        frame.render_widget(
            StatusBar::new(
                &state.model_name,
                &state.permission_mode,
                state.total_cost,
                state.git_branch.as_deref(),
                state.is_streaming,
                state.spinner_frame,
            )
            .with_tokens(state.total_input, state.total_output)
            .with_scroll(scroll_pct),
            stat,
        );

        if let Some(ModalKind::Permission {
            tool_name,
            description,
        }) = &state.modal.active
        {
            frame.render_widget(
                PermissionDialog::new(tool_name, description),
                full,
            );
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self
    }
}
