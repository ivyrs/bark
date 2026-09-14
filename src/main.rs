use gtk::glib::DateTime;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CenterBox, Label};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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

fn build_ui(app: &Application) {
    if let Some(window) = app.windows().first() {
        window.present();
        return;
    }

    let layout = CenterBox::new();

    let time_text = current_time();
    let time_label = Label::new(Some(&time_text));
    start_clock(&time_label);

    let workspaces = Label::new(Some("workspaces"));
    let sys_info = Label::new(Some("system info"));

    layout.set_start_widget(Some(&time_label));
    layout.set_center_widget(Some(&workspaces));
    layout.set_end_widget(Some(&sys_info));

    let margin: i32 = 16;

    layout.set_margin_start(margin);
    layout.set_margin_end(margin);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("bark")
        .default_height(32)
        .child(&layout)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);

    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.auto_exclusive_zone_enable();

    window.present();
}

fn main() {
    let app = Application::builder().application_id("rs.ivy.bark").build();

    app.connect_activate(build_ui);

    app.run();
}
