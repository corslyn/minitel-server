use std::io::Read;

use serialport::SerialPort;

use crate::{
    modem::{self, *},
    page::zone::*,
};

pub mod zone;
pub struct Page {
    pub name: String,
    pub vdt_file: String,
    pub zones: Vec<Zone>,
}

impl Page {
    pub fn new(name: &str, vdt_file: &str) -> Page {
        Page {
            name: name.to_string(),
            vdt_file: vdt_file.to_string(),
            zones: Vec::new(),
        }
    }
    pub fn next_zone(&self, current_zone: u8) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == current_zone + 1)
    }

    /// retourne l'id de la zone courante
    pub fn current_zone(&self, id: u8) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == id)
    }

    pub fn previous_zone(&self, current_zone: u8) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == current_zone - 1)
    }

    pub fn send(&self, modem: &mut Box<dyn SerialPort>) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Envoi de la page: {}", self.name);
        modem.write_all(&self.get_vdt_file().unwrap())?;
        modem.flush()?;
        Ok(())
    }

    fn get_vdt_file(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(&self.vdt_file)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }
}
