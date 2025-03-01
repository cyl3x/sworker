use swayipc::{Connection, WorkspaceChange};

mod manager;

pub use manager::*;

pub fn process_event(connection: Connection, event: swayipc::Event) -> Result<(), swayipc::Error> {
    match event {
        swayipc::Event::Workspace(event) => match event.change {
            WorkspaceChange::Init | WorkspaceChange::Empty => Manager::new(connection)?.reorder(),
            _ => Ok(()),
        },
        swayipc::Event::Output(_) => Manager::new(connection)?.reorder(),
        _ => Ok(()),
    }
}
