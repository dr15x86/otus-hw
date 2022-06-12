use otus_hw::devices::{socket::Socket, socket::SocketState, thermometer::Thermometer, Device};
use otus_hw::house_stat::{DeviceInfoProvider, House, Room};

#[test]
fn stat_house_new() {
    let house = House::new("test house".into());

    assert_eq!(house.name(), "test house");
    assert_eq!(house.room_names(), [] as [&str; 0]);
}

#[test]
fn stat_house_add_get_rooms() {
    let mut house = House::new("test house".into());

    assert!(house.add_room(Room::new("room 1".into())).is_ok());
    assert!(house.add_room(Room::new("room 2".into())).is_ok());
    assert!(house.add_room(Room::new("room 1".into())).is_err());

    assert_eq!(house.room_names(), ["room 1", "room 2"]);

    assert_eq!(house.get_room("room 2").unwrap().name(), "room 2");
}

#[test]
fn stat_room_add_devices() {
    let mut room = Room::new("room 1".into());

    assert!(room.add_device("thermometer 1".into()).is_ok());
    assert!(room.add_device("thermometer 2".into()).is_ok());
    assert!(room.add_device("thermometer 1".into()).is_err());

    assert_eq!(room.device_names(), ["thermometer 1", "thermometer 2"]);
}

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
    fn get_device_description(&self, room: &str, name: &str) -> Result<String, &'static str> {
        for si in &self.sockets {
            if si.room == room && si.name == name {
                return Ok(si.device.info()?);
            }
        }

        for ti in &self.thermometers {
            if ti.room == room && ti.name == name {
                return Ok(ti.device.info()?);
            }
        }

        Err("Device with specified room and name was not found")
    }
}

#[test]
fn stat_house_create_report() {
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
    assert!(room1.add_device("sock 1-1".into()).is_ok());
    assert!(room1.add_device("therm 1-1".into()).is_ok());
    assert!(house.add_room(room1).is_ok());

    let mut room2 = Room::new("room 2".into());
    assert!(room2.add_device("sock 2-1".into()).is_ok());
    assert!(room2.add_device("sock 2-2".into()).is_ok());
    assert!(room2.add_device("therm 2-1".into()).is_ok());
    assert!(house.add_room(room2).is_ok());

    let report = house.create_report(&holder).unwrap();
    assert!(report.find("device: sock 2-2").is_some());
    assert!(report.find("temperature: 27").is_some());
}
