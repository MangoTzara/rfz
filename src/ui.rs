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
    // let mut matcher: Nucleo<String> = Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 10);
    // matcher.pattern.reparse(
    //     9,
    //     app.query.as_str(),
    //     CaseMatching::Ignore,
    //     Normalization::Never,
    //     true,
    // );
    // for c in app.paths.iter() {
    //     matcher.injector().push(c.to_string(), |_| {});
    // }
    // matcher.tick(100);
    // let mut res: Vec<String> = Vec::new();
    // let snap = matcher.snapshot();
    // for i in 0..snap.item_count() {
    //     match snap.get_item(i) {
    //         Some(x) => res.push(x.data.to_string()),
    //         None => break,
    //     }
    // }
    // let items: Vec<_> =
    //     nucleo::pattern::Pattern::parse(&app.query, CaseMatching::Ignore, Normalization::Smart)
    //         .match_list(
    //             app.paths.clone().into_iter(),
    //             &mut Matcher::new(Config::DEFAULT),
    //         )
    //         .iter()
    //         .filter(|c| c.1 > 70)
    //         .map(|c| c.0.clone())
    // //         .collect();
    // let widget = Paragraph::new(Span::styled(

    //     Style::new(),
    // ))
    // .block(Block::default().borders(Borders::ALL).title(">"));
    // frame.render_widget(widget, chunks[1]);

    let list = List::new(
        app.snapshot()
            .matched_items(0..app.snapshot().matched_item_count())
            .map(|c| c.data.clone())
            .collect::<Vec<String>>(),
    )
    .block(Block::default().borders(Borders::ALL))
    .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true);
    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
}
