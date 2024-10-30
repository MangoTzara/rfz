use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout =
        Layout::default().constraints([Constraint::Length(3), Constraint::Min(1)].as_slice());
    let chunks = layout.split(frame.size());
    frame.render_widget(app.get_state_area(), chunks[0]);

    let binding = app.get_items_with_indices();
    let list: List = binding
        .iter()
        .map(|(key, value)| format_line(key, value))
        .collect::<List>()
        .block(Block::default().borders(Borders::ALL))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    frame.render_stateful_widget(list, chunks[1], app.get_list_state());

    frame.render_widget(
        Span::from(format!(
            "{}/{}",
            app.get_matched_items(),
            app.get_total_items()
        )),
        chunks[1],
    );
}

///Format the span corresponding to a line in the list, highlighting the letters matched
///
/// # Arguments
///
/// * `key` - The text of the line
/// * `matched_indices` - The position of the matches
fn format_line<'a>(line_text: &'a str, matched_indices: &'a [u32]) -> Line<'a> {
    Line::from(
        line_text
            .char_indices()
            .map(|(i, c)| match matched_indices.contains(&(i as u32)) {
                false => Span::raw(String::from(c)),
                true => Span::styled(String::from(c), Style::new().red()),
            })
            .collect::<Vec<Span>>(),
    )
}
