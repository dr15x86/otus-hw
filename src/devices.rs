pub mod socket;
pub mod thermometer;

use crate::{custom_reporter::Accept, error::ResultStr};

pub trait Device: Accept {
    fn info(&self) -> ResultStr<String>;
}
