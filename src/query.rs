use std::str::Utf8Error;

#[derive(Default)]
struct Query {
    cursor_position: u16,
    text: Vec<u8>,
}

#[allow(dead_code)]
impl Query {
    fn push(&mut self, to_push: char) {
        self.cursor_position += 1;
        self.text.push(to_push as u8);
    }

    fn text(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.text)
    }

    fn delete(&mut self) {
        if self.text.pop().is_some() {
            self.cursor_position -= 1;
        }
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
        assert_eq!(sut.text(), Ok("aa"));
        assert_eq!(sut.cursor_position, 2);
    }

    #[test]
    fn delete() {
        let mut sut = Query::default();
        sut.push('a');
        sut.push('a');

        sut.delete();
        assert_eq!(sut.text(), Ok("a"));
        assert_eq!(sut.cursor_position, 1);
    }

    #[test]
    fn delete_empty() {
        let mut sut = Query::default();

        sut.delete();
        assert_eq!(sut.cursor_position, 0);
    }
}
