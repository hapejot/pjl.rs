use std::io;

use crossterm::{
    cursor, event, execute, queue, style::{self, Stylize},
    terminal::{self, ClearType},
};

use crate::{action::Action, state::State};

//------------------------- 

pub mod glyph;

//-------------------------



pub struct Application {
    state: crate::state::State,
}
impl Application {
    pub(crate) fn init() -> Self {
        Application {
            state: State::new(),
        }
    }

    pub(crate) fn run(&mut self) -> io::Result<()> {
        let mut w = io::stdout();
        execute!(w, terminal::EnterAlternateScreen)?;
        let (_cols, rows) = terminal::size()?;
        terminal::enable_raw_mode()?;

        while !self.state.done() {
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

            for line in self.state.output_lines() {
                if lineno == self.state.current_row() {
                    queue!(w, cursor::MoveToNextLine(1), style::Print(line.clone().negative()))?;
                }
                else {
                    queue!(w, cursor::MoveToNextLine(1), style::Print(line))?;
                }
                lineno += 1;
                if lineno >= rows as usize {
                    break;
                }
            }

            io::Write::flush(&mut w)?;

            if let Some(a) = self.next_action() {
                a.perform(&mut self.state);
            }

            // match read_char()? {
            //     'q' => {
            //         execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
            //         break;
            //     }
            //     _ => {}
            // };
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
                let r = self.state.settings().actions().translate(e);
                return r;
            }
        }
    }
}
