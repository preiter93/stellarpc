use crate::{
    app::{AppContext, Tab},
    controller::Controller,
};

use super::{headers::HeadersTab, messages::MessagesTab, selection::SelectionTab, theme::THEME};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Tabs, Widget},
};
use std::rc::Rc;

pub struct Root<'a, 'b> {
    context: &'a AppContext,
    ctrl: &'a Controller<'b>,
}

impl<'a, 'b> Root<'a, 'b> {
    pub fn new(context: &'a AppContext, ctrl: &'a Controller<'b>) -> Self {
        Root { context, ctrl }
    }
}
impl Root<'_, '_> {
    fn render_navbar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 55]);

        Paragraph::new(Span::styled("StellaRPC", THEME.app_title)).render(area[0], buf);
        let titles = vec![" Selection ", " Messages ", " Address & Headers "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.context.tab.index())
            .divider("")
            .render(area[1], buf);
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        match self.context.tab {
            Tab::Selection => SelectionTab {
                model: &mut self.ctrl.selection.borrow_mut(),
                sub: self.context.sub,
            }
            .render(area, buf),
            Tab::Messages => MessagesTab {
                model: &self.ctrl.messages.borrow(),
                sub: self.context.sub,
            }
            .render(area, buf),
            Tab::Headers => HeadersTab::new(&self.ctrl.headers.borrow()).render(area, buf),
        };
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let keys = match self.context.tab {
            Tab::Selection => SelectionTab::footer_keys(self.context.sub),
            Tab::Messages => MessagesTab::footer_keys(),
            Tab::Headers => HeadersTab::footer_keys(),
        };
        let spans: Vec<Span> = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {key} "), THEME.key_binding.key);
                let desc = Span::styled(format!(" {desc} "), THEME.key_binding.description);
                [key, desc]
            })
            .collect();
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}

impl Widget for Root<'_, '_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_navbar(area[0], buf);
        self.render_content(area[1], buf);
        self.render_footer(area[2], buf);
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}
