use crossterm::{cursor::MoveTo, style::Print, terminal, QueueableCommand};
use tracing::*;

use super::{next_glyph_number, AppRequest, AppResponse, Glyph, Rect, TermEvent};

#[derive(Debug)]
pub struct Frame {
    id: u16,
    area: Rect,
    content: Box<dyn Glyph>,
}

impl Frame {
    pub fn new(content: Box<dyn Glyph>) -> Self {
        let (h, w) = terminal::size().unwrap();
        let id = next_glyph_number();
        Self {
            id,
            area: Rect::new(w, h),
            content,
        }
    }
}

impl Glyph for Frame {
    fn id(&self) -> u16 {
        self.id
    }
    fn write_to(&self, w: &mut dyn std::io::Write) {
        if self.area.w > 2 && self.area.h > 2 {
            self.content.write_to(w);
            let r = self.area();
            w.queue(MoveTo(r.x, r.y)).unwrap();
            w.queue(Print("┌")).unwrap();
            horizontal_line(r.w - 2, w);
            w.queue(Print("┐")).unwrap();

            for y in 0..(r.h - 2) {
                w.queue(MoveTo(r.x, r.y + 1 + y)).unwrap();
                w.queue(Print("│")).unwrap();
                w.queue(MoveTo(r.x + r.w - 1, r.y + y + 1)).unwrap();
                w.queue(Print("│")).unwrap();
            }

            w.queue(MoveTo(r.x, r.y + r.h)).unwrap();
            w.queue(Print("└")).unwrap();
            horizontal_line(r.w - 2, w);
            w.queue(Print("┘")).unwrap();
        }
    }

    fn name(&self) -> String {
        String::from("Frame")
    }

    fn area(&self) -> super::Rect {
        self.area.clone()
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.content.resize(width, height);
    }

    fn handle_term_event(&mut self, r: TermEvent) -> AppResponse {
        let r = self.content.handle_term_event(r);
        r
    }

    fn request(&self) -> super::Requirements {
        todo!()
    }

    fn allocate(&mut self, allocation: super::Rect) {
        info!("allocated {:?}", &allocation);
        self.area = allocation.clone();
        self.content.allocate(Rect {
            x: allocation.x + 1,
            y: allocation.y + 1,
            w: allocation.w - 2,
            h: allocation.h - 2,
        });
    }

    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse {
        self.content.handle_app_request(req)
    }

    fn hit(&mut self, x: u16, y: u16) -> super::AppResponse {
        self.content.hit(x, y)
    }

    fn allocated(&self) -> bool {
        self.area.w > 2 && self.area.h > 2
    }
}

fn horizontal_line(width: u16, w: &mut dyn std::io::Write) {
    for _i in 0..(width) {
        w.queue(Print("─")).unwrap();
    }
}
