use swayipc::{Connection, Error, Node, NodeType, Workspace};

use crate::NUMBERS_PER_GROUP;
use crate::numberer::Numberer;
use crate::positioner::Positioner;
use crate::run_command;

pub struct Manager<'a> {
    connection: &'a mut Connection,
    workspaces: Vec<Workspace>,
    numberer: Numberer,
    positioner: Positioner,
    /// Number of windows on the focused workspace.
    nodes: usize,
}

impl<'a> Manager<'a> {
    pub fn new(connection: &'a mut Connection) -> Result<Self, Error> {
        let workspaces = connection.get_workspaces()?;
        let outputs = connection.get_outputs()?;

        let mut nodes = 0;

        if let Some(focused) = workspaces.iter().find(|w| w.focused).or_else(|| workspaces.first()) {
            nodes = connection
                .get_tree()?
                .find(|node| node.node_type == NodeType::Workspace && focused.id == node.id)
                .map_or(0, |ws| Self::count_nodes(&ws));
        }

        Ok(Self {
            numberer: Numberer::new(&workspaces, &outputs),
            positioner: Positioner::new(&workspaces),
            connection,
            workspaces,
            nodes,
        })
    }

    pub fn reorder(&mut self) -> Result<(), Error> {
        let commands = self.numberer.rename_commands(&self.workspaces);

        if commands.is_empty() {
            return Ok(());
        }

        run_command(self.connection, commands.join("; "))
    }

    /// Focus the next position, inserting a workspace there if `new`.
    pub fn position_focus_next(&mut self, new: bool) -> Result<(), Error> {
        let num = if (new || self.positioner.is_end()) && !self.positioner.is_full() && self.nodes > 0 {
            let num = self.numberer.append_at(self.positioner.num());
            self.reorder()?;

            num
        } else {
            self.positioner.wrapping_position_add(1)
        };

        run_command(self.connection, format!("workspace number {num}"))
    }

    /// Focus the previous position, inserting a workspace there if `new`.
    pub fn position_focus_prev(&mut self, new: bool) -> Result<(), Error> {
        let num = if (new || self.positioner.is_start()) && !self.positioner.is_full() && self.nodes > 0 {
            let num = self.numberer.prepend_at(self.positioner.num());
            self.reorder()?;

            num
        } else {
            self.positioner.wrapping_position_add(-1)
        };

        run_command(self.connection, format!("workspace number {num}"))
    }

    /// Focus `position`, inserting a workspace before the one already there if `new`.
    pub fn position_focus_to(&mut self, position: i32, new: bool) -> Result<(), Error> {
        let num = if new {
            self.insert_position(position, self.nodes == 0)?
        } else {
            self.positioner.position_to(position)
        };

        run_command(self.connection, format!("workspace number {num}"))
    }

    /// Move the focused container to the next position, inserting a workspace there if `new`.
    pub fn position_move_next(&mut self, new: bool) -> Result<(), Error> {
        let num = if (new || self.positioner.is_end()) && !self.positioner.is_full() && self.nodes > 1 {
            let num = self.numberer.append_at(self.positioner.num());
            self.reorder()?;

            num
        } else {
            self.positioner.wrapping_position_add(1)
        };

        run_command(
            self.connection,
            format!("[con_id=__focused__] move container to workspace number {num}, focus"),
        )
    }

    /// Move the focused container to the previous position, inserting a workspace there if `new`.
    pub fn position_move_prev(&mut self, new: bool) -> Result<(), Error> {
        let num = if (new || self.positioner.is_start()) && !self.positioner.is_full() && self.nodes > 1 {
            let num = self.numberer.prepend_at(self.positioner.num());
            self.reorder()?;

            num
        } else {
            self.positioner.wrapping_position_add(-1)
        };

        run_command(
            self.connection,
            format!("[con_id=__focused__] move container to workspace number {num}, focus"),
        )
    }

    /// Move the focused container to `position`, inserting a workspace before the one already there if `new`.
    pub fn position_move_to(&mut self, position: i32, new: bool) -> Result<(), Error> {
        let num = if new {
            self.insert_position(position, self.nodes <= 1)?
        } else {
            self.positioner.position_to(position)
        };

        run_command(
            self.connection,
            format!("[con_id=__focused__] move container to workspace number {num}"),
        )
    }

    pub fn group_focus_next(&mut self) -> Result<(), Error> {
        self.group_focus_to(self.positioner.wrapping_group(1))
    }

    pub fn group_focus_prev(&mut self) -> Result<(), Error> {
        self.group_focus_to(self.positioner.wrapping_group(-1))
    }

    pub fn group_focus_to(&mut self, group: i32) -> Result<(), Error> {
        let num = self.positioner.saturating_group_to(group);

        if let Some(output) = self.output_of(num) {
            run_command(self.connection, format!("focus output {output}"))?;
        }

        run_command(self.connection, format!("workspace number {num}"))
    }

    pub fn group_move_next(&mut self) -> Result<(), Error> {
        self.group_move_to(self.positioner.wrapping_group(1))
    }

    pub fn group_move_prev(&mut self) -> Result<(), Error> {
        self.group_move_to(self.positioner.wrapping_group(-1))
    }

    pub fn group_move_to(&mut self, group: i32) -> Result<(), Error> {
        let num = self.positioner.saturating_group_to(group);

        if let Some(output) = self.output_of(num) {
            run_command(
                self.connection,
                format!("[con_id=__focused__] move container to output {output}, focus"),
            )?;
        }

        run_command(
            self.connection,
            format!("[con_id=__focused__] move container to workspace number {num}"),
        )
    }

    /// Free `position` for a new workspace and return its number.
    ///
    /// `vacates` tells whether the command leaves the focused workspace empty.
    /// Sway destroys it then, so the group keeps the size it has and the focused workspace can simply
    /// take `position` itself, which needs no free position and therefore fits a full group too.
    fn insert_position(&mut self, position: i32, vacates: bool) -> Result<i32, Error> {
        // A group without a free position cannot grow, so the workspace already there is used.
        if !vacates && self.positioner.is_full() {
            return Ok(self.positioner.position_to(position));
        }

        let num = if vacates {
            self.numberer.relocate(self.positioner.num(), self.positioner.saturating_position_to(position))
        } else {
            self.numberer.prepend_at(self.positioner.position_to(position))
        };

        self.reorder()?;

        Ok(num)
    }

    /// The output holding the group of `num`.
    fn output_of(&self, num: i32) -> Option<String> {
        self.workspaces
            .iter()
            .find(|workspace| workspace.num / NUMBERS_PER_GROUP == num / NUMBERS_PER_GROUP)
            .map(|workspace| workspace.output.clone())
    }

    /// Count the windows below `node`, descending into split and floating containers.
    ///
    /// A child without children of its own is a window itself.
    fn count_nodes(node: &Node) -> usize {
        node.nodes
            .iter()
            .chain(&node.floating_nodes)
            .map(|child| Self::count_nodes(child).max(1))
            .sum()
    }
}
