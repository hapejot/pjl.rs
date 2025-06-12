use super::AppError::*;
use super::AppRequest::*;
use super::AppResult::*;
use super::*;
use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};

#[derive(Debug)]
pub struct Input {
    id: u16,
    active: bool,
    pos: u16,
    area: Rect,
    name: String,
    txt: Vec<char>,
}

impl Input {
    pub fn new<T: ToString, U: ToString>(name: T, txt: U) -> Self {
        let id = next_glyph_number();
        Self {
            id,
            active: false,
            pos: 0,
            area: Rect::new(30, 1),
            txt: txt.to_string().chars().collect(),
            name: name.to_string(),
        }
    }

    fn handle_backspace(&mut self) -> AppResponse {
        if self.active {
            if self.txt.len() >= self.pos as usize {
                self.pos -= 1;
                self.txt.remove(self.pos as usize);
                Ok(vec![NewCursorPosition(self.area.x + self.pos, self.area.y)])
            } else {
                Ok(vec![NewCursorPosition(
                    self.area.x + self.pos - 1,
                    self.area.y,
                )])
            }
        } else {
            Err(NotRelevant)
        }
    }

    fn handle_keypress(&mut self, ch: char) -> AppResponse {
        if self.active {
            if (self.txt.len() as u16) <= self.pos {
                self.txt.push(ch);
                self.pos = self.txt.len() as u16;
            } else {
                self.txt[self.pos as usize] = ch;
                self.pos += 1;
            }
            Ok(vec![NewCursorPosition(self.area.x + self.pos, self.area.y)])
        } else {
            Err(NotRelevant)
        }
    }
}

impl Glyph for Input {
    fn id(&self) -> u16 {
        self.id
    }
    fn write_to(&self, w: &mut dyn std::io::Write) {
        w.queue(MoveTo(self.area.x, self.area.y)).unwrap();
        let mut idx = 0;
        for c in self.txt.iter() {
            idx += 1;
            w.queue(Print(c)).unwrap();
        }
        for _ in idx..self.area.w {
            w.queue(Print('_')).unwrap();
        }
    }
    fn name(&self) -> String {
        String::from("Input")
    }

    fn area(&self) -> Rect {
        todo!()
    }

    fn resize(&mut self, _width: u16, _height: u16) {
        todo!()
    }

    fn handle_term_event(&mut self, event: TermEvent) -> AppResponse {
        match event {
            TermEvent::BackSpace => self.handle_backspace(),
            TermEvent::Key(ch) => self.handle_keypress(ch),
            _ => Err(AppError::NotRelevant),
        }
    }

    fn request(&self) -> super::Requirements {
        Requirements {
            w: super::Requirement::Max,
            h: super::Requirement::Chars(1),
        }
    }

    fn allocate(&mut self, allocation: Rect) {
        self.area = allocation;
        info!("allocate {:?}", &self.area);
    }

    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse {
        match req {
            SetValue { name, value } => {
                if name == &self.name {
                    self.txt = value.chars().collect();
                    Ok(vec![Redraw])
                } else {
                    Err(NotRelevant)
                }
            }
            GetValue(name) => {
                if name == &self.name {
                    Ok(vec![AppResult::StringValue(self.txt.iter().collect())])
                } else {
                    Err(NotRelevant)
                }
            }
            NextInput(_, y) => {
                if self.area.y > *y {
                    Ok(vec![NewCursorPosition(self.area.x, self.area.y)])
                } else {
                    Err(NotRelevant)
                }
            }
            CollectAllValues => Ok(vec![Values(vec![(
                self.name.clone(),
                self.txt.iter().collect(),
            )])]),
            _ => Err(NotRelevant),
        }
    }

    fn hit(&mut self, x: u16, y: u16) -> super::AppResponse {
        if self.area.contains(x, y) {
            self.active = true;
            self.pos = x - self.area.x;
            Ok(vec![AppResult::InputEnabled])
        } else {
            self.active = false;
            Err(NotRelevant)
        }
    }

    fn allocated(&self) -> bool {
        self.area.w > 0 && self.area.h > 0
    }
}
