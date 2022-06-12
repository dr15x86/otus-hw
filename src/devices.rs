pub mod socket;
pub mod thermometer;

pub trait Device {
    fn info(&self) -> Result<String, &'static str>;
}
