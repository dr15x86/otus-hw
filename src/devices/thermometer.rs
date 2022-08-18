use crate::{
    custom_reporter::{Accept, Reporter},
    devices::Device,
    error::{Result, ResultStr},
};

pub struct Thermometer {
    temperature: f32,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self { temperature: 0.0 }
    }
}

impl Thermometer {
    pub fn new(temperature: f32) -> Self {
        Self { temperature }
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature
    }
}

impl Device for Thermometer {
    fn info(&self) -> ResultStr<String> {
        Ok(format!("temperature: {}", self.temperature()))
    }
}

impl Accept for Thermometer {
    fn accept(&self, visitor: &mut dyn Reporter) -> Result<()> {
        visitor.element_type("thermometer".into())?;
        visitor.element_attr("temperature".into(), self.temperature.to_string())?;
        Ok(())
    }
}
