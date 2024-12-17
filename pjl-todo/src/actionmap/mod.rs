use crossterm::event::KeyEvent;

use crate::action::Action;

pub struct ActionMap {}

impl ActionMap {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn translate(&self, e: crossterm::event::Event) -> Option<Box<dyn Action>> {
        match e {
            crossterm::event::Event::FocusGained => None,
            crossterm::event::Event::FocusLost => None,
            crossterm::event::Event::Key(key_event) => self.translate_key_event(key_event),
            crossterm::event::Event::Mouse(_mouse_event) => None,
            crossterm::event::Event::Paste(_) => None,
            crossterm::event::Event::Resize(_, _) => None,
        }
    }
    fn translate_key_event(&self, key_event: KeyEvent) -> Option<Box<dyn Action>> {
        match key_event.code {
            crossterm::event::KeyCode::Backspace => todo!(),
            crossterm::event::KeyCode::Enter => todo!(),
            crossterm::event::KeyCode::Left => todo!(),
            crossterm::event::KeyCode::Right => todo!(),
            crossterm::event::KeyCode::Up => Some(Box::new(crate::action::CursorUp)),
            crossterm::event::KeyCode::Down => Some(Box::new(crate::action::CursorDown)),
            crossterm::event::KeyCode::Home => todo!(),
            crossterm::event::KeyCode::End => todo!(),
            crossterm::event::KeyCode::PageUp => todo!(),
            crossterm::event::KeyCode::PageDown => todo!(),
            crossterm::event::KeyCode::Tab => todo!(),
            crossterm::event::KeyCode::BackTab => todo!(),
            crossterm::event::KeyCode::Delete => todo!(),
            crossterm::event::KeyCode::Insert => todo!(),
            crossterm::event::KeyCode::F(_) => todo!(),
            crossterm::event::KeyCode::Char(c) => match c {
                'q' => Some(Box::new(crate::action::Exit)),
                _ => None,
            },
            crossterm::event::KeyCode::Null => todo!(),
            crossterm::event::KeyCode::Esc => todo!(),
            crossterm::event::KeyCode::CapsLock => todo!(),
            crossterm::event::KeyCode::ScrollLock => todo!(),
            crossterm::event::KeyCode::NumLock => todo!(),
            crossterm::event::KeyCode::PrintScreen => todo!(),
            crossterm::event::KeyCode::Pause => todo!(),
            crossterm::event::KeyCode::Menu => todo!(),
            crossterm::event::KeyCode::KeypadBegin => todo!(),
            crossterm::event::KeyCode::Media(_media_key_code) => todo!(),
            crossterm::event::KeyCode::Modifier(_modifier_key_code) => todo!(),
        }
    }
}
