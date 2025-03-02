use clap::Parser;
use cli::{Cli, Action};
use swayipc::Connection;
use sworker::{process_event, Manager};

mod cli;

fn main() -> Result<(), swayipc::Error> {
    match Cli::parse() {
        Cli::Focus { action } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match action {
                Action::Next => manager.position_focus_next()?,
                Action::Prev => manager.position_focus_prev()?,
                Action::Position(position) => manager.position_focus_to(position)?,
            }
        }
        Cli::Move { action } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match action {
                Action::Next => manager.position_move_next()?,
                Action::Prev => manager.position_move_prev()?,
                Action::Position(position) => manager.position_move_to(position)?,
            }
        }
        Cli::FocusGroup { action } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match action {
                Action::Next => manager.group_focus_next()?,
                Action::Prev => manager.group_focus_prev()?,
                Action::Position(position) => manager.group_focus_to(position)?,
            }
        }
        Cli::MoveGroup { action } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match action {
                Action::Next => manager.group_move_next()?,
                Action::Prev => manager.group_move_prev()?,
                Action::Position(position) => manager.group_move_to(position)?,
            }
        }
        Cli::Daemon => {
            Manager::new(Connection::new()?)?.reorder()?;

            for event in Connection::new()?.subscribe([swayipc::EventType::Workspace])? {
                match event {
                    Ok(event) => process_event(Connection::new()?, event)?,
                    Err(err) => eprintln!("Error processing event: {err}"),
                }
            }
        }
    }

    Ok(())
}
