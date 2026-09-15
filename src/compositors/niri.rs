use color_eyre::eyre::{Result, eyre};
use niri_ipc::{
    Request, Response, Workspace,
    socket::Socket,
    state::{EventStreamStatePart, WorkspacesState},
};
use std::sync::mpsc::Sender;

pub fn watch_workspaces(sender: &Sender<Vec<Workspace>>) -> Result<()> {
    let mut socket = Socket::connect()?;

    let reply = socket.send(Request::EventStream)?;

    match reply {
        Ok(Response::Handled) => {}
        Ok(other) => return Err(eyre!("unexpected result: {other:?}")),
        Err(message) => return Err(eyre!("niri rejected request: {message}")),
    }

    let mut state = WorkspacesState::default();
    let mut read_event = socket.read_events();

    loop {
        let event = read_event()?;

        if state.apply(event).is_none() {
            let mut workspaces: Vec<_> = state.workspaces.values().cloned().collect();

            workspaces.sort_by_key(|ws| ws.idx);

            sender
                .send(workspaces)
                .map_err(|_| eyre!("workspace receiver disconnected"))?;
        }
    }
}
