use otus_hw::{
    devices::{socket::Socket, socket::SocketState, thermometer::Thermometer},
    house_dyn::{House, Room},
};

fn main() {
    let mut house = House::new("house 1".into());

    let mut room1 = Room::new("room 1".into());

    room1
        .add_device("therm 1-1".into(), Box::new(Thermometer::new(25.0)))
        .unwrap();

    room1
        .add_device(
            "sock 1-1".into(),
            Box::new(Socket::new(SocketState::On, 1000.0)),
        )
        .unwrap();

    house.add_room(room1).unwrap();

    let mut room2 = Room::new("room 2".into());

    room2
        .add_device("therm 2-1".into(), Box::new(Thermometer::new(27.0)))
        .unwrap();

    room2
        .add_device(
            "sock 2-1".into(),
            Box::new(Socket::new(SocketState::Off, 0.0)),
        )
        .unwrap();

    room2
        .add_device(
            "sock 2-2".into(),
            Box::new(Socket::new(SocketState::On, 200.0)),
        )
        .unwrap();

    house.add_room(room2).unwrap();

    let report = house.create_report().unwrap();
    print!("Report:\n{}", report);
}
