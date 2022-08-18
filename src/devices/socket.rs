use crate::{
    custom_reporter::{Accept, Reporter},
    devices::Device,
    error::{Result, ResultStr},
};

pub struct Socket {
    state: SocketState,
    power: f32,
}

#[derive(Copy, Clone)]
pub enum SocketState {
    On,
    Off,
}

impl Default for Socket {
    fn default() -> Self {
        Self {
            state: SocketState::Off,
            power: 0.0,
        }
    }
}

impl Socket {
    pub fn new(state: SocketState, power: f32) -> Self {
        Self { state, power }
    }

    pub fn state(&self) -> SocketState {
        self.state
    }

    pub fn set_state(&mut self, state: SocketState) {
        self.state = state
    }

    pub fn power(&self) -> f32 {
        self.power
    }

    pub fn set_power(&mut self, power: f32) {
        self.power = power
    }
}

impl Device for Socket {
    fn info(&self) -> ResultStr<String> {
        let state = match self.state {
            SocketState::On => "on",
            SocketState::Off => "off",
        };

        Ok(format!("state: {}, power: {}", state, self.power))
    }
}

impl Accept for Socket {
    fn accept(&self, visitor: &mut dyn Reporter) -> Result<()> {
        visitor.element_type("socket".into())?;
        visitor.element_attr(
            "state".into(),
            match self.state {
                SocketState::On => "on".into(),
                SocketState::Off => "off".into(),
            },
        )?;
        visitor.element_attr("power".into(), self.power.to_string())?;
        Ok(())
    }
}
