use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, ResetColor},
    terminal::{self, ClearType},
};
use std::collections::HashMap;
use std::io::{self, Write};

const VIEW_WIDTH: usize = 10;
const VIEW_HEIGHT: usize = 10;

type Position = (i32, i32); // Infinite scrolling with signed coordinates

struct Spreadsheet {
    cells: HashMap<Position, String>,
    cursor: Position,
    offset: Position,
}

impl Spreadsheet {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            cursor: (0, 0),
            offset: (0, 0),
        }
    }

    fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen)?;

        loop {
            self.draw(&mut stdout)?;
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => self.cursor.0 -= 1,
                    KeyCode::Right => self.cursor.0 += 1,
                    KeyCode::Up => self.cursor.1 -= 1, 
                    KeyCode::Down => self.cursor.1 += 1,
                    KeyCode::Char('e') => self.edit_cell()?,
                    _ => {}
                }
                self.adjust_offset();
            }
        }

        execute!(stdout, terminal::LeaveAlternateScreen, ResetColor)?;
        terminal::disable_raw_mode()
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(stdout, terminal::Clear(ClearType::All))?;

        for row in 0..VIEW_HEIGHT {
            for col in 0..VIEW_WIDTH {
                let x = self.offset.0 + col as i32;
                let y = self.offset.1 + row as i32;
                let val = self
                    .cells
                    .get(&(x, y))
                    .cloned()
                    .unwrap_or_else(|| " ".to_string());
                let is_cursor = self.cursor == (x, y);
                let display = if is_cursor {
                    format!(">{val: <7}")
                } else {
                    format!(" {val: <7}")
                };
                execute!(
                    stdout,
                    cursor::MoveTo((col * 8) as u16, row as u16),
                    Print(display)
                )?;
            }
        }

        execute!(
            stdout,
            cursor::MoveTo(0, VIEW_HEIGHT as u16 + 1),
            Print(format!("Cursor: {:?}  Offset: {:?}", self.cursor, self.offset))
        )?;
        stdout.flush()
    }

    fn edit_cell(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        terminal::disable_raw_mode()?;

        execute!(
            stdout,
            cursor::MoveTo(0, VIEW_HEIGHT as u16 + 2),
            terminal::Clear(ClearType::FromCursorDown),
            Print("Enter value: ")
        )?;
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        self.cells.insert(self.cursor, input.trim().to_string());

        terminal::enable_raw_mode()
    }

    fn adjust_offset(&mut self) {
        let (cx, cy) = self.cursor;
        let (ox, oy) = self.offset;

        if cx < ox {
            self.offset.0 = cx;
        } else if cx >= ox + VIEW_WIDTH as i32 {
            self.offset.0 = cx - VIEW_WIDTH as i32 + 1;
        }

        if cy < oy {
            self.offset.1 = cy;
        } else if cy >= oy + VIEW_HEIGHT as i32 {
            self.offset.1 = cy - VIEW_HEIGHT as i32 + 1;
        }
    }
}

fn main() -> io::Result<()> {
    Spreadsheet::new().run()
}
