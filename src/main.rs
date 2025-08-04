use crate::{modem::*, page::*};
use log;
use serialport::{self, SerialPort};
use std::error::Error;

use crate::page::zone::Zone;

mod modem;
mod page;

fn main() -> Result<(), Box<dyn Error>> {
    log::info!("Démarrage du serveur Minitel...");

    // a faire: mise en place d'un flag pour ne pas init le modem si on utilise un minitel retourné
    let mut modem = init_modem("/dev/ttyUSB0", None)?;

    handle_connection(&mut modem);

    main_loop(modem);

    Ok(())
}

fn main_loop(mut modem: Box<dyn SerialPort>) {
    log::info!("En attente de la fin de connexion...");
    let mut pages: Vec<Page> = Vec::new();
    let mut teletel = Page::new("teletel", "teletel.vdt");
    Zone::add(&mut teletel, 1, 11, 17, 40 - 11);
    pages.push(teletel);
    let mut current_page = &pages[0];
    let _ = current_page.send(&mut modem);
    loop {
        match modem.read_carrier_detect() {
            Ok(false) => {
                log::info!("Connexion interrompue ! Arrêt du serveur.");
                break;
            }
            Ok(true) => match current_page {
                page => {
                    log::info!("Connexion active sur la page: {}", page.name);
                }
            },
            Err(e) => {
                log::error!("Erreur lors de la lecture de CD: {}", e);
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
