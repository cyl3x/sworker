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
                PositionCommand::Next => manager.focus_next()?,
                PositionCommand::Prev => manager.focus_prev()?,
                PositionCommand::Position(position) => manager.focus_to(position)?,
            }
        }
        Cli::Move { position } => {
            let mut manager = Manager::new(Connection::new()?)?;

            match position {
                PositionCommand::Next => manager.move_next()?,
                PositionCommand::Prev => manager.move_prev()?,
                PositionCommand::Position(position) => manager.move_to(position)?,
            }
        }
        Cli::Daemon => {
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
