use otus_hw::{
    builder::Builder,
    custom_reporter::{html::HtmlReporter, json::JsonReporter, Accept},
    devices::socket::SocketState,
    error::ResultStr,
};

fn main() -> ResultStr<()> {
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

    let writer = &mut std::io::stdout();

    println!("JsonReporter:");
    let mut json_reporter = JsonReporter::new(writer);
    house.accept(&mut json_reporter).unwrap();
    json_reporter.finish().unwrap();
    println!("------------------------------\n");

    println!("HtmlReporter:");
    let mut html_reporter = HtmlReporter::new(writer);
    house.accept(&mut html_reporter).unwrap();
    html_reporter.finish().unwrap();
    println!("------------------------------\n");

    Ok(())
}
