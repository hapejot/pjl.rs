use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};

use tracing::*;

use super::{AppRequest, AppResponse, Glyph, Rect, Requirements, TermEvent};

#[derive(Debug, Clone)]
pub struct Sparse<T> {
    data: Vec<(u16, T)>,
    default: T,
}

impl<T> Sparse<T> {
    pub fn new(default: T) -> Self {
        let data: Vec<(u16, T)> = vec![];
        Self { data, default }
    }
}

impl<T> std::ops::Index<u16> for Sparse<T> {
    type Output = T;

    fn index(&self, index: u16) -> &Self::Output {
        let mut r = &self.default;
        for (k, v) in self.data.iter() {
            if *k == index {
                r = v;
                break;
            }
        }
        &r
    }
}

impl<T> std::ops::IndexMut<u16> for Sparse<T> where T: Clone {
    
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let ndx = match self.data.iter().position(|(k, _)| *k == index) {
            Some(n) => n,
            None => {
                self.data.push((index, self.default.clone()));
                self.data.len() - 1
            }
        };
        let (k, v) = self.data.get_mut(ndx).unwrap();
        v
    }
}

#[derive(Debug)]
pub struct FGrid {
    dimensions: (usize, usize),
    col_heads: Vec<String>,
    row_heads: Vec<String>,
    cells: HashMap<(u16, u16), Box<dyn Glyph>>,
}

impl FGrid {
    pub fn new() -> Self {
        let dimensions = (2, 2);
        let col_heads = vec![];
        let row_heads = vec![];
        let cells = HashMap::new();
        Self {
            dimensions,
            col_heads,
            row_heads,
            cells,
        }
    }

    pub fn set_at(&mut self, row: u16, col: u16, txt: Box<dyn Glyph>) {
        self.cells.insert((row, col), txt);
    }
}

impl Glyph for FGrid {
    fn id(&self) -> u16 {
        todo!()
    }

    fn hit(&mut self, x: u16, y: u16) -> AppResponse {
        let _ = (x, y);
        todo!()
    }

    fn write_to(&self, w: &mut dyn Write) {
        let _ = (&self.col_heads, &self.dimensions, &self.row_heads);

        for x in self.cells.values() {
            x.write_to(w);
        }
    }

    fn area(&self) -> Rect {
        todo!()
    }

    fn resize(&mut self, width: u16, height: u16) {
        let _ = (width, height);
        todo!()
    }

    fn handle_term_event(&mut self, r: TermEvent) -> AppResponse {
        for x in self.cells.values_mut() {
            let res = x.handle_term_event(r.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err(super::AppError::InvalidRequest)
    }

    fn handle_app_request(&mut self, req: &AppRequest) -> AppResponse {
        let _ = req;
        todo!()
    }

    fn request(&self) -> Requirements {
        todo!()
    }

    fn allocate(&mut self, allocation: Rect) {
        let _ = allocation;
        let mut max_col = 0;
        let mut max_row = 0;
        let mut widths = Sparse::<u16>::new(0);

        let mut rows = Sparse::<Sparse<Option<Requirements>>>::new(Sparse::new(None));

        for ((row, col), x) in self.cells.iter() {
            if max_col < *col {
                max_col = *col;
            }
            if max_row < *row {
                max_row = *row;
            }
            let req = x.request();
            widths[*col] = req.w.max(widths[*col]);
            rows[*row][*col] = Some(req);
            // row_data.insert(*col, req);
        }

        debug!("rows: {:#?} widths: {:?}", rows, widths);
    }

    fn allocated(&self) -> bool {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}
