use std::{fs, io};

use crossterm::{execute, style};
use pjl_tab::Table;
use pjl_terminal_tools::{
    App, Render,
    grid::{Grid, GridModel},
    read_table_from_stream,
};

struct TableGrid {
    t: Table,
}

impl GridModel for TableGrid {
    fn width(&self) -> usize {
        self.t.column_count()
    }

    fn height(&self) -> usize {
        self.t.lines()
    }

    fn value(&self, x: usize, y: usize) -> String {
        match self.t.get(y + 1, x + 1) {
            Some(v) => v,
            None => String::new(),
        }
    }

    fn set_value(&mut self, x: usize, y: usize, value: &str) {
        todo!()
    }
}

fn main() {
    let f = fs::File::open(std::env::args().nth(1).unwrap()).unwrap();
    let t = read_table_from_stream(f);
    // dump_table_to_console(&t);
    let mut app = App::new(0);
    app.add_view(Box::new(Grid::new(Box::new(TableGrid { t }))));
    app.run().unwrap();
}
