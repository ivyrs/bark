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
        let status = fs::read_to_string(path.join("status"))?;
        return Ok(format!("{}% {}", capacity.trim(), status.trim()));
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "battery not found"))
}

fn update_battery(label: &Label) {
    let l = label.clone();

    gtk::glib::timeout_add_seconds_local(10, move || {
        let text = match battery_text() {
            Ok(text) => text,
            Err(error) => {
                eprintln!("couldn't read battery percentage: {error:?}");
                "bat unavailable".to_owned()
            }
        };

        l.set_label(&text);

        gtk::glib::ControlFlow::Continue
    });
}

pub fn build() -> Label {
    let text = match battery_text() {
        Ok(text) => text,
        Err(error) => {
            eprintln!("could not read battery: {error}");
            "battery unavailable".to_owned()
        }
    };
    let label = Label::new(Some(&text));

    update_battery(&label);
    label
}
