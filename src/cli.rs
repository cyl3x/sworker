#[derive(Debug, Clone, clap::Parser)]
#[clap(bin_name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub enum Cli {
    Daemon,
    Focus {
        /// Focus in the given direction or position.
        #[arg(value_parser = parse_position)]
        position: PositionCommand,
    },
    Move {
        /// Focus in the given direction or position.
        #[arg(value_parser = parse_position)]
        position: PositionCommand,
    },
}

#[derive(Debug, Clone)]
pub enum PositionCommand {
    Next,
    Prev,
    Position(i32),
}

fn parse_position(s: &str) -> Result<PositionCommand, String> {
    match s {
        "next" => Ok(PositionCommand::Next),
        "prev" => Ok(PositionCommand::Prev),
        _ => s
            .parse::<u8>()
            .map(|u| PositionCommand::Position(u as i32))
            .map_err(|_| format!("Invalid position: {}", s)),
    }
}
