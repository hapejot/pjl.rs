use std::{
    io::{self, Write},
    sync::{Mutex, MutexGuard},
};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, style, terminal,
};
use serde_json::Value;

pub mod grid;

pub trait Render {
    fn render(&self) -> std::io::Result<()>;
}

pub trait HandleEvent {
    fn handle_event(&mut self, event: event::Event) -> bool;
}

pub fn dump_table_to_console(t: &pjl_tab::Table) {
    let mut out = String::new();
    t.dump(&mut out);
    eprintln!("{}", out);
}

pub fn read_table_from_stream<S>(mut input_stream: S) -> pjl_tab::Table
where
    S: std::io::Read,
{
    let mut buf = String::new();
    input_stream.read_to_string(&mut buf).unwrap();
    let value: Value = serde_json::from_str(&buf).unwrap();

    let t = pjl_tab::ser::table_from(&value).unwrap();
    t
}

pub struct App<M> {
    model: Mutex<M>,
    views: Vec<Box<dyn Render>>,
    controllers: Vec<Box<dyn HandleEvent>>,
    messages: Vec<String>,
}
impl<M> HandleEvent for App<M> {
    fn handle_event(&mut self, event: event::Event) -> bool {
        match event {
            event::Event::FocusGained => todo!(),
            event::Event::FocusLost => todo!(),
            event::Event::Key(key_event) => self.handle_key_event(key_event.code),
            event::Event::Mouse(mouse_event) => todo!(),
            event::Event::Paste(_) => todo!(),
            event::Event::Resize(_, _) => todo!(),
        }
    }
}

impl<M> Render for App<M> {
    fn render(&self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        // Terminal leeren
        execute!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, 0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            style::SetForegroundColor(style::Color::White),
            style::SetBackgroundColor(style::Color::Blue),
            style::Print("System Dynamics Modellierung".to_string()),
            style::ResetColor
        )?;

        execute!(
            stdout,
            cursor::MoveTo(0, 2),
        )?;

        for x in self.views.iter() {
            x.render()?;
        }

        // Meldungen anzeigen
        execute!(
            stdout,
            cursor::MoveTo(0, 20),
            style::SetForegroundColor(style::Color::Yellow)
        )?;
        for (i, message) in self.messages.iter().enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(0, 20 + i as u16),
                style::Print(message)
            )?;
        }
        execute!(stdout, style::ResetColor)?;
        stdout.flush()?;

        Ok(())
    }
}

impl<M> App<M> {
    pub fn new(m: M) -> Self {
        Self {
            model: Mutex::new(m),
            messages: Vec::new(),
            controllers: vec![],
            views: vec![],
        }
    }

    pub fn model(&self) -> MutexGuard<M> {
        self.model.lock().unwrap()
    }

    pub fn add_message(&self, msg: &str) {}

    fn handle_key_event(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') => false,
            _ => true,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        // Terminal in Raw-Mode und Alternate Screen wechseln
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture
        )?;

        // Event-Loop
        loop {
            self.render()?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    if !self.handle_key_event(key.code) {
                        break;
                    }
                }
            }
        }

        // Terminal wiederherstellen
        execute!(
            io::stdout(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture,
            cursor::Show,
        )?;
        terminal::disable_raw_mode()?;

        Ok(())
    }

    pub fn add_view(&mut self, view: Box<dyn Render>) {
        self.views.push(view);
    }
}
