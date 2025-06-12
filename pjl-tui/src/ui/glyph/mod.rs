use lazy_static::lazy_static;
use std::io::Write;
use tracing::info;

pub mod frame;
pub mod input;
pub mod label;
pub mod panel;
pub mod tedit;
pub mod fgrid; // flexible grid

#[derive(Debug, Clone)]
pub struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}
impl Rect {
    pub fn new(w: u16, h: u16) -> Rect {
        Rect { x: 0, y: 0, w, h }
    }

    pub fn contains(&self, x: u16, y: u16) -> bool {
        self.x <= x && self.y <= y && x < (self.x + self.w) && y < (self.y + self.h)
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, Clone)]
pub enum Requirement {
    Chars(u16),
    Max,
}
impl Requirement {
    fn max(&self, col: u16) -> u16 {
        match self {
            Requirement::Chars(n) => if col > *n {col} else {*n},
            Requirement::Max => col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Requirements {
    pub w: Requirement,
    pub h: Requirement,
}

#[derive(Debug, Clone)]
pub enum AppRequest {
    /// Dummy request to see what happens
    None,
    /// setting a value for a named element
    SetValue { name: String, value: String },
    /// getting a value for a named element somewhere in the tree
    GetValue(String),
    /// go to the next input field given the current cursor position
    NextInput(u16, u16),
    /// collect all values for named labels
    CollectAllValues,
    /// Handle Enter key
    HandleEnterKey,
}

#[derive(Debug, Clone)]
pub enum AppResult {
    StringValue(String),
    Redraw,
    RedrawGlyph,
    InputEnabled,
    NewCursorPosition(u16, u16),
    Values(Vec<(String, String)>),
    Nothing,
}

#[derive(Debug, Clone)]
pub enum AppError {
    NotRelevant,
    InvalidRequest,
}

pub type AppResponse = Result<Vec<AppResult>, AppError>;

#[derive(Debug,Clone)]
pub enum TermEvent{
    None,
    Key(char),
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    GoToStart,
    GoToEnd,
    BackSpace,
}


pub trait Glyph: std::fmt::Debug {
    fn id(&self) -> u16;
    fn hit(&mut self, x: u16, y: u16) -> AppResponse;
    fn write_to(&self, w: &mut dyn Write);
    fn area(&self) -> Rect;
    fn resize(&mut self, width: u16, height: u16);
    fn handle_term_event(&mut self, r: TermEvent) -> AppResponse;
    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse;
    fn request(&self) -> Requirements;
    fn allocate(&mut self, allocation: Rect);
    fn allocated(&self) -> bool;
    fn name(&self) -> String;
}
lazy_static! {
    static ref nr: std::sync::Mutex<u16> = std::sync::Mutex::new(0);
}

pub fn next_glyph_number() -> u16 {
    let mut x = nr.lock().unwrap();
    *x += 1;
    info!("next glyph: {}", *x);
    *x
}
