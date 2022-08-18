use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Write,
};

use crate::{
    custom_reporter::{Accept, Reporter},
    devices::Device,
    error::{Result, ResultStr},
};

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

    pub fn add_room(&mut self, room: Room) -> ResultStr<()> {
        return match self.rooms.entry(room.name().into()) {
            Entry::Occupied(_) => Err("Room with this name already exists"),
            Entry::Vacant(v) => {
                v.insert(room);
                Ok(())
            }
        };
    }

    pub fn remove_room(&mut self, room_name: &str) -> ResultStr<()> {
        match self.rooms.remove(room_name) {
            Some(_) => Ok(()),
            None => Err("Room with this name not exists"),
        }
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

    pub fn create_report(&self) -> ResultStr<String> {
        let mut result = format!("home: {}\n", self.name);

        for r in self.room_names() {
            let _ = writeln!(result, "    room: {}", r);

            let cur_room = self.get_room(r).unwrap();

            for d in cur_room.device_names() {
                let _ = writeln!(result, "        device: {}", d);
                let _ = writeln!(
                    result,
                    "            {}",
                    &cur_room.get_device(d).unwrap().info()?
                );
            }
        }

        Ok(result)
    }
}

impl Accept for House {
    fn accept(&self, visitor: &mut dyn Reporter) -> Result<()> {
        visitor.start_element(self.name.clone())?;
        visitor.element_type("house".into())?;

        for name in self.room_names() {
            visitor.start_element(name.into())?;
            self.get_room(name).unwrap().accept(visitor)?;
            visitor.end_element()?;
        }
        visitor.end_element()?;

        Ok(())
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

    pub fn add_device(&mut self, device_name: String, device: Box<dyn Device>) -> ResultStr<()> {
        return match self.devices.entry(device_name) {
            Entry::Occupied(_) => Err("Device with this name already exists"),
            Entry::Vacant(v) => {
                v.insert(device);
                Ok(())
            }
        };
    }

    pub fn remove_device(&mut self, device_name: &str) -> ResultStr<()> {
        match self.devices.remove(device_name) {
            Some(_) => Ok(()),
            None => Err("Device with this name not exists"),
        }
    }

    pub fn get_device(&self, device_name: &str) -> Option<&dyn Device> {
        self.devices.get(device_name).map(|v| v.as_ref())
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

impl Accept for Room {
    fn accept(&self, visitor: &mut dyn Reporter) -> Result<()> {
        visitor.element_type("room".into())?;

        for name in self.device_names() {
            visitor.start_element(name.into())?;
            self.get_device(name).unwrap().accept(visitor)?;
            visitor.end_element()?;
        }
        Ok(())
    }
}
