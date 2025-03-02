use clap::builder::PossibleValue;

#[derive(Debug, Clone, clap::Parser)]
#[clap(bin_name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub enum Cli {
    /// Start a daemon that reorders workspaces on workspace creation and output changes
    Daemon,
    /// Reorder all workspaces to their correct number/index
    Reorder,
    /// Change focus to the desired position of the focused group.
    Focus {
        /// Direction or position to focus.
        action: Action,
    },
    /// Move the focused container to the desired position of the focused group.
    Move {
        /// Direction or position to focus.
        action: Action,
    },
    /// Change focus to the desired group.
    /// Position will be retained.
    FocusGroup {
        /// Direction or position to focus.
        action: Action,
    },
    /// Move the focused container to the desired group.
    /// Position will be retained.
    MoveGroup {
        /// Direction or position to focus.
        action: Action,
    },
}

#[derive(Debug, Clone)]
pub enum Action {
    Next,
    Prev,
    Position(i32),
}

impl clap::ValueEnum for Action {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Action::Next,
            Action::Prev,
            Action::Position(1),
            Action::Position(2),
            Action::Position(3),
            Action::Position(4),
            Action::Position(5),
            Action::Position(6),
            Action::Position(7),
            Action::Position(8),
            Action::Position(9),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Action::Next => Some(PossibleValue::new("next")),
            Action::Prev => Some(PossibleValue::new("prev")),
            Action::Position(1) => Some(PossibleValue::new("1")),
            Action::Position(2) => Some(PossibleValue::new("2")),
            Action::Position(3) => Some(PossibleValue::new("3")),
            Action::Position(4) => Some(PossibleValue::new("4")),
            Action::Position(5) => Some(PossibleValue::new("5")),
            Action::Position(6) => Some(PossibleValue::new("6")),
            Action::Position(7) => Some(PossibleValue::new("7")),
            Action::Position(8) => Some(PossibleValue::new("8")),
            Action::Position(9) => Some(PossibleValue::new("9")),
            _ => None,
        }
    }
}
