use otus_hw::devices::{
    socket::{Socket, SocketState},
    thermometer::Thermometer,
    Device,
};

struct Logger<D: Device> {
    tag: &'static str,
    device: D,
}

impl<D: Device> Logger<D> {
    fn new(device: D, tag: &'static str) -> Self {
        Self { tag, device }
    }
}

impl<D: Device> Device for Logger<D> {
    fn info(&self) -> Result<String, &'static str> {
        println!("Before call `info` for {}", self.tag);
        self.device.info()
    }
}

fn print_info(device: impl Device) {
    println!("{}\n", device.info().unwrap());
}

fn main() {
    let real_socket = Socket::new(SocketState::Off, 120.0);
    let real_thermometer = Thermometer::new(25.0);
    let wrapped_socket = Logger::new(Socket::new(SocketState::On, 220.0), "wrapped socket");
    let wrapped_thermometer = Logger::new(Thermometer::new(28.0), "wrapped thermometer");

    print_info(real_socket);
    print_info(real_thermometer);
    print_info(wrapped_socket);
    print_info(wrapped_thermometer);
}