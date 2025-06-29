use crossterm::event::KeyEvent;
use crossterm::{
    cursor, event, execute, queue, style,
    terminal::{self, ClearType},
};
use std::{
    cell::{Cell, RefCell},
    io,
    rc::Rc,
};
use tracing::*;
use ui::glyph::{Glyph, Rect, TermEvent};

pub mod action;
pub mod actionmap;
pub mod settings;
pub mod state;
pub mod ui;

#[derive(Debug, Clone)]
pub struct Screen {
    g: Rc<RefCell<Box<dyn Glyph>>>,
}

impl Screen {
    pub fn new(g: Box<dyn Glyph>) -> Self {
        Self {
            g: Rc::new(RefCell::new(g)),
        }
    }
}
impl Glyph for Screen {
    fn id(&self) -> u16 {
        self.g.borrow().id()
    }

    fn hit(&mut self, x: u16, y: u16) -> ui::glyph::AppResponse {
        self.g.borrow_mut().hit(x, y)
    }

    fn write_to(&self, w: &mut dyn io::Write) {
        self.g.borrow().write_to(w)
    }

    fn area(&self) -> ui::glyph::Rect {
        self.g.borrow().area()
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.g.borrow_mut().resize(width, height)
    }

    fn handle_term_event(&mut self, r: TermEvent) -> ui::glyph::AppResponse {
        self.g.borrow_mut().handle_term_event(r)
    }

    fn handle_app_request(&mut self, req: &ui::glyph::AppRequest) -> ui::glyph::AppResponse {
        self.g.borrow_mut().handle_app_request(req)
    }

    fn request(&self) -> ui::glyph::Requirements {
        self.g.borrow_mut().request()
    }

    fn allocate(&mut self, allocation: ui::glyph::Rect) {
        self.g.borrow_mut().allocate(allocation)
    }

    fn allocated(&self) -> bool {
        self.g.borrow().allocated()
    }

    fn name(&self) -> String {
        self.g.borrow().name()
    }
}

pub trait ActiveComponent {
    fn handle_activity(&self, selector: &str, args: &[&str]);
}

pub trait TApplication: std::fmt::Debug {
    fn name(&self) -> &str;
    fn screen(&self, id: usize) -> Screen;
}

#[derive(Debug)]
pub struct Application<'a> {
    name: String,
    done: Cell<bool>,
    app: RefCell<&'a dyn TApplication>,
}

impl<'a> Application<'a> {
    pub fn new(app: &'a dyn TApplication) -> Self {
        Self {
            name: app.name().to_string(),
            done: Cell::new(false),
            app: RefCell::new(app),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut w = io::stdout();
        execute!(w, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        let mut screen = self.app.borrow().screen(1);
        if let Ok((cols, rows)) = terminal::size() {
            let area = Rect::new(cols, rows);
            screen.allocate(area);
        }
        while !self.done.get() {
            queue!(
                w,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::Hide,
                cursor::MoveTo(0, 0),
                style::Print(&self.name),
                cursor::MoveToNextLine(2)
            )?;

            // render controls...
            screen.write_to(&mut w);
            io::Write::flush(&mut w)?;

            if let Ok(e) = event::read() {
                match e {
                    crossterm::event::Event::Key(key_event) => {
                        match screen.handle_term_event(translate(key_event)){
                            Ok(_x) => todo!(),
                            Err(e) => error!("could not handle {:?} -> {:?}", key_event, e),
                        }
                    }
                    crossterm::event::Event::Mouse(_) => todo!(),
                    crossterm::event::Event::Resize(cols, rows) => {
                        let area = Rect::new(cols, rows);
                        screen.allocate(area);
                    }
                    _ => todo!(),
                };
            }

            // if let Some(action) = &self.next_action() {
            //     info!("action: {:#?}", action);
            //     action.perform(self);
            // }
        }
        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()
    }

}

fn translate(key_event: event::KeyEvent) -> TermEvent {
    let KeyEvent { code, .. } = key_event;
    match code {
        event::KeyCode::Backspace => todo!(),
        event::KeyCode::Enter => todo!(),
        event::KeyCode::Left => TermEvent::Left,
        event::KeyCode::Right => TermEvent::Right,
        event::KeyCode::Up => TermEvent::Up,
        event::KeyCode::Down => TermEvent::Down,
        event::KeyCode::Home => TermEvent::GoToStart,
        event::KeyCode::End => TermEvent::GoToEnd,
        event::KeyCode::PageUp => TermEvent::PageUp,
        event::KeyCode::PageDown => TermEvent::PageDown,
        event::KeyCode::Tab => todo!(),
        event::KeyCode::BackTab => todo!(),
        event::KeyCode::Delete => todo!(),
        event::KeyCode::Insert => todo!(),
        event::KeyCode::F(_) => todo!(),
        event::KeyCode::Char(c) => TermEvent::Key(c),
        event::KeyCode::Null => todo!(),
        event::KeyCode::Esc => todo!(),
        event::KeyCode::CapsLock => todo!(),
        event::KeyCode::ScrollLock => todo!(),
        event::KeyCode::NumLock => todo!(),
        event::KeyCode::PrintScreen => todo!(),
        event::KeyCode::Pause => todo!(),
        event::KeyCode::Menu => todo!(),
        event::KeyCode::KeypadBegin => todo!(),
        event::KeyCode::Media(_) => todo!(),
        event::KeyCode::Modifier(_) => todo!(),
    }
}

impl<'a> ActiveComponent for Application<'a> {
    fn handle_activity(&self, selector: &str, args: &[&str]) {
        let _ = args;
        match selector {
            "done" => {
                self.done.replace(true);
            }
            "resize" => {
                let w: u16 = args[0].parse().unwrap();
                let h: u16 = args[1].parse().unwrap();
                debug!("resize {}x{}", w, h);
            }
            _ => todo!("{}", selector),
        }
    }
}
