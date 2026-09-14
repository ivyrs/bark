use gtk::Label;
use gtk::glib::DateTime;

fn current_time() -> String {
    const UNKNOWN_TIME: &str = "--:--:--";

    let Ok(time) = DateTime::now_local() else {
        return UNKNOWN_TIME.to_owned();
    };

    let Ok(formatted) = time.format("%H:%M:%S") else {
        return UNKNOWN_TIME.to_owned();
    };

    formatted.to_string()
}

fn start_clock(label: &Label) {
    let l = label.clone();

    gtk::glib::timeout_add_seconds_local(1, move || {
        let time = current_time();
        l.set_label(&time);

        gtk::glib::ControlFlow::Continue
    });
}

pub fn build() -> Label {
    let time = current_time();
    let label = Label::new(Some(&time));

    start_clock(&label);
    label
}
