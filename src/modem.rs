use std::{thread::sleep, time::Duration};

use serialport::SerialPort;

pub fn init_modem(
    tty_path: &str,
    init_str: Option<&str>,
) -> Result<Box<dyn SerialPort>, Box<dyn std::error::Error>> {
    log::info!("Initialisation du modem");
    // config du port série 1200 bauds, 7-E-1
    // a faire: detection auto du port
    let mut modem = serialport::new(tty_path, 1200)
        .parity(serialport::Parity::Even)
        .data_bits(serialport::DataBits::Seven)
        .stop_bits(serialport::StopBits::One)
        .timeout(std::time::Duration::from_secs(2))
        .open()?;

    modem.write_all(b"ATZ0\r")?; // reset
    let init_str = init_str.unwrap_or("ATE0L0M0X4&N2S27=16S10=100S0=1\r");
    modem.write_all(init_str.as_bytes())?;
    Ok(modem)
}

pub fn handle_connection(modem: &mut Box<dyn SerialPort>) {
    loop {
        if modem.read_ring_indicator().unwrap_or(false) {
            log::info!("Appel reçu ! Décrochage...");
            break;
        }
        sleep(Duration::from_millis(100));
    }
    loop {
        if modem.read_carrier_detect().unwrap_or(false) {
            log::info!("Connexion établie !");
            break;
        }
        sleep(Duration::from_millis(100));
    }
}
