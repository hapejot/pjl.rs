mod action;
mod actionmap;
mod settings;
mod state;
mod ui;

fn main() {
    let mut tui = ui::Application::init();
    match tui.run() {
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {e}"),
    }
}
