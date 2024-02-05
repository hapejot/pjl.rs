use std::{cell::RefCell, fmt, rc::Rc};

pub struct Format {
    out: Rc<RefCell<dyn fmt::Write>>,
}

impl Format {
    pub fn new(out: Rc<RefCell<dyn fmt::Write>>) -> Self {
        Format { out }
    }

    pub fn convert(&self, atom: &crate::value::Value) -> Result<(), std::fmt::Error> {
        match atom {
            crate::value::Value::PrimitiveValue(v) => {
                let mut w = self.out.borrow_mut();
                write!(w, "\"{}\"", v)
            }
            crate::value::Value::StructureValue(v) => self.convert_structure(v),
            crate::value::Value::ListValue(v) => self.convert_list(v),
        }
    }

    fn convert_structure(&self, v: &crate::structure::StructureValue) -> Result<(), fmt::Error> {
        {
            let mut w = self.out.borrow_mut();
            write!(w, "{{\"__metadata\":{{\"type\":\"{}\"}}", v.datatype())?;
        }
        let mut sep = ",";
        for x in v.keys() {
            {
                let mut w = self.out.borrow_mut();
                write!(w, "{sep}\"{}\":", x)?;
            }
            self.convert(&v[x])?;
            sep = ",";
        }
        {
            let mut w = self.out.borrow_mut();
            if sep == "{" {
                write!(w, "{sep}")?;
            }
            write!(w, "}}")?;
        }
        Ok(())
    }

    fn convert_list(&self, v: &crate::list::ListValue) -> Result<(), fmt::Error> {
        let mut sep = "[";
        for x in v.iter() {
            {
                let mut w = self.out.borrow_mut();
                write!(w, "{sep}")?;
            }
            self.convert(&x)?;
            sep = ",";
        }
        {
            let mut w = self.out.borrow_mut();
            if sep == "[" {
                write!(w, "{sep}")?;
            }
            write!(w, "]")?;
        }
        Ok(())
    }
}
