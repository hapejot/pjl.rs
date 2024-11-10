use super::state::State;

mod cursor;

pub use cursor::{CursorDown, CursorUp, Exit};

pub trait Action {
    fn perform(&self, s: &mut State) -> bool;
}
