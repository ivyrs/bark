use crate::compositors::niri;
use gtk::Label;

pub fn build() -> Label {
    let workspaces = niri::get_workspaces();

    let text = match workspaces {
        Ok(workspaces) => workspaces
            .iter()
            .map(|ws| {
                if ws.is_focused {
                    format!("[{}]", ws.idx)
                } else {
                    ws.idx.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
        Err(error) => {
            eprint!("could not get workspaces: {error}");
            "niri unavailable".to_owned()
        }
    };

    Label::new(Some(&text))
}
