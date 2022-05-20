struct _Socket {
    state: _SocketState,
    power: f32,
}

enum _SocketState {
    On,
    Off,
}

impl _Socket {
    fn _to_string(&self) -> String {
        todo!();
    }

    fn _state(&self) -> _SocketState {
        todo!();
    }

    fn _set_state(&mut self, _new_state: _SocketState) {
        todo!();
    }

    fn _power(&self) -> f32 {
        self.power
    }
}

struct _Thermometer {
    _temperature: f32,
}

impl _Thermometer {
    fn _to_string(&self) -> String {
        todo!();
    }

    fn _temperature(&self) -> f32 {
        self._temperature
    }
}
