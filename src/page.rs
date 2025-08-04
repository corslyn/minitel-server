pub struct Page {
    name: String,
    vdt_file: String,
    zones: Vec<Zone>,
}

pub struct Zone {
    id: u8,
    x: u8,
    y: u8,
    width: u8,
}

pub fn new_page(name: &str, vdt_file: &str) -> Page {
    Page {
        name: name.to_string(),
        vdt_file: vdt_file.to_string(),
        zones: Vec::new(),
    }
}
