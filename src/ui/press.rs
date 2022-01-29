use crate::ui::key::Key;
use std::time::Instant;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

struct PressInner {
    key: Key,
    phase: PressPhase,
    timestamp: Instant
}

pub struct Press {
    inner: Rc<RefCell<PressInner>>
}

impl Press {
    pub(crate) fn new(key: Key) -> Press {
        Press {
            inner: Rc::new(RefCell::new(PressInner {
                key,
                phase: PressPhase::Began,
                timestamp: Instant::now()
            }))
        }
    }

    pub fn key(&self) -> Ref<'_, Key> {
        Ref::map(self.inner.borrow(), |inner| &inner.key)
    }

    pub fn phase(&self) -> Ref<'_, PressPhase> {
        Ref::map(self.inner.borrow(), |inner| &inner.phase)
    }

    pub fn timestamp(&self) -> Ref<'_, Instant> {
        Ref::map(self.inner.borrow(), |inner| &inner.timestamp)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PressPhase {
    Began,
    Ended
}

impl PartialEq for Press {
    fn eq(&self, other: &Press) -> bool {
        *self.key() == *other.key()
    }
}

impl std::fmt::Debug for Press {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Press")
         .field(&self.key())
         .finish()
    }
}

impl Clone for Press {
    fn clone(&self) -> Press {
        Press {
            inner: self.inner.clone()
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn press_defaults() {
//         let press = Press::new(Key::A);
//         assert_eq!(press.key(), &Key::A);
//         assert_eq!(press.phase(), &PressPhase::Began);
//     }

//     #[test]
//     fn press_eq() {
//         let press1 = Press::new(Key::A);
//         let press2 = Press::new(Key::A);
//         assert_eq!(press1, press2);
//     }
// }
