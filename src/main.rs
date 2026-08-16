use clap::Parser;
use cli::{Action, Cli};
use swayipc::Connection;
use sworker::{Manager, process_event};

mod cli;

fn main() -> Result<(), swayipc::Error> {
    let cli = Cli::parse();
    let mut connection = Connection::new()?;
    let mut manager = Manager::new(&mut connection)?;

    match cli {
        Cli::Focus { action } => match action {
            Action::Next => manager.position_focus_next()?,
            Action::Prev => manager.position_focus_prev()?,
            Action::Position(position) => manager.position_focus_to(position)?,
        },
        Cli::Move { action } => match action {
            Action::Next => manager.position_move_next()?,
            Action::Prev => manager.position_move_prev()?,
            Action::Position(position) => manager.position_move_to(position)?,
        },
        Cli::FocusGroup { action } => match action {
            Action::Next => manager.group_focus_next()?,
            Action::Prev => manager.group_focus_prev()?,
            Action::Position(position) => manager.group_focus_to(position)?,
        },
        Cli::MoveGroup { action } => match action {
            Action::Next => manager.group_move_next()?,
            Action::Prev => manager.group_move_prev()?,
            Action::Position(position) => manager.group_move_to(position)?,
        },
        Cli::Reorder => manager.reorder()?,
        Cli::Daemon => {
            manager.reorder()?;

            let events = Connection::new()?.subscribe([swayipc::EventType::Workspace, swayipc::EventType::Output])?;

            for event in events {
                match event {
                    Ok(event) => {
                        if let Err(err) = process_event(&mut connection, event) {
                            eprintln!("Error processing event: {err}");
                        }
                    }
                    Err(swayipc::Error::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(err) => eprintln!("Error receiving event: {err}"),
                }
            }
        }
    }

    Ok(())
}
