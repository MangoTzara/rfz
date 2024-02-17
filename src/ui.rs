use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text,
    widgets::{Block, BorderType, Borders, List, Paragraph},
    Frame,
};
use tui_textarea::{Input, Key, TextArea};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.counter
    //     ))
    //     .block(
    //         Block::bordered()
    //             .title("Template")
    //             .title_alignment(Alignment::Center)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //     .centered(),
    //     frame.size(),
    // )
    let layout =
        Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    let mut textarea = TextArea::default();
    textarea.set_block(Block::default().borders(Borders::ALL).title(">"));
    let chunks = layout.split(frame.size());
    let widget = textarea.widget();
    frame.render_widget(widget, chunks[0]);
    let chunks = layout.split(frame.size());
    let widget = textarea.widget();
    frame.render_widget(widget, chunks[0]);
    let list = List::new(app.paths.clone())
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true); // f.render_stateful_widget(list, chunks[1], &mut state.clone());
    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
}
