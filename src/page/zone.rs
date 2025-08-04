use crate::page::Page;

pub struct Zone {
    pub id: u8,
    x: u8,
    y: u8,
    width: u8,
}

impl Zone {
    pub fn add(page: &mut Page, id: u8, x: u8, y: u8, width: u8) {
        page.zones.push(Zone { id, x, y, width });
    }
}
