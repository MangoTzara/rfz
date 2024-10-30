use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, List, Widget, WidgetRef},
};

#[derive(Default)]
pub struct Query {
    cursor_position: usize,
    style: ratatui::style::Style,
    text: String,
}

impl WidgetRef for Query {
    #[doc = " Draws the current state of the widget in the given buffer. That is the only method required"]
    #[doc = " to implement a custom widget."]
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text = Span::styled(&self.text, self.style.clone());
        let cursor = Span::styled("", self.style.clone());
        let line = List::new([text, cursor])
            .block(Block::bordered().title_top(Span::styled(">", self.style.clone())));

        line.render(area, buf);
    }
}

#[allow(dead_code)]
impl Query {
    pub fn new(style: ratatui::style::Style) -> Self {
        Self {
            cursor_position: 0,
            style,
            text: String::new(),
        }
    }

    pub fn push(&mut self, to_push: char) {
        self.cursor_position += 1;
        self.text.push(to_push);
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn pop(&mut self) -> bool {
        if self.text.pop().is_some() {
            self.cursor_position -= 1;
            true
        } else {
            false
        }
    }

    pub(crate) fn push_str(&mut self, to_paste: &str) {
        self.cursor_position += to_paste.len();
        self.text.push_str(to_paste);
    }
}

#[cfg(test)]
mod test {

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
    fn delete() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');

        sut.pop();
        assert_eq!(sut.text(), "a");
        assert_eq!(sut.cursor_position, 1);
    }

    #[test]
    fn delete_empty() {
        let mut sut = Query::default();

        sut.pop();
        assert_eq!(sut.cursor_position, 0);
    }
}
