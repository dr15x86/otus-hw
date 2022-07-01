use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fmt::Write as _;

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

    pub fn create_report(&self, info: &impl DeviceInfoProvider) -> Result<String, &'static str> {
        let mut result = format!("home: {}\n", self.name);

        for r in self.room_names() {
            let _ = writeln!(result, "    room: {}", r);

            let cur_room = self.get_room(r).ok_or("Room with this name not exists")?;

            for d in cur_room.device_names() {
                let _ = writeln!(result, "        device: {}", d);
                let _ = writeln!(result, "            {}", info.get_device_description(r, d)?);
            }
        }

        Ok(result)
    }
}

pub struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: HashSet::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device_name: String) -> Result<(), &'static str> {
        if self.devices.insert(device_name) {
            Ok(())
        } else {
            Err("Device with this name already exists")
        }
    }

    pub fn device_names(&self) -> Vec<&str> {
        let mut result = Vec::with_capacity(self.devices.len());
        for d in &self.devices {
            result.push(d.as_str());
        }

        result.sort_unstable();

        result
    }
}

pub trait DeviceInfoProvider {
    fn get_device_description(&self, room: &str, name: &str) -> Result<String, &'static str>;
}
