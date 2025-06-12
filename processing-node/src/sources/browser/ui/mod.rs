use std::{
    fs::File,
    io::{self, Write},
};

use crossterm::{
    cursor, event, execute, queue,
    style::{self, Stylize},
    terminal::{self, window_size, ClearType},
};

use crate::sources::Sources;

use super::{action::Action, state::State};

pub trait Control {
    fn display_on(&self, w: &mut impl Write);
}

pub struct ListControl {
    list: Vec<String>,
    current_idx: usize,
    offset: usize,
    y: usize,
    h: usize,
}

impl Control for ListControl {
    fn display_on(&self, w: &mut impl Write) {
        for (lineno, line) in self.list.iter().enumerate() {
            if lineno < self.offset {
                continue;
            }
            if lineno == self.current_idx {
                queue!(
                    w,
                    cursor::MoveToNextLine(1),
                    style::Print(line.clone().negative())
                )
                .unwrap();
            } else {
                queue!(w, cursor::MoveToNextLine(1), style::Print(line)).unwrap();
            }

            if lineno - self.current_idx >= self.h {
                break;
            }
        }
    }
}

impl ListControl {
    pub fn new(list: Vec<String>) -> Self {
        Self {
            list,
            current_idx: 0,
            h: 10,
            y: 0,
            offset: 0,
        }
    }

    fn set_position(&mut self, _x: usize, y: usize) {
        self.y = y;
    }
}

pub struct Application {
    state: State,
}
impl Application {
    pub fn init(sources: Sources<File>) -> Self {
        Application {
            state: State::new(sources),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut w = io::stdout();
        execute!(w, terminal::EnterAlternateScreen)?;
        self.internal_loop(&mut w)?;
        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()
    }

    fn internal_loop(&mut self, w: &mut io::Stdout) -> Result<(), io::Error> {
        let (_cols, rows) = terminal::size()?;
        terminal::enable_raw_mode()?;
        let mut c1 = ListControl::new(
            self.state
                .sources()
                .categories()
                .iter()
                .map(|x| x.to_string())
                .collect(),
        );
        c1.set_position(0, 5);
        Ok(while !self.state.done() {
            // let offset = 3;
            let mut lineno = 0;
            queue!(
                w,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::Hide,
                cursor::MoveTo(0, 0),
                style::Print(self.state.title()),
                cursor::MoveToNextLine(2)
            )?;
            let s = window_size().unwrap();
            queue!(w, cursor::MoveTo(1, 0), style::Print(format!("{:?}", s))).unwrap();
            c1.display_on(w);
            io::Write::flush(w)?;
            if let Some(a) = self.next_action() {
                a.perform(&mut self.state);
            }
        })
    }

    fn next_action(&self) -> Option<Box<dyn Action>> {
        loop {
            if let Ok(e) = event::read() {
                let r = self.state.settings().actions().translate(e);
                return r;
            }
        }
    }
}
