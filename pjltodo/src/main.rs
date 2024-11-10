use std::fs::File;

use pjltui::{Application, TApplication};
use tracing::*;

mod app;

struct App {}

impl TApplication for App {
    fn name(&self) -> &str {
        "ToDo-Liste"
    }
}

fn main() {
    let trace_file = File::create("pjltodo.trace").unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
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
