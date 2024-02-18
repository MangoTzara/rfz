use std::borrow::BorrowMut;

use ratatui::{
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout =
        Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    let chunks = layout.split(frame.size());
    let widget = Paragraph::new(Span::styled(app.query.clone(), Style::new()))
        .block(Block::default().borders(Borders::ALL).title(">"));
    frame.render_widget(widget, chunks[0]);

    // frame.render_widget(
    //     Span::raw(matched_items.into_iter().next().unwrap().data),
    //     chunks[1],
    // );
    // matched_items.for_each(|c| frame.render_widget(Span::raw(c.data.clone()), chunks[1]));
    let list = List::new(app.get_items())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
}
