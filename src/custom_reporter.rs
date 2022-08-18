pub mod html;
pub mod json;

use crate::error::Result;

pub trait Reporter {
    fn start_element(&mut self, elem_name: String) -> Result<()>;
    fn element_type(&mut self, elem_type: String) -> Result<()>;
    fn element_attr(&mut self, attr_name: String, attr_value: String) -> Result<()>;
    fn end_element(&mut self) -> Result<()>;
}

pub trait Accept {
    fn accept(&self, visitor: &mut dyn Reporter) -> Result<()>;
}
