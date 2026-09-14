use gtk::Label;
use gtk::glib::DateTime;

fn current_time() -> String {
    let time = DateTime::now_local().expect("could not get local time");
    let formatted = time.format("%H:%M:%S").expect("could not format time");
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
