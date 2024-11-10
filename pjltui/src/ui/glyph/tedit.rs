use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent},
    style::Print,
};
use tracing::info;

use super::{
    next_glyph_number, AppError, AppRequest, AppResponse, AppResult, Glyph, Rect, Requirement,
    Requirements,
};

pub struct TextEdit {
    id: u16,
    area: Rect,
    pos: (u16, u16),
    txt: Vec<Vec<char>>,
}

impl TextEdit {
    pub fn new() -> Self {
        let txt = vec![];
        let area = Rect::new(10, 1);
        let pos = (0, 0);
        let id = next_glyph_number();
        Self { id, txt, area, pos }
    }
}

impl Glyph for TextEdit {
    fn id(&self) -> u16 {
        self.id
    }
    fn hit(&mut self, x: u16, y: u16) -> AppResponse {
        if self.area.contains(x, y) {
            // self.pos = x - self.area.x;
            self.pos = (x - self.area.x, y - self.area.y);
            // info!("hit: {:?}", self.pos);
            Ok(vec![AppResult::InputEnabled])
        } else {
            Err(AppError::NotRelevant)
        }
    }

    fn write_to(&self, w: &mut dyn std::io::Write) {
        use crossterm::QueueableCommand;
        let mut y = self.area.y;
        for x in self.txt.iter() {
            w.queue(MoveTo(self.area.x, y)).unwrap();
            for c in x.iter() {
                w.queue(Print(c)).unwrap();
            }
            y += 1;
        }
    }

    fn area(&self) -> super::Rect {
        self.area.clone()
    }

    fn resize(&mut self, width: u16, height: u16) {
        let _ = (width, height);
        todo!()
    }

    fn handle_term_event(&mut self, r: Event) -> AppResponse {
        info!("event {:?}", r);
        match r {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) => match code {
                KeyCode::Char(c) => {
                    let _ = (modifiers, kind, state);
                    while self.txt.len() <= self.pos.1 as usize {
                        self.txt.push(vec![]);
                    }
                    let l = self.txt.get_mut(self.pos.1 as usize).unwrap();
                    // l.push(c);
                    while l.len() <= self.pos.0 as usize {
                        l.push(' ');
                    }
                    {
                        let x = self.pos.0 as usize;
                        l[x] = c;
                        self.pos.0 += 1;
                    }
                    let new_pos = AppResult::NewCursorPosition(
                        self.area.x + self.pos.0,
                        self.area.y + self.pos.1,
                    );
                    Ok(vec![new_pos, AppResult::Redraw])
                }
                KeyCode::Backspace => todo!(),
                KeyCode::Enter => todo!(),
                KeyCode::Left => todo!(),
                KeyCode::Right => todo!(),
                KeyCode::Up => todo!(),
                KeyCode::Down => todo!(),
                KeyCode::Home => todo!(),
                KeyCode::End => todo!(),
                KeyCode::PageUp => todo!(),
                KeyCode::PageDown => todo!(),
                KeyCode::Tab => todo!(),
                KeyCode::BackTab => todo!(),
                KeyCode::Delete => todo!(),
                KeyCode::Insert => todo!(),
                KeyCode::F(_) => todo!(),
                KeyCode::Null => todo!(),
                KeyCode::Esc => todo!(),
                KeyCode::CapsLock => todo!(),
                KeyCode::ScrollLock => todo!(),
                KeyCode::NumLock => todo!(),
                KeyCode::PrintScreen => todo!(),
                KeyCode::Pause => todo!(),
                KeyCode::Menu => todo!(),
                KeyCode::KeypadBegin => todo!(),
                KeyCode::Media(_) => todo!(),
                KeyCode::Modifier(_) => todo!(),
            },
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }

    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse {
        info!("{} handle request {:?}", self.id, req);
        match req {
            AppRequest::SetValue { .. } => {                
                Err(AppError::NotRelevant)
            }
            AppRequest::GetValue(_) => todo!(),
            AppRequest::CollectAllValues => {
                let ls = self
                    .txt
                    .iter()
                    .map(|line| line.iter().collect::<String>())
                    .collect::<Vec<_>>();
                Ok(vec![AppResult::Values(vec![(
                    String::from("text"),
                    ls.join("\n"),
                )])])
            }
            _ => AppResponse::Err(AppError::NotRelevant),
        }
    }

    fn request(&mut self) -> super::Requirements {
        Requirements {
            w: Requirement::Max,
            h: Requirement::Max,
        }
    }

    fn allocate(&mut self, allocation: super::Rect) {
        info!("allocate {:?}", allocation);
        self.area = allocation;
    }

    fn allocated(&self) -> bool {
        info!("allocated");
        true
    }

    fn name(&self) -> String {
        todo!()
    }
}
