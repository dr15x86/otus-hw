use std::collections::{hash_map::Entry, HashMap};

use crate::devices::Device;

pub struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), &'static str> {
        return match self.rooms.entry(room.name().into()) {
            Entry::Occupied(_) => Err("Room with this name already exists"),
            Entry::Vacant(v) => {
                v.insert(room);
                Ok(())
            }
        };
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        self.rooms.get(room_name)
    }

    pub fn room_names(&self) -> Vec<&str> {
        let mut result = Vec::with_capacity(self.rooms.len());
        for k in self.rooms.keys() {
            result.push(k.as_str());
        }

        result.sort_unstable();

        result
    }

    pub fn create_report(&self) -> Result<String, &'static str> {
        let mut result = format!("home: {}\n", self.name);

        for r in self.room_names() {
            result.push_str(&format!("    room: {}\n", r));

            let cur_room = self.get_room(r).unwrap();

            for d in cur_room.device_names() {
                result.push_str(&format!("        device: {}\n", d));
                result.push_str(&format!(
                    "            {}\n",
                    &cur_room.get_device(d).unwrap().info()?
                ));
            }
        }

        Ok(result)
    }
}

pub struct Room {
    name: String,
    devices: HashMap<String, Box<dyn Device>>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_device(
        &mut self,
        device_name: String,
        device: Box<dyn Device>,
    ) -> Result<(), &'static str> {
        return match self.devices.entry(device_name) {
            Entry::Occupied(_) => Err("Device with this name already exists"),
            Entry::Vacant(v) => {
                v.insert(device);
                Ok(())
            }
        };
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Box<dyn Device>> {
        self.devices.get(device_name)
    }

    pub fn device_names(&self) -> Vec<&str> {
        let mut result = Vec::with_capacity(self.devices.len());
        for k in self.devices.keys() {
            result.push(k.as_str());
        }

        result.sort_unstable();

        result
    }
}
