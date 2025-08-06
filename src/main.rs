use crate::{modem::*, page::*};
use log;
use serialport::{self, SerialPort};
use std::{error::Error, process::exit};

mod modem;
mod page;
mod services;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("Démarrage du serveur Minitel...");

    // a faire: mise en place d'un flag pour ne pas init le modem si on utilise un minitel retourné
    let mut modem = init_modem("/dev/ttyUSB0", None)?;

    handle_connection(&mut modem);

    main_loop(modem).unwrap();

    Ok(())
}

fn main_loop(mut modem: Box<dyn SerialPort>) -> Result<(), Box<dyn Error>> {
    log::info!("En attente de la fin de connexion...");

    let pages = Page::load_pages_from_config("pages.json")?;
    let mut current_page = &pages[0];
    current_page.send(&mut modem)?;

    let mut code_service = String::new();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        let input = current_page.handle_input(&mut modem)?;

        match input {
            Some(0x13) => {
                // Touche spéciale
                log::info!("Touche spéciale pressée");
                let input = current_page.handle_input(&mut modem)?;
                match input {
                    Some(0x47) => {
                        // touche CORRECTION
                        log::info!("Touche CORRECTION pressée");
                        if !code_service.is_empty() {
                            code_service.pop();
                            modem.write_all(b"\x08.\x08")?; // efface le dernier caractère affiché et le remplace par un point
                        }
                    }
                    Some(0x41) => {
                        // Touche ENVOI
                        log::info!("Touche ENVOI pressée");
                        log::info!("Service demandé: {}", code_service);
                        if let Some(target_name) = current_page.routes.get(&code_service) {
                            if let Some(next_page) = pages.iter().find(|p| &p.name == target_name) {
                                if current_page.name == "teletel" {
                                    modem
                                        .write_all(b"\x1f\x40\x41\x14\x1b\x48connexion.\x12\x42")?; // tout ca pour envoyer "connexion..." clignotant sur la ligne 0...
                                    std::thread::sleep(std::time::Duration::from_secs(3)); // simulation du temps de connexion au service distant
                                }

                                if current_page.name == "meteo" {
                                    log::info!(
                                        "Récupération des données météo pour {}",
                                        code_service
                                    );
                                    let data = services::meteo::main_meteo(&code_service)
                                        .expect("Erreur lors de la récupération des données météo");
                                    let (ville, id, desc, temp, pression) = data;
                                    modem.write_all(
                                        format!(
                                            "\x0cMeteo pour {}: ID {}, {} à {}C, Pression: {} hPa",
                                            ville, id, desc, temp, pression
                                        )
                                        .as_bytes(),
                                    )?;
                                }

                                current_page = next_page;
                                modem.write_all(b"\x1f\x40\x41\x18\x0a")?; // efface la ligne 0
                                current_page.send(&mut modem)?;
                                code_service.clear();
                                continue;
                            } else {
                                modem.write_all(b"\x1f\x40\x41\x14Code de service inconnu")?;
                                std::thread::sleep(std::time::Duration::from_secs(1));
                                modem.write_all(b"\x1f\x40\x41\x18\x0a")?; // efface la ligne 0
                            }
                        }
                    }
                    Some(0x49) => {
                        // Touche CX/FIN
                        log::info!("Touche CX/FIN pressée");
                        if current_page.name == "teletel" {
                            modem.write_all(b"\x0cAu revoir !")?;
                            std::thread::sleep(std::time::Duration::from_secs(3));
                            break;
                        } else {
                            if let Some(target_name) = current_page.routes.get("cx") {
                                if let Some(previous_page) =
                                    pages.iter().find(|p| &p.name == target_name)
                                {
                                    current_page = previous_page;
                                    current_page.send(&mut modem)?;
                                    code_service.clear();
                                    continue;
                                } else {
                                    unreachable!()
                                }
                            }
                        }
                    }
                    Some(0x44) => {
                        // touche guide
                        log::info!("Touche GUIDE pressée");
                        if let Some(target_name) = &current_page.guide {
                            if let Some(previous_page) =
                                pages.iter().find(|p| &p.name == target_name)
                            {
                                current_page = previous_page;
                                current_page.send(&mut modem)?;
                                code_service.clear();
                                continue;
                            } else {
                                log::error!(
                                    "Page cible '{target_name}' introuvable pour la touche GUIDE"
                                );
                            }
                        } else {
                            log::debug!(
                                "Aucune page guide définie pour la page '{}'",
                                current_page.name
                            );
                        }
                    }
                    Some(0x42) => {
                        // touche retour
                        log::info!("Touche RETOUR pressée");
                        if let Some(target_name) = current_page.routes.get("retour") {
                            if let Some(previous_page) =
                                pages.iter().find(|p| &p.name == target_name)
                            {
                                current_page = previous_page;
                                current_page.send(&mut modem)?;
                                code_service.clear();
                                continue;
                            } else {
                                continue;
                            }
                        }
                    }

                    _ => log::error!("Touche non reconnue"),
                }
            }
            Some(input_char) => {
                if input_char.is_ascii_alphanumeric() {
                    code_service.push(input_char as char);
                    modem.write_all(&[input_char]).unwrap();
                }
            }
            _ => continue,
        }

        match modem.read_carrier_detect() {
            Ok(false) => {
                log::info!("Connexion interrompue ! Arrêt du serveur.");
                break;
            }
            Ok(true) => {}
            Err(e) => {
                log::error!("Erreur lors de la lecture de CD: {}", e);
                break;
            }
        }

        log::debug!("Page actuelle: {}", current_page.name);
    }

    Ok(())
}

