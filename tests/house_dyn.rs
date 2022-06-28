use otus_hw::devices::{socket::Socket, socket::SocketState, thermometer::Thermometer};
use otus_hw::house_dyn::{House, Room};

#[test]
fn dyn_house_new() {
    let house = House::new("test house".into());

    assert_eq!(house.name(), "test house");
    assert_eq!(house.room_names(), [] as [&str; 0]);
}

#[test]
fn dyn_house_modify_rooms() {
    let mut house = House::new("test house".into());

    assert!(house.add_room(Room::new("room 1".into())).is_ok());
    assert!(house.add_room(Room::new("room 2".into())).is_ok());
    assert!(house.add_room(Room::new("room 1".into())).is_err());
    assert!(house.remove_room("room 1").is_ok());
    assert!(house.remove_room("room 1").is_err());
    assert!(house.add_room(Room::new("room 1".into())).is_ok());

    assert_eq!(house.room_names(), ["room 1", "room 2"]);

    assert_eq!(house.get_room("room 2").unwrap().name(), "room 2");
}

#[test]
fn dyn_room_modify_devices() {
    let mut room = Room::new("room 1".into());

    assert!(room
        .add_device("thermometer 1".into(), Box::new(Thermometer::default()))
        .is_ok());

    assert!(room
        .add_device("thermometer 2".into(), Box::new(Thermometer::new(15.0)))
        .is_ok());

    assert!(room
        .add_device("thermometer 1".into(), Box::new(Thermometer::default()))
        .is_err());

    assert!(room.remove_device("thermometer 1").is_ok());
    assert!(room.remove_device("thermometer 1").is_err());

    assert!(room
        .add_device("thermometer 1".into(), Box::new(Thermometer::default()))
        .is_ok());

    assert_eq!(room.device_names(), ["thermometer 1", "thermometer 2"]);

    assert!(room
        .get_device("thermometer 2")
        .unwrap()
        .info()
        .unwrap()
        .contains("15"));
}

#[test]
fn dyn_house_create_report() {
    let mut house = House::new("house 1".into());

    let mut room1 = Room::new("room 1".into());

    assert!(room1
        .add_device("therm 1-1".into(), Box::new(Thermometer::new(25.0)))
        .is_ok());

    assert!(room1
        .add_device(
            "sock 1-1".into(),
            Box::new(Socket::new(SocketState::On, 1000.0))
        )
        .is_ok());

    assert!(house.add_room(room1).is_ok());

    let mut room2 = Room::new("room 2".into());

    assert!(room2
        .add_device("therm 2-1".into(), Box::new(Thermometer::new(27.0)))
        .is_ok());

    assert!(room2
        .add_device(
            "sock 2-1".into(),
            Box::new(Socket::new(SocketState::Off, 0.0))
        )
        .is_ok());

    assert!(room2
        .add_device(
            "sock 2-2".into(),
            Box::new(Socket::new(SocketState::On, 200.0))
        )
        .is_ok());

    assert!(house.add_room(room2).is_ok());

    let report = house.create_report().unwrap();

    assert!(report.contains("device: sock 2-2"));
    assert!(report.contains("temperature: 27"));
}
