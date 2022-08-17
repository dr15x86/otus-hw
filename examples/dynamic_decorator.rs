use otus_hw::devices::{
    socket::{Socket, SocketState},
    thermometer::Thermometer,
    Device,
};

struct Logger {
    tag: &'static str,
    device: Box<dyn Device>,
}

impl Logger {
    fn new(device: Box<dyn Device>, tag: &'static str) -> Self {
        Self { tag, device }
    }
}

impl Device for Logger {
    fn info(&self) -> Result<String, &'static str> {
        println!("Before call `info` for {}", self.tag);
        self.device.info()
    }
}

fn print_info(device: &dyn Device) {
    println!("{}\n", device.info().unwrap());
}

fn main() {
    let real_socket = Socket::new(SocketState::Off, 120.0);
    let real_thermometer = Thermometer::new(25.0);
    let wrapped_socket = Logger::new(
        Box::new(Socket::new(SocketState::On, 220.0)),
        "wrapped socket",
    );
    let wrapped_thermometer = Logger::new(Box::new(Thermometer::new(28.0)), "wrapped thermometer");

    print_info(&real_socket);
    print_info(&real_thermometer);
    print_info(&wrapped_socket);
    print_info(&wrapped_thermometer);
}
