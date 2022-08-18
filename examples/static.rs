use otus_hw::{
    devices::{socket::Socket, socket::SocketState, thermometer::Thermometer, Device},
    error::ResultStr,
    house_stat::{DeviceInfoProvider, House, Room},
};

struct HolderItem<T: Device> {
    device: T,
    room: String,
    name: String,
}

struct DevicesSimpleHolder {
    sockets: Vec<HolderItem<Socket>>,
    thermometers: Vec<HolderItem<Thermometer>>,
}

impl DeviceInfoProvider for DevicesSimpleHolder {
    fn get_device_description(&self, room: &str, name: &str) -> ResultStr<String> {
        for si in &self.sockets {
            if si.room == room && si.name == name {
                return si.device.info();
            }
        }

        for ti in &self.thermometers {
            if ti.room == room && ti.name == name {
                return ti.device.info();
            }
        }

        Err("Device with specified room and name was not found")
    }
}

fn main() {
    let holder = DevicesSimpleHolder {
        sockets: vec![
            HolderItem {
                device: Socket::new(SocketState::On, 1000.0),
                room: "room 1".into(),
                name: "sock 1-1".into(),
            },
            HolderItem {
                device: Socket::new(SocketState::Off, 0.0),
                room: "room 2".into(),
                name: "sock 2-1".into(),
            },
            HolderItem {
                device: Socket::new(SocketState::On, 200.0),
                room: "room 2".into(),
                name: "sock 2-2".into(),
            },
        ],
        thermometers: vec![
            HolderItem {
                device: Thermometer::new(25.0),
                room: "room 1".into(),
                name: "therm 1-1".into(),
            },
            HolderItem {
                device: Thermometer::new(27.0),
                room: "room 2".into(),
                name: "therm 2-1".into(),
            },
        ],
    };

    let mut house = House::new("house 1".into());

    let mut room1 = Room::new("room 1".into());
    room1.add_device("sock 1-1".into()).unwrap();
    room1.add_device("therm 1-1".into()).unwrap();
    house.add_room(room1).unwrap();

    let mut room2 = Room::new("room 2".into());
    room2.add_device("sock 2-1".into()).unwrap();
    room2.add_device("sock 2-2".into()).unwrap();
    room2.add_device("therm 2-1".into()).unwrap();
    house.add_room(room2).unwrap();

    let report = house.create_report(&holder).unwrap();
    print!("Report:\n{}", report);
}
