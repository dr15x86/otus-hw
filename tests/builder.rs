use otus_hw::{
    builder::Builder,
    devices::{
        socket::{Socket, SocketState},
        thermometer::Thermometer,
    },
    house_dyn::{House, Result, Room},
};

#[test]
fn builder() -> Result<()> {
    let house = Builder::new("test house".into())
        .add_room("room 1".into())
        // ------------------------
        .add_thermometer("thermometer 1".into())
        .set_temperature(25.0)
        .set_temperature(23.0)
        .build_thermometer()?
        // ------------------------
        .add_thermometer("thermometer 2".into())
        .build_thermometer()?
        // ------------------------
        .add_socket("socket 1".into())
        .set_state(SocketState::On)
        .set_power(120.0)
        .build_socket()?
        // ------------------------
        .build_room()?
        // ------------------------
        .add_room("room 2".into())
        .add_socket("socket 1-2".into())
        .build_socket()?
        // ------------------------
        .build_room()?
        .build_house();

    let mut house_expected = House::new("test house".into());
    let mut room1 = Room::new("room 1".into());
    room1.add_device("thermometer 1".into(), Box::new(Thermometer::new(23.0)))?;
    room1.add_device("thermometer 2".into(), Box::new(Thermometer::default()))?;
    room1.add_device(
        "socket 1".into(),
        Box::new(Socket::new(SocketState::On, 120.0)),
    )?;
    house_expected.add_room(room1)?;
    
    let mut room2 = Room::new("room 2".into());
    room2.add_device(
        "socket 1-2".into(),
        Box::new(Socket::default()),
    )?;
    house_expected.add_room(room2)?;

    assert_eq!(house.create_report()?, house_expected.create_report()?);

    Ok(())
}
