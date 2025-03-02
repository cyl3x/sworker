use clap::Parser;
use cli::{Cli, PositionCommand};
use swayipc::Connection;
use sworker::{process_event, Manager};

mod cli;

fn main() -> Result<(), swayipc::Error> {
    match Cli::parse() {
        Cli::Focus { position } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match position {
                PositionCommand::Next => manager.position_focus_next()?,
                PositionCommand::Prev => manager.position_focus_prev()?,
                PositionCommand::Position(position) => manager.position_focus_to(position)?,
            }
        }
        Cli::Move { position } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match position {
                PositionCommand::Next => manager.position_move_next()?,
                PositionCommand::Prev => manager.position_move_prev()?,
                PositionCommand::Position(position) => manager.position_move_to(position)?,
            }
        }
        Cli::FocusGroup { position } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match position {
                PositionCommand::Next => manager.group_focus_next()?,
                PositionCommand::Prev => manager.group_focus_prev()?,
                PositionCommand::Position(position) => manager.group_focus_to(position)?,
            }
        }
        Cli::MoveGroup { position } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match position {
                PositionCommand::Next => manager.group_move_next()?,
                PositionCommand::Prev => manager.group_move_prev()?,
                PositionCommand::Position(position) => manager.group_move_to(position)?,
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
