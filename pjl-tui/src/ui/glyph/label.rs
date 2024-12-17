use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};
use tracing::*;
// use std::backtrace::Backtrace;
use super::{
    next_glyph_number, AppError::{self, NotRelevant}, AppRequest::{self, SetValue}, AppResponse, AppResult::Redraw, Glyph, Rect, Requirements, TermEvent
};

#[derive(Debug)]
pub struct Label {
    id: u16,
    area: Rect,
    name: String,
    txt: String,
}

impl Label {
    pub fn new<T: ToString, U: ToString>(name: T, txt: U) -> Self {
        let id = next_glyph_number();
        let txt = txt.to_string();
        Self {
            id,
            area: Rect::new(txt.len() as u16, 1),
            name: name.to_string(),
            txt,
        }
    }
}

impl Glyph for Label {
    fn id(&self) -> u16 { self.id }
    fn write_to(&self, w: &mut dyn std::io::Write) {
        w.queue(MoveTo(self.area.x, self.area.y)).unwrap();
        w.queue(Print(self.txt.clone())).unwrap();
    }
    fn name(&self) -> String {
        String::from("Label")
    }

    fn area(&self) -> Rect {
        self.area.clone()
    }

    fn resize(&mut self, _width: u16, _height: u16) {
        todo!()
    }

    fn handle_term_event(
        &mut self,
        event: TermEvent,
    ) -> AppResponse {
        match event {
            _ => Err(AppError::NotRelevant),
        }
    }

    fn request(&self) -> super::Requirements {
        let r = Requirements {
            w: super::Requirement::Chars(self.area.w),
            h: super::Requirement::Chars(self.area.h),
        };
        trace!("requirement: {:?} ", r); // , Backtrace::force_capture());
        r
    }

    fn allocate(&mut self, allocation: Rect) {
        self.area = allocation;
        info!("allocate {:?}", &self.area);
    }

    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse {
        match req {
            SetValue { name, value } => {
                if name == &self.name {
                    self.txt = value.clone();
                    Ok(vec![Redraw])
                } else {
                    Err(NotRelevant)
                }
            }
            _ => Err(NotRelevant),
        }
    }

    fn hit(&mut self, _x: u16, _y: u16) -> super::AppResponse {
        Err(NotRelevant)
    }

    fn allocated(&self) -> bool {
        true
    }
}
