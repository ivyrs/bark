use gtk::Label;
use std::{fs, io};

fn battery_text() -> io::Result<String> {
    for entry in fs::read_dir("/sys/class/power_supply")? {
        let path = entry?.path();

        let Ok(supply_type) = fs::read_to_string(path.join("type")) else {
            continue;
        };

        if supply_type.trim() != "Battery" {
            continue;
        }

        let capacity = fs::read_to_string(path.join("capacity"))?;
        return Ok(format!("{}%", capacity.trim()));
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "battery not found"))
}
pub fn build() -> Label {
    let text = match battery_text() {
        Ok(text) => text,
        Err(error) => {
            eprintln!("could not read battery: {error}");
            "battery unavailable".to_owned()
        }
    };
    Label::new(Some(&text))
}
