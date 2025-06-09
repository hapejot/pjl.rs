use std::io;

use crossterm::{cursor, execute, style, terminal};

use crate::Render;

pub trait GridModel {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn value(&self, x: usize, y: usize) -> String;
    fn set_value(&mut self, x: usize, y: usize, value: &str);
}

pub struct Grid {
    model: Box<dyn GridModel>,
}

impl Grid {
    pub fn new(model: Box<dyn GridModel>) -> Self {
        Self { model }
    }
}

type Position = (i32,i32);


impl Render for Grid {
    fn render(&self) -> std::io::Result<()> {
        let mut stdout = io::stdout();
        let (columns, rows) = terminal::size()?;
        let (max_col, max_row) = (self.model.width(), self.model.height());
        for row in 1..(rows - 1) {
            
            if (row - 1) as usize >= max_row {
                break;
            }
            let mut col = 1;
            let mut idx = 0;
            while col < columns && idx < max_col {
                execute!(
                    stdout,
                    cursor::MoveTo(col, row),
                    style::Print(format!(">{}<", self.model.value((row-1) as usize, idx)))
                )?;
                col += 10;
                idx += 1;
            }
        }
        Ok(())
    }
}
