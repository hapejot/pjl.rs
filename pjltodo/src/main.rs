use std::fs::File;

use pjltui::{
    ui::glyph::{fgrid::FGrid, frame::Frame, label::Label},
    Application, Screen, TApplication,
};
use tracing::*;

// mod app;

#[derive(Debug)]
struct App {}

impl TApplication for App {
    fn name(&self) -> &str {
        "ToDo-Liste"
    }

    fn screen(&self, id: usize) -> pjltui::Screen {
        // let mut panel = Panel::new();
        let mut grid = FGrid::new();
        for (row, col, txt) in [
            (0,0, format!("Standard Text <{}>", id)),
            (0,1, format!("TODO (0)")),
            (1,0, format!("-------------")),
            (2,0,format!("Erste Aufgabe")),
            (3,0,format!("zweite Aufgabe")),
        ] {
            grid.set_at(row, col, Box::new(Label::new("l", txt)));
        }

        let frame = Frame::new(Box::new(grid));

        Screen::new(Box::new(frame))
    }
}

fn main() {
    let trace_file = File::create("pjltodo.trace").unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(true)
        .with_writer(trace_file)
        .with_ansi(false)
        .init();
    info!("starting");
    let mut app = App {};
    let mut tui = Application::new(&mut app);
    match tui.run() {
        Ok(_) => info!("exiting"),
        Err(e) => error!("error: {e}"),
    }
}
