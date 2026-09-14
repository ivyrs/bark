use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn build_ui(app: &Application) {
    let text = Label::new(Some("woof"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("bark")
        .default_height(100)
        .default_width(400)
        .child(&text)
        .build();

    window.present();
}

fn main() {
    let app = Application::builder().application_id("rs.ivy.bark").build();

    app.connect_activate(build_ui);

    app.run();
}
