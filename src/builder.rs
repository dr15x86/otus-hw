use crate::{
    devices::{socket::Socket, socket::SocketState, thermometer::Thermometer},
    error::ResultStr,
    house_dyn::{House, Room},
};

pub struct Builder {
    house: House,
}

impl Builder {
    pub fn new(house_name: String) -> Self {
        Self {
            house: House::new(house_name),
        }
    }

    pub fn add_room(self, room_name: String) -> RoomBuilder {
        RoomBuilder::new(room_name, self.house)
    }

    pub fn build_house(self) -> House {
        self.house
    }

    fn restore(house: House) -> Self {
        Self { house }
    }
}

pub struct RoomBuilder {
    room: Room,
    house: House,
}

impl RoomBuilder {
    pub fn add_socket(self, socket_name: String) -> SocketBuilder {
        SocketBuilder::new(socket_name, self.room, self.house)
    }

    pub fn add_thermometer(self, thermometer_name: String) -> ThermometerBuilder {
        ThermometerBuilder::new(thermometer_name, self.room, self.house)
    }

    pub fn build_room(mut self) -> ResultStr<Builder> {
        self.house.add_room(self.room)?;
        Ok(Builder::restore(self.house))
    }

    fn new(room_name: String, house: House) -> Self {
        Self {
            room: Room::new(room_name),
            house,
        }
    }

    fn restore(room: Room, house: House) -> Self {
        Self { room, house }
    }
}

pub struct SocketBuilder {
    socket_name: String,
    socket: Socket,
    room: Room,
    house: House,
}

impl SocketBuilder {
    pub fn set_state(mut self, state: SocketState) -> SocketBuilder {
        self.socket.set_state(state);
        self
    }

    pub fn set_power(mut self, power: f32) -> SocketBuilder {
        self.socket.set_power(power);
        self
    }

    pub fn build_socket(mut self) -> ResultStr<RoomBuilder> {
        self.room
            .add_device(self.socket_name, Box::new(self.socket))?;
        Ok(RoomBuilder::restore(self.room, self.house))
    }

    fn new(socket_name: String, room: Room, house: House) -> Self {
        Self {
            socket_name,
            socket: Default::default(),
            room,
            house,
        }
    }
}

pub struct ThermometerBuilder {
    thermometer_name: String,
    thermometer: Thermometer,
    room: Room,
    house: House,
}

impl ThermometerBuilder {
    pub fn set_temperature(mut self, temperature: f32) -> ThermometerBuilder {
        self.thermometer.set_temperature(temperature);
        self
    }

    pub fn build_thermometer(mut self) -> ResultStr<RoomBuilder> {
        self.room
            .add_device(self.thermometer_name, Box::new(self.thermometer))?;
        Ok(RoomBuilder::restore(self.room, self.house))
    }

    fn new(thermometer_name: String, room: Room, house: House) -> Self {
        Self {
            thermometer_name,
            thermometer: Default::default(),
            room,
            house,
        }
    }
}