/*
this rust code was sponsored by SPAMTON G. SPAMTON

HOLY [Cungadero] KRIS I HAVE BECOME [The Big One]


                  **************              **^^
              ****@@@@@@@@@@@@@@*^**********^*@@^^
            ^^@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@**
          ^^@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@^>^>^^^^
          ^^@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@######^^
          ^^((@@@@**^^^^^^^^^^^^^*@@@@@@@@@@((((^^
            >^@@}}................()@@@@@@}}^^^^
              @@))====..========....}}@@@@))))^^
            @@%%%%%%%%::%#%%%%#%::....@@@@%@%%^^
            @@~~===~=~@@--------@@....@@@@^^^^
            @@=====~==@@--------@@@@@@@@++
            @@------~-@@--------@@....@@@@
              @@@@@@@@..@@@@@@@@...   ..@@
            ^^]]@@@@ .  ..  ..(([[... ..@@  ****
        ^^^^.....       .   ..@@]]... ..@@^^@@@@^^
  ^^^^^^....................@@@@......@@@@@@@@^^^^
^^^^^^^^****################@@--....##@@@@@@><^^^^
^^^^^^^^^^^^))+*++@@********@@..  ..@@@@@@((^^
            >^<<..<<<)<<<<<)<< ...<<@@[[[[>>
              {{++..@@[[[]@@..  ..@@{{^^^^
            ^^<<@@..@@----@@..  --@@<<^^
            ^^@@@@..@@@@@@@@....@@@@@@^^
          ^^@@@@@@..............@@@@@@**
          *+@@@@@@@@@@@@@@@@@@@@@@@@@@@@^^
        **@@@@@@@@@@=+......==@@@@@@@@@@**
      **@@@@@@@@@@@@@@^^..^^@@@@@@++@@@@@@^^
    ^^@@@@@@**@@@@@@@@@@^^@@@@@@@@^^**@@@@^^
^^^^..^*@@^^  ^^@@@@@@@@@@@@@@@@@@^^  ^*@@@@^^
^^....%%>>^^  ^^@@@@@@@@@@@@@@@@>>^^  ^^@@>>..^^
^^++++))^>    ^^))@@@@@@@@@@@@@@^^    ^^)){#..--^^
  ^^^^^^        ^^@@@@@@@@@@@@@@^^      >^]]====^^
                ^^}}@@@@@@@@@@}}^^        ^^^^^^^^
                  ^^@@{{{{{{{{^^
                  ^^@@........^^
                    ^^..@@... ^^
                    ^^..@@..^^
                      >>**^>^^



*/
