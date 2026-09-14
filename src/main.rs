use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CenterBox};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

mod battery;
mod clock;
mod compositors;
mod workspaces;

fn build_ui(app: &Application) {
    if let Some(window) = app.windows().first() {
        window.present();
        return;
    }

    let layout = CenterBox::new();

    let time_label = clock::build();
    let workspaces = workspaces::build();
    let sys_info = battery::build();

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
