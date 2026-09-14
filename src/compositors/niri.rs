use color_eyre::eyre::{Result, eyre};
use niri_ipc::{Request, Response, Workspace, socket::Socket};

pub fn get_workspaces() -> Result<Vec<Workspace>> {
    let mut socket = Socket::connect()?;

    let reply = socket.send(Request::Workspaces)?;

    match reply {
        Ok(Response::Workspaces(mut workspaces)) => {
            workspaces.sort_by_key(|workspace| workspace.idx);
            Ok(workspaces)
        }
        Ok(other) => Err(eyre!("unexpected result: {other:?}")),
        Err(message) => Err(eyre!("niri rejected request: {message}")),
    }
}
