use crate::devices::Device;

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
    fn info(&self) -> Result<String, &'static str> {
        let state = match self.state {
            SocketState::On => "on",
            SocketState::Off => "off",
        };

        Ok(format!("state: {}, power: {}", state, self.power))
    }
}
