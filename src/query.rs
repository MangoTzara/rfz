use std::ops::SubAssign;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Widget, WidgetRef},
};

#[derive(Default)]
pub struct Query {
    cursor_position: usize,
    style: Style,
    text: String,
}

const BLOCK: &str = "\u{2588}";

impl WidgetRef for Query {
    #[doc = " Draws the current state of the widget in the given buffer. That is the only method required"]
    #[doc = " to implement a custom widget."]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let line = self.to_widget();
        line.render(area, buf);
    }
}

#[allow(dead_code)]
impl Query {
    pub fn new(style: Style) -> Self {
        Self {
            cursor_position: 0,
            style,
            text: String::new(),
        }
    }

    pub fn push(&mut self, to_push: char) {
        unsafe {
            self.text
                .as_mut_vec()
                .insert(self.cursor_position, to_push as u8)
        };

        self.cursor_position += 1;
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn pop(&mut self) -> bool {
        if self.text.is_empty() {
            false
        } else {
            if self.cursor_position.ne(&0) {
                self.cursor_position.sub_assign(1);
            }
            self.text.remove(self.cursor_position);
            true
        }
    }

    pub(crate) fn push_str(&mut self, to_paste: &str) {
        self.cursor_position += to_paste.len();
        self.text.push_str(to_paste);
    }

    pub fn move_left(&mut self, how_much: usize) {
        self.cursor_position = self
            .cursor_position
            .checked_sub(how_much)
            .unwrap_or_default();
    }
    pub fn move_right(&mut self, how_much: usize) {
        if self.text.is_empty() {
            return;
        } else if self.cursor_position + how_much > self.text.len() {
            self.cursor_position = self.text.len();
        } else {
            self.cursor_position += how_much;
        }
    }

    fn to_widget(&self) -> Line {
        if self.text.is_empty() {
            Line::styled(BLOCK, self.style.clone())
        } else if self.cursor_position < self.text.len() {
            let first = Span::styled(&self.text[..self.cursor_position], self.style.clone());
            let middle = Span::styled(
                self.text
                    .chars()
                    .nth(self.cursor_position)
                    .unwrap()
                    .to_string(),
                self.style.clone().add_modifier(Modifier::REVERSED),
            );
            let second = Span::styled(&self.text[self.cursor_position + 1..], self.style.clone());
            Line::from(vec![first, middle, second])
        } else {
            Line::styled(format!("{}{BLOCK}", &self.text), self.style.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use ratatui::{
        style::{Modifier, Style},
        text::Span,
    };

    use crate::query::BLOCK;

    use super::Query;

    #[test]
    fn push() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');
        assert_eq!(sut.text(), "aa");
        assert_eq!(sut.cursor_position, 2);
    }

    #[test]
    fn push_at_pos() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');
        sut.move_left(1);
        sut.push('b');
        assert_eq!(sut.text(), "aba");
        assert_eq!(sut.cursor_position, 2);
    }
    #[test]
    fn delete() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');

        sut.pop();
        assert_eq!(sut.text(), "a");
        assert_eq!(sut.cursor_position, 1);
    }
    #[test]
    fn delete_at_post() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('b');
        sut.move_left(1);
        sut.pop();
        assert_eq!(sut.text(), "b");
        assert_eq!(sut.cursor_position, 0);
    }
    #[test]
    fn delete_at_first() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('b');
        sut.move_left(2);
        sut.pop();
        assert_eq!(sut.text(), "b");
        assert_eq!(sut.cursor_position, 0);
    }

    #[test]
    fn delete_empty() {
        let mut sut = Query::default();

        sut.pop();
        assert_eq!(sut.cursor_position, 0);
    }

    #[test]
    fn move_left() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');
        sut.move_left(1);
        assert_eq!(sut.cursor_position, 1);
    }
    #[test]
    fn move_left_at_limit() {
        let mut sut = Query::default();
        sut.move_left(1);
        assert_eq!(sut.cursor_position, 0);
    }
    #[test]
    fn move_right() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');
        sut.move_left(1);
        sut.move_right(1);
        assert_eq!(sut.cursor_position, 2);
    }
    #[test]
    fn move_right_at_limit() {
        let mut sut = Query::default();
        sut.push('a');
        sut.move_right(1);
        assert_eq!(sut.cursor_position, 1);
    }

    #[test]
    fn to_widget_empty() {
        let sut = Query::default();
        let sut: ratatui::prelude::Line = sut.to_widget();
        let sut = sut.into_iter().collect::<Vec<Span>>();
        assert_eq!(sut.len(), 1);
        assert_eq!(sut.get(0), Some(&Span::raw(BLOCK)));
    }
    #[test]
    fn to_widget_end_cursor() {
        let mut sut = Query::default();
        sut.push('a');
        let sut: ratatui::prelude::Line = sut.to_widget();
        let sut = sut.into_iter().collect::<Vec<Span>>();
        assert_eq!(sut.len(), 1);
        assert_eq!(sut.get(0), Some(&Span::raw(format!("a{BLOCK}"))));
    }
    #[test]
    fn to_widget_middle() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');
        sut.push('a');
        sut.move_left(2);
        let sut: ratatui::prelude::Line = sut.to_widget();
        let sut = sut.into_iter().collect::<Vec<Span>>();

        let inverted = Span::styled("a", Style::default().add_modifier(Modifier::REVERSED));
        assert_eq!(sut.len(), 3);
        assert_eq!(sut, vec![Span::raw("a"), inverted, Span::raw("a")]);
    }
}
