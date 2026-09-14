use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn build_ui(app: &Application) {
    let text = Label::new(Some("woof"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("bark")
        .default_height(32)
        .child(&text)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);

    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.auto_exclusive_zone_enable();

    window.present()
}

fn main() {
    let app = Application::builder().application_id("rs.ivy.bark").build();

    app.connect_activate(build_ui);

    app.run();
}
