use crate::model::headers::{AuthSelection, HeadersModel, HeadersSelection};
use edtui::{editor::theme::EditorTheme, Editor, EditorBuffer, EditorState};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Padding, StatefulWidget, Widget},
};
use tui_widget_list::{List, ListState, Listable};

use super::theme::THEME;

/// The request and response tab
pub struct HeadersTab<'a> {
    model: &'a HeadersModel,
}

impl<'a> HeadersTab<'a> {
    pub fn new(model: &'a HeadersModel) -> Self {
        Self { model }
    }

    pub fn footer_keys() -> Vec<(&'static str, &'static str)> {
        vec![
            ("q", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
            ("Enter", "Select"),
        ]
    }
}

impl Widget for HeadersTab<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 1, 1));
        let mut items: Vec<ListElements> = vec![ListElements::SingleInput(SingleInput {
            buffer: self.model.address.buffer.clone(),
            title: "Address".to_string(),
            selected: self.model.selected == HeadersSelection::Address,
        })];
        match self.model.auth.selected {
            AuthSelection::Bearer => items.append(&mut vec![
                ListElements::VSpace(2),
                ListElements::SingleInput(SingleInput {
                    buffer: self.model.auth.bearer.buffer.clone(),
                    title: "Bearer".to_string(),
                    selected: self.model.selected == HeadersSelection::Auth,
                }),
            ]),
            AuthSelection::Basic => items.append(&mut vec![
                ListElements::VSpace(2),
                ListElements::SingleInput(SingleInput {
                    buffer: self.model.auth.basic.buffer.clone(),
                    title: "Basic".to_string(),
                    selected: self.model.selected == HeadersSelection::Auth,
                }),
            ]),
        }
        let mut list = List::new(items);
        list = list.block(block);
        let mut state = ListState::default();
        list.render(area, buf, &mut state);
    }
}

enum ListElements {
    VSpace(usize),
    SingleInput(SingleInput),
}

impl Listable for ListElements {
    fn height(&self) -> usize {
        match &self {
            Self::VSpace(inner) => *inner,
            Self::SingleInput(inner) => inner.height(),
        }
    }
}
impl Widget for ListElements {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::VSpace(_) => {}
            Self::SingleInput(inner) => inner.render(area, buf),
        };
    }
}

#[derive(Clone)]
struct SingleInput {
    buffer: EditorBuffer,
    title: String,
    selected: bool,
}

impl Listable for SingleInput {
    fn height(&self) -> usize {
        5
    }
}
impl Widget for SingleInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Left)
            .style(THEME.content)
            .padding(Padding::new(1, 1, 0, 1));
        let mut state = EditorState::default();
        let input = Editor::new(&self.buffer, &mut state);
        let mut theme = EditorTheme::default().status_line(None);
        if self.selected {
            block = block.border_type(BorderType::Double);
        }
        if !self.selected {
            theme = theme.cursor_style(EditorTheme::default().base_style());
        }
        theme = theme.block(block.title(self.title.clone()).bold().white());
        input.theme(theme).render(area, buf);
    }
}
