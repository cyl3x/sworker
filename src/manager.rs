use swayipc::{Connection, Error, NodeType, Output, Workspace};

use crate::{Numberer, Positioner};

pub struct Manager {
    pub connection: Connection,
    pub workspaces: Vec<Workspace>,
    pub outputs: Vec<Output>,
    pub numberer: Numberer,
    pub positioner: Positioner,
    pub is_alone: bool,
}

impl Manager {
    pub fn new(mut connection: Connection) -> Result<Self, Error> {
        let workspaces = connection.get_workspaces()?;
        let outputs = connection.get_outputs()?;

        let focused = workspaces
            .iter()
            .find(|w| w.focused)
            .expect("focused workspace should always be focused");

        let focused_node = connection
            .get_tree()?
            .find(|node| node.node_type == NodeType::Workspace && focused.id == node.id)
            .expect("workspace of get_workspaces should be in get_tree");

        Ok(Self {
            numberer: Numberer::new(&workspaces, &outputs),
            positioner: Positioner::new(&workspaces),
            workspaces,
            outputs,
            connection,
            is_alone: focused_node.nodes.len() <= 1,
        })
    }

    pub fn reorder(&mut self) -> Result<(), Error> {
        self.numberer.reorder(&mut self.connection)
    }

    pub fn position_focus_next(&mut self) -> Result<(), Error> {
        let num = if self.positioner.is_end() && !self.positioner.is_full() && !self.is_alone {
            let num = self.numberer.append_at(self.positioner.num());
            println!("position_focus_next: {} | {}", self.positioner.num(), num);
            self.numberer.reorder(&mut self.connection)?;

            num
        } else {
            self.positioner.wrapping_position_add(1)
        };

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn position_focus_prev(&mut self) -> Result<(), Error> {
        let num = if self.positioner.is_start() && !self.positioner.is_full() && !self.is_alone {
            let num = self.numberer.prepend_at(self.positioner.num());
            self.numberer.reorder(&mut self.connection)?;

            num
        } else {
            self.positioner.wrapping_position_add(-1)
        };

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn position_focus_to(&mut self, position: i32) -> Result<(), Error> {
        let num = self.positioner.group * 10 + position.clamp(1, 9);

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn position_move_next(&mut self) -> Result<(), Error> {
        let num = if self.positioner.is_end() && !self.positioner.is_full() && !self.is_alone {
            let num = self.numberer.append_at(self.positioner.num());
            self.numberer.reorder(&mut self.connection)?;

            num
        } else {
            self.positioner.wrapping_position_add(1)
        };

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn position_move_prev(&mut self) -> Result<(), Error> {
        let num = if self.positioner.is_start() && !self.positioner.is_full() && !self.is_alone {
            let num = self.numberer.prepend_at(self.positioner.num());
            self.numberer.reorder(&mut self.connection)?;

            num
        } else {
            self.positioner.wrapping_position_add(-1)
        };

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn position_move_to(&mut self, position: i32) -> Result<(), Error> {
        let num = self.positioner.saturating_position_to(position);

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn group_focus_next(&mut self) -> Result<(), Error> {
        let num = self.positioner.wrapping_group_add(1);

        self.group_focus_to(num / 10)
    }

    pub fn group_focus_prev(&mut self) -> Result<(), Error> {
        let num = self.positioner.wrapping_group_add(-1);

        self.group_focus_to(num / 10)
    }

    pub fn group_focus_to(&mut self, group: i32) -> Result<(), Error> {
        let num = self.positioner.saturating_group_to(group);

        let output = self
            .workspaces
            .iter()
            .find(|w| w.num / 10 == num / 10)
            .map(|w| w.output.as_str());

        if let Some(output) = output {
            self.connection.run_command(format!("focus output {output}"))?;
        }

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn group_move_next(&mut self) -> Result<(), Error> {
        let num = self.positioner.wrapping_group_add(1);

        self.group_move_to(num / 10)
    }

    pub fn group_move_prev(&mut self) -> Result<(), Error> {
        let num = self.positioner.wrapping_group_add(-1);

        self.group_move_to(num / 10)
    }

    pub fn group_move_to(&mut self, group: i32) -> Result<(), Error> {
        let num = self.positioner.saturating_group_to(group);

        let output = self
            .workspaces
            .iter()
            .find(|w| w.num / 10 == num / 10)
            .map(|w| w.output.as_str());

        if let Some(output) = output {
            self.connection
                .run_command(format!("[con_id=__focused__] move container to output {output}, focus"))?;
        }

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }
}
