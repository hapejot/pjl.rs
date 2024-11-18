use crate::ActiveComponent;

mod cursor;

pub use cursor::{CursorDown, CursorUp, Exit, Resize};

pub trait Action: std::fmt::Debug {
    fn perform(&self, s: &dyn ActiveComponent) -> bool;
}
