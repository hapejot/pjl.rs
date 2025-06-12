use std::io::Write;

use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal, QueueableCommand,
};

use tracing::*;

use crate::ui::glyph::Requirement;

use super::{
    next_glyph_number,
    AppError::{self, NotRelevant},
    AppRequest, AppResponse, Glyph, Rect, TermEvent,
};

#[derive(Debug)]
pub struct Panel {
    id: u16,
    area: Rect,
    elements: Vec<Box<dyn Glyph>>,
}

impl Panel {
    pub fn new() -> Self {
        let (h, w) = terminal::size().unwrap();
        let id = next_glyph_number();
        Self {
            id,
            area: Rect::new(h, w),
            elements: vec![],
        }
    }
    pub fn add(&mut self, g: Box<dyn Glyph>) {
        self.elements.push(g);
    }
    #[allow(dead_code)]
    fn write_width_markers(&self, w: &mut dyn Write) {
        info!("area: {:?}", self.area);
        for i in 1..=self.area.w {
            let label: Vec<char> = format!("{i:03}").chars().collect();
            w.queue(MoveTo(self.area.x + i - 1, self.area.y + 1))
                .unwrap();
            w.queue(Print(label[0])).unwrap();
            w.queue(MoveTo(self.area.x + i - 1, self.area.y + 2))
                .unwrap();
            w.queue(Print(label[1])).unwrap();
            w.queue(MoveTo(self.area.x + i - 1, self.area.y + 3))
                .unwrap();
            w.queue(Print(label[2])).unwrap();
        }
    }
}

impl Glyph for Panel {
    fn id(&self) -> u16 {
        self.id
    }
    fn resize(&mut self, width: u16, height: u16) {
        self.area = Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        };
    }

    fn write_to(&self, w: &mut dyn Write) {
        for x in self.elements.iter() {
            x.write_to(w);
        }
    }
    fn area(&self) -> super::Rect {
        self.area.clone()
    }
    fn handle_term_event(&mut self, event: TermEvent) -> AppResponse {
        match event {
            r => {
                let mut handled = Err(AppError::NotRelevant);
                for x in self.elements.iter_mut() {
                    handled = x.handle_term_event(r.clone());
                    if handled.is_ok() {
                        break;
                    }
                }
                handled
            }
        }
    }
    fn request(&self) -> super::Requirements {
        todo!()
    }
    fn allocate(&mut self, allocation: Rect) {
        self.area = allocation.clone();
        info!("allocate {:?}", self.area);
        let mut y = self.area.y;
        let max_y = self.area.y + self.area.h;
        let mut x = self.area.x;
        let max_x = self.area.x + self.area.w;
        for element in self.elements.iter_mut() {
            assert!(max_y >= y);
            let request = element.request();
            match request.h {
                Requirement::Chars(1) => match request.w {
                    Requirement::Chars(w) => {
                        if w + x > max_x {
                            x = self.area.x;
                            y += 1;
                        }
                        element.allocate(Rect { x, y, h: 1, w });
                        x += w;
                    }
                    Requirement::Max => {
                        let w = max_x - x;
                        element.allocate(Rect { x, y, h: 1, w });
                        y += 1;
                        x = self.area.x;
                    }
                },
                Requirement::Chars(h) => {
                    let x = allocation.x;
                    let w = allocation.w;
                    element.allocate(Rect { x, y, w, h });
                    y += h;
                }
                Requirement::Max => {
                    let x = allocation.x;
                    let w = allocation.w;
                    let h = max_y - y;
                    element.allocate(Rect { x, y, w, h });
                    y = max_y;
                }
            }
        }
    }
    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse {
        let result = Err(NotRelevant);
        let mut results = vec![];
        for x in self.elements.iter_mut() {
            let r = x.handle_app_request(req);
            if let Ok(res) = r {
                for x in res {
                    results.push(x);
                }
            }
        }
        result
    }
    fn hit(&mut self, x: u16, y: u16) -> super::AppResponse {
        let mut r = Err(NotRelevant);
        for el in self.elements.iter_mut() {
            let el_result = el.hit(x, y);
            if el_result.is_ok() {
                r = el_result.clone();
            }
        }
        r
    }

    fn allocated(&self) -> bool {
        self.area.w > 0 && self.area.h > 0
    }

    fn name(&self) -> String {
        String::from("Panel")
    }
}
