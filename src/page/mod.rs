use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error, Read},
};

use serialport::SerialPort;

use crate::page::zone::*;

pub mod config;
pub mod zone;

pub struct Page {
    pub name: String,
    pub vdt_file: String,
    pub zones: Vec<Zone>,
    pub routes: HashMap<String, String>,
    pub guide: Option<String>,
}

impl Page {
    pub fn new(name: &str, vdt_file: &str) -> Page {
        Page {
            name: name.to_string(),
            vdt_file: vdt_file.to_string(),
            zones: Vec::new(),
            routes: HashMap::new(),
            guide: None,
        }
    }

    pub fn load_pages_from_config(path: &str) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let configs: Vec<config::PageConfig> = serde_json::from_reader(reader)?;

        let pages: Vec<Page> = configs
            .into_iter()
            .map(|conf| Page {
                name: conf.name,
                vdt_file: conf.path,
                zones: Vec::new(),
                routes: conf.routes.unwrap_or_default(),
                guide: conf.guide,
            })
            .collect();

        Ok(pages)
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

    pub fn handle_input(&self, modem: &mut Box<dyn SerialPort>) -> Result<Option<u8>, Error> {
        let mut input = [0; 1];
        match modem.read_exact(&mut input) {
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // Ignore timeout: no input received
                Ok(None)
            }

            Ok(_) => {
                let input_char = input[0] as u8;
                log::info!("Input reçu: {}", String::from_utf8_lossy(&[input_char]));
                Ok(Some(input_char))
            }
            Err(e) => {
                log::error!("Erreur lors de la lecture de l'input: {}", e);
                Err(e)
            }
        }
    }
}
