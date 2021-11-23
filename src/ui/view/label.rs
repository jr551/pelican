use crate::graphics::Rectangle;
use crate::graphics::Font;
use crate::ui::Color;
use crate::ui::{View, WeakView};
use crate::ui::view::{Behavior, DefaultBehavior};
use std::cell::RefCell;

pub struct LabelBehavior{
    view: WeakView,
    super_behavior: Box<dyn Behavior>,
    font: RefCell<Font<'static, 'static>>,
    text_color: Color,
    text: RefCell<String>
}

pub struct Label {
    pub view: View
}

impl Label {
    pub fn new(frame: Rectangle<i32, u32>, font: Font<'static, 'static>, text: String) -> Label {
        let super_behavior = DefaultBehavior {
            view: WeakView::none()
        };

        let behavior = LabelBehavior {
            view: WeakView::none(),
            super_behavior: Box::new(super_behavior),
            font: RefCell::new(font),
            text_color: Color::black(),
            text: RefCell::new(text)
        };

        let view = View::new_with_behavior(Box::new(behavior), frame);
        Label {
            view
        }
    }

    fn set_text(&self, text: String) {
        let behavior = &self.view.behavior.borrow();
        let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();
        behavior.text.replace(text);
        behavior.set_needs_display();
    }

    // TODO:
    // font=(value)
    // text_color=(value)
    // text_alignment=(value)
    // number_of_lines=(value)
}

impl Behavior for LabelBehavior {
    fn set_view(&mut self, view: WeakView) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView {
        &self.view
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn draw(&self) {
        let view = self.view.upgrade().unwrap().clone();
        let inner_self = view.inner_self.borrow();
        let behavior = view.behavior.borrow();
        let behavior = behavior.as_any().downcast_ref::<LabelBehavior>().unwrap();

        let text = behavior.text.borrow();

        if let Some(layer) = &inner_self.layer {
            let mut font = behavior.font.borrow_mut();
            let child_layer = font.layer_for(layer.context.clone(), &text);
            let position = inner_self.frame.position.clone();
            let size = child_layer.get_size().clone();
            let destination = Rectangle { position, size };
            layer.draw_child_layer(&child_layer, &destination);
        }
    }
}

// module UserInterface
//   class Label < View
//     def initialize(frame)
//       super
//       @background_color = UserInterface::Color.clear
//       @text_color = Color.black
//       @text_alignment = :left
//       @number_of_lines = 1
//       @font = CoreGraphics::Font.new("Arial", 17)
//     end

//     attr_reader :text
//     attr_reader :font
//     attr_reader :text_color
//     attr_reader :text_alignment
//     attr_reader :number_of_lines

//     def text=(value)
//       @text = value
//       refresh_text_layer
//       set_needs_display
//     end

//     def font=(value)
//       @font = value
//       refresh_text_layer
//       set_needs_display
//     end

//     def text_color=(value)
//       @text_color = value
//       set_needs_display
//     end

//     def text_alignment=(value)
//       @text_alignment = value
//       set_needs_display
//     end

//     def number_of_lines=(value)
//       @number_of_lines = value
//       set_needs_display
//     end

//     def draw
//       super

//       raise "No font for #{self}" unless @font

//       @text_layer ||= @font.layer_for(window.graphics_context, text)

//       layer.draw_child_layer(
//         @text_layer,
//         0,
//         0,
//         @text_layer.size.width,
//         @text_layer.size.height
//       )
//     end

//     private

//     def refresh_text_layer
//       @text_layer = nil
//     end
//   end
// end
