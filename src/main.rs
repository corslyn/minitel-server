use crate::{modem::*, page::*};
use log;
use serialport::{self, SerialPort};
use std::error::Error;

use crate::page::zone::Zone;

mod modem;
mod page;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
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
    let mut teletel = Page::new("teletel", "ecrans/teletel.vdt");
    pages.push(teletel);
    let mut current_page = &pages[0];
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        let input = current_page.handle_input(&mut modem).unwrap();

        match input {
            Some(0x13) => {
                log::info!("Touche spéciale pressée");
                let input = current_page.handle_input(&mut modem).unwrap();
                match input {
                    Some(0x41) => {
                        log::info!("Touche ENVOI pressée");
                    }
                    Some(0x49) => {
                        log::info!("Touche CX/FIN pressée");
                        modem.write_all(b"\x0cAu revoir !").unwrap();
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        break;
                    }
                    _ => log::warn!("Touche non reconnue"),
                }
            }

            _ => {}
        }

        match modem.read_carrier_detect() {
            Ok(false) => {
                log::info!("Connexion interrompue ! Arrêt du serveur.");
                break;
            }
            Ok(true) => {
                continue;
            }
            Err(e) => {
                log::error!("Erreur lors de la lecture de CD: {}", e);
                break;
            }
        }
    }
}
