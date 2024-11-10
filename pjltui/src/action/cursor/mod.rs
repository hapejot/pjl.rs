use super::Action;

pub struct CursorUp;

impl Action for CursorUp {
    fn perform(&self, s: &mut crate::state::State) -> bool {
        s.move_current_row(-1)
    }
}

pub struct CursorDown;

impl Action for CursorDown {
    fn perform(&self, s: &mut crate::state::State) -> bool {
        s.move_current_row(1)
    }
}

pub struct Exit;

impl Action for Exit {
    fn perform(&self, s: &mut crate::state::State) -> bool {
        s.set_done(true);
        true
    }
}
