use swayipc::Workspace;

use crate::{NUMBERS_PER_GROUP, POSITIONS_PER_GROUP};

/// A struct to manage the positioning of workspaces.
/// Workspaces are grouped per output by 10s, starting at 1x.
/// Workspaces are positioned per output by 1s, starting at x1.
pub(crate) struct Positioner {
    pub(crate) group: i32,
    pub(crate) group_highest: i32,
    pub(crate) position: i32,
    pub(crate) position_highest: i32,
}

impl Positioner {
    /// Create a new Positioner from a list of workspaces.
    /// The list is assumed to be numbered already by [`Numberer`](struct@crate::numberer::Numberer).
    pub(crate) fn new(workspaces: &[Workspace]) -> Self {
        let focused = workspaces.iter().find(|ws| ws.focused).map_or(NUMBERS_PER_GROUP, |ws| ws.num);

        Self {
            group: focused / NUMBERS_PER_GROUP,
            group_highest: workspaces.iter().map(|ws| ws.num / NUMBERS_PER_GROUP).max().unwrap_or(1),
            position: focused % NUMBERS_PER_GROUP,
            position_highest: workspaces
                .iter()
                .filter(|ws| ws.num / NUMBERS_PER_GROUP == focused / NUMBERS_PER_GROUP)
                .map(|ws| ws.num % NUMBERS_PER_GROUP)
                .max()
                .unwrap_or(0),
        }
    }

    pub(crate) const fn num(&self) -> i32 {
        self.group * NUMBERS_PER_GROUP + self.position
    }

    /// Move `add` groups, wrapping at the first and last group.
    pub(crate) const fn wrapping_group(&self, add: i32) -> i32 {
        let group = self.group + add;

        if group > self.group_highest {
            1
        } else if group < 1 {
            self.group_highest
        } else {
            group
        }
    }

    /// The number of `group` at the current position, clamped to the existing groups.
    pub(crate) fn saturating_group_to(&self, group: i32) -> i32 {
        group.clamp(1, self.group_highest) * NUMBERS_PER_GROUP + self.position
    }

    pub(crate) const fn wrapping_position_add(&self, add: i32) -> i32 {
        self.wrapping_position_to(self.position + add)
    }

    /// The number of `position` in the current group, wrapping at the first and last position.
    pub(crate) const fn wrapping_position_to(&self, position: i32) -> i32 {
        let position = if position > self.position_highest {
            1
        } else if position < 1 {
            self.position_highest
        } else {
            position
        };

        self.group * NUMBERS_PER_GROUP + position
    }

    /// The number of `position` in the current group, clamped to the positions a group can hold.
    pub(crate) fn position_to(&self, position: i32) -> i32 {
        self.group * NUMBERS_PER_GROUP + position.clamp(1, POSITIONS_PER_GROUP)
    }

    /// The number of `position` in the current group, clamped to the positions the group holds.
    pub(crate) fn saturating_position_to(&self, position: i32) -> i32 {
        self.group * NUMBERS_PER_GROUP + position.clamp(1, self.position_highest.max(1))
    }

    pub(crate) const fn is_start(&self) -> bool {
        self.position == 1
    }

    pub(crate) const fn is_end(&self) -> bool {
        self.position == self.position_highest
    }

    pub(crate) const fn is_full(&self) -> bool {
        self.position_highest >= POSITIONS_PER_GROUP
    }
}
