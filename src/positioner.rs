use swayipc::Workspace;

/// A struct to manage the positioning of workspaces.
/// Workspaces are grouped per output by 10s, starting at 1x.
/// Workspaces are positioned per output by 1s, starting at x1.
pub struct Positioner {
    pub group: i32,
    pub group_highest: i32,
    pub position: i32,
    pub position_highest: i32,
}

impl Positioner {
    /// Create a new Positioner from a list of workspaces.
    /// The list is assumed to ordered already by [`Numberer`](struct@numberer::Numberer).
    pub fn new(workspaces: &[Workspace]) -> Self {
        let focused = workspaces.iter().find(|ws| ws.focused).unwrap();

        Self {
            group: focused.num / 10,
            group_highest: workspaces.iter().map(|ws| ws.num / 10).max().unwrap(),
            position: focused.num % 10,
            position_highest: workspaces
                .iter()
                .filter(|ws| ws.num / 10 == focused.num / 10)
                .map(|ws| ws.num % 10)
                .max()
                .unwrap(),
        }
    }

    pub const fn num(&self) -> i32 {
        self.group * 10 + self.position
    }

    pub fn saturating_group_add(&self, add: i32) -> i32 {
        self.saturating_group_to(self.group + add)
    }

    pub fn wrapping_group_add(&self, add: i32) -> i32 {
        self.wrapping_group_to(self.group + add)
    }

    pub fn saturating_group_to(&self, group: i32) -> i32 {
        let group = group.clamp(1, self.group_highest);

        group * 10 + self.position
    }

    pub fn wrapping_group_to(&self, group: i32) -> i32 {
        let group = if group > self.group_highest {
            1
        } else if group < 1 {
            self.group_highest
        } else {
            group
        };

        group * 10 + self.position
    }

    pub fn saturating_position_add(&self, add: i32) -> i32 {
        self.saturating_position_to(self.position + add)
    }

    pub fn wrapping_position_add(&self, add: i32) -> i32 {
        self.wrapping_position_to(self.position + add)
    }

    pub fn saturating_position_to(&self, position: i32) -> i32 {
        let position = position.clamp(1, self.position_highest);

        self.group * 10 + position
    }

    pub fn wrapping_position_to(&self, position: i32) -> i32 {
        let position = if position > self.position_highest {
            1
        } else if position < 1 {
            self.position_highest
        } else {
            position
        };

        self.group * 10 + position
    }

    pub fn is_start(&self) -> bool {
        self.position == 1
    }

    pub fn is_end(&self) -> bool {
        self.position == self.position_highest
    }

    pub fn is_full(&self) -> bool {
        self.position_highest >= 9
    }
}
