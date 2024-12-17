use crate::ActiveComponent;

use super::Action;

#[derive(Debug)]
pub struct Resize {
    pub w: u16,
    pub h: u16,
}

impl Action for Resize {
    fn perform(&self, s: &dyn ActiveComponent) -> bool {
        let w = format!("{}", self.w);
        let h = format!("{}", self.h);
        let args = vec![w.as_str(), h.as_str()];
        s.handle_activity("resize", &args);
        true
    }
}

#[derive(Debug)]
pub struct CursorUp;

impl Action for CursorUp {
    fn perform(&self, s: &dyn ActiveComponent) -> bool {
        let _ = s;
        // s.move_current_row(-1)
        true
    }
}

#[derive(Debug)]
pub struct CursorDown;

impl Action for CursorDown {
    fn perform(&self, s: &dyn ActiveComponent) -> bool {
        let _ = s;
        // s.move_current_row(1)
        true
    }
}

#[derive(Debug)]
pub struct Exit;

impl Action for Exit {
    fn perform(&self, s: &dyn ActiveComponent) -> bool {
        s.handle_activity("done", &vec![]);
        true
    }
}
