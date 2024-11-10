use std::io;

use action::Action;
use crossterm::{
    cursor, event, execute, queue, style,
    terminal::{self, ClearType},
};
use tracing::*;

pub mod action;
pub mod actionmap;
pub mod settings;
pub mod state;
pub mod ui;

pub trait TApplication {
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct Application {
    name: String,
    done: bool,
}

impl Application {
    pub fn new(app: &mut dyn TApplication) -> Self {
        Self {
            name: app.name().to_string(),
            done: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut w = io::stdout();
        execute!(w, terminal::EnterAlternateScreen)?;
        let (cols, rows) = terminal::size()?;
        terminal::enable_raw_mode()?;

        info!("terminal size: {cols} cols, {rows} rows");

        while !self.done {
            let mut lineno = 0;
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

            io::Write::flush(&mut w)?;

            if let Some(a) = &self.next_action() {
                // a.perform(&mut self.state);
            }
        }
        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()
    }

    fn next_action(&self) -> Option<Box<dyn Action>> {
        loop {
            if let Ok(e) = event::read() {
                match e {
                    event::Event::FocusGained => debug!("focus gained"),
                    event::Event::FocusLost => debug!("focus lost"),
                    event::Event::Key(key_event) => debug!("key {:?}", key_event),
                    event::Event::Mouse(mouse_event) => debug!("mouse {:?}", mouse_event),
                    event::Event::Paste(_) => debug!("paste"),
                    event::Event::Resize(_, _) => debug!("resize"),
                }
                let r = None;
                return r;
            }
        }
    }
}
