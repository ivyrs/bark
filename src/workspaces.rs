use crate::compositors::niri;
use gtk::Label;
use niri_ipc::Workspace;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn format_workspaces(workspaces: &[Workspace]) -> String {
    workspaces
        .iter()
        .map(|ws| {
            if ws.is_focused {
                format!("[{}]", ws.idx)
            } else {
                ws.idx.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn start_stream(label: &Label) {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        if let Err(error) = niri::watch_workspaces(&sender) {
            eprintln!("workspace event stream stopped: {error}");
        }
    });

    let l = label.clone();

    gtk::glib::timeout_add_local(Duration::from_millis(100), move || {
        match receiver.try_recv() {
            Ok(workspaces) => {
                l.set_label(&format_workspaces(&workspaces));
                gtk::glib::ControlFlow::Continue
            }
            Err(TryRecvError::Empty) => gtk::glib::ControlFlow::Continue,
            Err(TryRecvError::Disconnected) => gtk::glib::ControlFlow::Break,
        }
    });
}

pub fn build() -> Label {
    let label = Label::new(Some("niri unavailable"));
    start_stream(&label);
    label
}
