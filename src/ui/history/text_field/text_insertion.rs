use crate::ui::view::WeakView;
use crate::ui::view::TextField;
use crate::platform::history::Action;
use crate::ui::view::text_field::CursorMovement;
use crate::ui::history::text_field::carat_snapshot::CaratSnapshot;

/// A reversible action that inserts text into a text field.
pub struct TextInsertion {
    view: WeakView,
    text: String,
    cursors_before: Vec<CaratSnapshot>,
    cursors_after: Vec<CaratSnapshot>
}

impl TextInsertion {
    pub fn new(view: WeakView, text: String, cursors_before: Vec<CaratSnapshot>) -> TextInsertion {
        TextInsertion {
            view,
            text,
            cursors_before,
            cursors_after: Vec::new()
        }
    }

    fn text_field(&self) -> TextField {
        let view = self.view.upgrade().unwrap();
        TextField::from_view(view)
    }
}

impl Action for TextInsertion {
    fn name(&self) -> &str {
        "TextInsertion"
    }

    fn forward(&mut self) {
        let text_field = self.text_field();
        text_field.restore_carat_snapshots(&self.cursors_before);
        text_field.insert_str(&self.text);
        self.cursors_after = text_field.carat_snapshots();
    }

    fn backward(&mut self) {
        if self.cursors_after.len() == 0 {
            return;
        }

        let text_field = self.text_field();
        text_field.restore_carat_snapshots(&self.cursors_after);
        text_field.backspace(CursorMovement::Character, self.text.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Rectangle;

    #[test]
    fn test_forward() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello");
        assert_eq!(text_field.carat_indexes(), vec![5]);
    }

    #[test]
    fn test_forward_multi_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "|".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));
        carats.push(CaratSnapshot::new(1, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );


        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello|Hello");
        assert_eq!(text_field.carat_indexes(), vec![5, 11]);
    }

    #[test]
    fn test_backward() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();
        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "");
        assert_eq!(text_field.carat_indexes(), vec![0]);
    }

    #[test]
    fn test_backward_multi_cursor() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "|".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, None));
        carats.push(CaratSnapshot::new(1, None));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello|Hello");

        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 2);
        assert_eq!(carats[0].character_index(), 5);
        assert_eq!(carats[0].selection(), &None);
        assert_eq!(carats[1].character_index(), 11);
        assert_eq!(carats[1].selection(), &None);

        text_insertion.backward();

        assert_eq!(text_field.label().text().string(), "|");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 2);
        assert_eq!(carats[0].character_index(), 0);
        assert_eq!(carats[0].selection(), &None);
        assert_eq!(carats[1].character_index(), 1);
        assert_eq!(carats[1].selection(), &None);
    }

    #[test]
    fn test_insertion_with_selection() {
        let frame = Rectangle::new(0, 0, 100, 100);
        let text_field = TextField::new(frame, "Hi world".to_string());

        let mut carats = Vec::new();
        carats.push(CaratSnapshot::new(0, Some(0..2)));

        let mut text_insertion = TextInsertion::new(
            text_field.view.downgrade(),
            "Hello".to_string(),
            carats
        );

        text_insertion.forward();

        assert_eq!(text_field.label().text().string(), "Hello world");
        let carats = text_field.carat_snapshots();
        assert_eq!(carats.len(), 1);
        assert_eq!(carats[0].character_index(), 5);
        assert_eq!(carats[0].selection(), &None);
    }
}
