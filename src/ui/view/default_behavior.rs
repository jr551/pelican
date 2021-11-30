use crate::ui::view::{View, WeakView, Behavior};
use crate::ui::Color;

pub struct DefaultBehavior {
    pub(crate) view: WeakView
}
impl Behavior for DefaultBehavior {
    fn set_view(&mut self, view: WeakView) {
        self.view = view;
    }

    fn get_view(&self) -> &WeakView {
        &self.view
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn mut_super_behavior(&mut self) -> Option<&mut dyn Behavior> {
        None
    }

    /// Request for this view to be redrawn soon.
    ///
    /// See `#draw`, which includes the instructions on what would actually be
    /// drawn to screen.
    fn set_needs_display(&self) {
        let view = self.view.upgrade().unwrap().clone();

        let mut inner_self = view.inner_self.borrow_mut();

        // The layer may not yet exist for this view if it's not drawn to the
        // context at least once. But this is ok, because when a layer is set
        // by `render::window_display()` it will be be implied needs display as
        // default.
        if let Some(layer) = &mut inner_self.layer {
            if layer.get_needs_display() {
                return;
            }

            layer.set_needs_display();

            if let Some(superview) = &inner_self.superview.upgrade() {
                superview.set_needs_display();
            }
        }
    }

    /// Defines what actually gets drawn to screen to represent this view.
    ///
    /// For example, the default `View` implementation simply draws the
    /// background color as a box of the size of the frame.
    fn draw(&self) {
        let view = self.view.upgrade().unwrap().clone();

        let inner_self = view.inner_self.borrow();

        let color = inner_self.background_color.to_graphics_color();

        if let Some(layer) = &inner_self.layer {
            layer.clear_with_color(color);
        }
    }
}
