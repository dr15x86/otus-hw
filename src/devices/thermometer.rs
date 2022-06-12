use crate::devices::Device;

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
    fn info(&self) -> Result<String, &'static str> {
        Ok(format!("temperature: {}", self.temperature()))
    }
}
