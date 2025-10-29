use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Error as IoError, Write},
};

use serde::Deserialize;

use crate::page::Page;

#[derive(Debug, Deserialize)]
pub struct PageConfig {
    pub name: String,
    pub path: String,
    pub routes: Option<HashMap<String, String>>,
    pub guide: Option<String>,
}

pub fn paginate_minitel(text: &str) -> Vec<String> {
    let mut pages = Vec::new();
    let mut current_page = String::new();
    let mut current_line = String::new();
    let mut line_count = 0;

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > 40 {
            current_page.push_str(&current_line);
            current_page.push('\n');
            current_line.clear();
            line_count += 1;
        }
        if line_count >= 23 {
            pages.push(current_page.clone());
            current_page.clear();
            line_count = 0;
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    // dernière ligne
    if !current_line.is_empty() {
        current_page.push_str(&current_line);
        current_page.push('\n');
    }
    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

pub fn to_minitel(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c {
            'é' => vec!['\x19', 'B', 'e'],
            'è' => vec!['\x19', 'A', 'e'],
            'ê' => vec!['\x19', 'C', 'e'],
            'à' => vec!['\x19', 'A', 'a'],
            'ù' => vec!['\x19', 'A', 'u'],
            'ç' => vec!['\x19', 'D', 'c'],
            'ï' => vec!['\x19', 'H', 'i'],
            'ö' => vec!['\x19', 'H', 'o'],
            _ => vec![c],
        })
        .collect()
}

pub fn generate_guide_vdt(pages: &[Page]) -> Result<(), Box<dyn Error>> {
    // Cherche la page "teletel.guide"
    let guide_page = pages
        .iter()
        .find(|p| p.name == "teletel.guide")
        .ok_or("Page teletel.guide introuvable dans le JSON")?;

    let mut buf: Vec<u8> = Vec::new();

    // -- Écran de titre --
    buf.push(0x0C); // ^L : efface écran
    buf.push(0x1F); // sélecteur
    buf.extend_from_slice(b"BA");
    buf.push(0x1B); // ESC
    buf.extend_from_slice(b"M"); // mode gras
    buf.extend_from_slice(to_minitel("Annuaire des services").as_bytes());

    // -- Nouvelle zone de texte --
    buf.push(0x1F);
    buf.extend_from_slice(b"CA");
    buf.push(0x1B);
    buf.push(0x44);
    buf.push(0x60);
    buf.push(0x12);
    buf.extend_from_slice(b"g");
    buf.push(0x1F);
    buf.extend_from_slice(b"DA");

    // -- Liste des routes numériques --
    for (key, target) in &guide_page.routes {
        if key.chars().all(|c| c.is_ascii_digit()) {
            let line = format!(" {} - {}\r\n", key, target.to_uppercase());
            buf.extend_from_slice(to_minitel(&line).as_bytes());
        }
    }

    // -- Ligne d'aide "Tapez un N° puis ENVOI" avec codes exacts --
    buf.push(0x1F);
    buf.extend_from_slice(b"WA");
    buf.push(0x1B);
    buf.push(0x44);
    buf.push(0x60);
    buf.push(0x12);
    buf.extend_from_slice(b"g");
    buf.push(0x1F);
    buf.extend_from_slice(b"XP");

    buf.extend_from_slice(b"Tapez un N\x190 .. puis ");
    buf.extend_from_slice(b"\x1b]ENVOI\x1fX\\\x11");

    // -- Écriture dans le fichier VDT --
    let mut f = File::create(&guide_page.vdt_file)?;
    f.write_all(&buf)?;
    f.flush()?;
    Ok(())
}
