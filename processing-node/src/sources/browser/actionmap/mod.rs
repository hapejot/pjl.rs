use crossterm::event::{Event, KeyCode, KeyEvent};

use super::action::{Action, CursorDown, CursorUp, Exit};

pub struct ActionMap {}

impl ActionMap {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn translate(&self, e: Event) -> Option<Box<dyn Action>> {
        match e {
            Event::FocusGained => None,
            Event::FocusLost => None,
            Event::Key(key_event) => self.translate_key_event(key_event),
            Event::Mouse(_mouse_event) => None,
            Event::Paste(_) => None,
            Event::Resize(_, _) => None,
        }
    }
    fn translate_key_event(&self, key_event: KeyEvent) -> Option<Box<dyn Action>> {
        match key_event.code {
            KeyCode::Backspace => todo!(),
            KeyCode::Enter => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => Some(Box::new(CursorUp)),
            KeyCode::Down => Some(Box::new(CursorDown)),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Char(c) => match c {
                'q' => Some(Box::new(Exit)),
                _ => None,
            },
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            KeyCode::CapsLock => todo!(),
            KeyCode::ScrollLock => todo!(),
            KeyCode::NumLock => todo!(),
            KeyCode::PrintScreen => todo!(),
            KeyCode::Pause => todo!(),
            KeyCode::Menu => todo!(),
            KeyCode::KeypadBegin => todo!(),
            KeyCode::Media(_media_key_code) => todo!(),
            KeyCode::Modifier(_modifier_key_code) => todo!(),
        }
    }
}
