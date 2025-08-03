use log;
use serialport::{self, SerialPort};
use std::time::Duration;

fn main() {
    // config du port série 1200 bauds, 7-E-1
    log::info!("Initialisation du modem");
    let mut modem = serialport::new("/dev/ttyUSB0", 1200) // a faire: detection auto du port
        .parity(serialport::Parity::Even)
        .data_bits(serialport::DataBits::Seven)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_secs(2))
        .open()
        .unwrap();

    init_modem(&mut modem); // a faire: mise en place d'un flag pour ne pas init le modem si on utilise un minitel retourné

    handle_connection(&mut modem);

    main_loop(modem)
}

fn init_modem(modem: &mut Box<dyn SerialPort>) {
    modem.write_all("ATZ0\r".as_bytes()).unwrap(); // reset
    modem
        .write_all("ATE0L0M0X4&N2S27=16S10=100S0=1\r".as_bytes())
        .unwrap(); // https://noelmrtn.fr/posts/v23_server/
}

fn handle_connection(modem: &mut Box<dyn SerialPort>) {
    loop {
        if modem.read_ring_indicator().unwrap() {
            log::info!("Appel reçu ! Décrochage...");
            break;
        }
    }
    loop {
        if modem.read_carrier_detect().unwrap() {
            log::info!("Connexion établie !");
            break;
        }
    }
}

fn main_loop(mut modem: Box<dyn SerialPort>) {
    loop {
        if !modem.read_carrier_detect().unwrap() {
            log::info!("Connexion interrompue ! Arrêt du serveur");
            break;
        }
    }
}
