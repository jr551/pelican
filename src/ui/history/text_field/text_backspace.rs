use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::ui::view::text_field::CursorMovement;
use crate::platform::history::Action;
use crate::ui::history::text_field::carat_snapshot::CaratSnapshot;

/// A reversible action that inserts text into a text field.
pub struct TextBackspace {
    view: WeakView,
    count: usize,
    cursor_movement: CursorMovement,
    cursors_before: Vec<CaratSnapshot>,
    texts_deleted: Vec<String>,
    cursors_after: Vec<CaratSnapshot>
}

impl TextBackspace {
    pub fn new(view: WeakView, count: usize, cursor_movement: CursorMovement, cursors_before: Vec<CaratSnapshot>) -> TextBackspace {
        TextBackspace {
            view,
            count,
            cursor_movement,
            cursors_before,
            texts_deleted: Vec::new(),
            cursors_after: Vec::new()
        }
    }

    fn text_field(&self) -> TextField {
        let view = self.view.upgrade().unwrap();
        TextField::from_view(view)
    }
}

impl Action for TextBackspace {
    fn name(&self) -> &str {
        "TextBackspace"
    }

    fn forward(&mut self) {
        let text_field = self.text_field();
        text_field.restore_carat_snapshots(&self.cursors_before);
        let texts_delete = text_field.backspace(self.cursor_movement.clone(), self.count);
        self.texts_deleted = texts_delete;
        self.cursors_after = text_field.carat_snapshots();
    }

    fn backward(&mut self) {
        if self.cursors_after.len() == 0 {
            return;
        }

        let text_field = self.text_field();
        let label = text_field.label();

        for (i, string) in self.texts_deleted.iter().enumerate().rev() {
            let carat_snapshot = &self.cursors_after[i];
            let index = carat_snapshot.character_index();
            label.replace_text_in_range(index..index, string);
        }
        text_field.restore_carat_snapshots(&self.cursors_before);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_single_cursor_single_character() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "ab");
        assert_eq!(text_field.carat_indexes(), vec![2]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![3]);
    }

    #[test]
    fn test_single_cursor_multiple_characters() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 2, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "a");
        assert_eq!(text_field.carat_indexes(), vec![1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![3]);
    }

    #[test]
    fn test_single_cursor_word() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Word, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "abc ");
        assert_eq!(text_field.carat_indexes(), vec![4]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![7]);
    }

    #[test]
    fn test_single_cursor_line() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Line, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![7]);
    }

    #[test]
    fn test_multiple_cursors_single_character() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(1, None));
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "b");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![1, 3]);
    }

    #[test]
    fn test_multiple_cursors_multiple_characters() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(1, None));
        carats.push(CaratSnapshot::new(3, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "b");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc");
        assert_eq!(text_field.carat_indexes(), vec![1, 3]);
    }

    #[test]
    fn test_multiple_cursors_word() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Word, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), " ");
        assert_eq!(text_field.carat_indexes(), vec![0, 1]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![3, 7]);
    }

    #[test]
    fn test_multiple_cursors_line() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(3, None));
        carats.push(CaratSnapshot::new(7, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Line, carats);
        action.forward();
        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0, 0]);
        action.backward();
        assert_eq!(text_field.label().text().string(), "abc def");
        assert_eq!(text_field.carat_indexes(), vec![3, 7]);
    }

    #[test]
    fn test_deletion_with_selection_single_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Character, carats);
        action.forward();

        // TODO: need cursor snapshot
        unimplemented!();
    }

    #[test]
    fn test_deletion_with_selection_single_cursor_when_specifying_word() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "abc def".to_string());
        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut action = TextBackspace::new(text_field.view.downgrade(), 1, CursorMovement::Word, carats);
        unimplemented!();
    }
}
