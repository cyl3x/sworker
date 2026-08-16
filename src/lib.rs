use swayipc::{Connection, WorkspaceChange, Error};

mod manager;
mod numberer;
mod positioner;

pub use manager::Manager;

/// Number of positions a group can hold, i.e. the workspaces `x1` to `x9`.
pub(crate) const POSITIONS_PER_GROUP: i32 = 9;

/// Numbers a group spans, i.e. `x0` to `x9`.
pub(crate) const NUMBERS_PER_GROUP: i32 = POSITIONS_PER_GROUP + 1;

/// Reorder all workspaces in reaction to a sway event.
///
/// Only events that can change the workspace layout are acted on. Notably
/// [`WorkspaceChange::Rename`] is ignored, as reordering emits those itself.
pub fn process_event(connection: &mut Connection, event: swayipc::Event) -> Result<(), swayipc::Error> {
    match event {
        swayipc::Event::Workspace(event) => match event.change {
            WorkspaceChange::Init | WorkspaceChange::Empty => Manager::new(connection)?.reorder(),
            _ => Ok(()),
        },
        swayipc::Event::Output(_) => Manager::new(connection)?.reorder(),
        _ => Ok(()),
    }
}

/// Run `command` and report commands that sway rejected.
///
/// [`Connection::run_command`] only surfaces transport failures through its outer
/// [`Result`]; every command sway refused comes back as an `Err` entry of the returned
/// vector, which is easy to drop on the floor by accident.
pub fn run_command(connection: &mut Connection, command: impl AsRef<str>) -> Result<(), Error> {
    let command = command.as_ref();

    let failures = connection
        .run_command(command)?
        .into_iter()
        .filter_map(Result::err)
        .map(|error| error.to_string())
        .collect::<Vec<_>>();

    if failures.is_empty() {
        return Ok(());
    }

    Err(Error::CommandFailed(format!("'{command}': {}", failures.join(", "))))
}

