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
                write!(w, "{}", v)
            }
            crate::value::Value::StructureValue(v) => self.convert_structure(v),
            crate::value::Value::ListValue(_) => todo!(),
        }
    }

    fn convert_structure(&self, v: &crate::structure::StructureValue) -> Result<(), fmt::Error> {
        {
            let mut w = self.out.borrow_mut();
            write!(w, "<{}>", v.datatype())?;
        }
        for x in v.keys() {
            {
                let mut w = self.out.borrow_mut();
                write!(w, "<{}>", x)?;
            }
            self.convert(&v[x])?;
            {
                let mut w = self.out.borrow_mut();
                write!(w, "</{}>", x)?;
            }
        }
        {
            let mut w = self.out.borrow_mut();
            write!(w, "</{}>", v.datatype())?;
        }
        Ok(())
    }
}
