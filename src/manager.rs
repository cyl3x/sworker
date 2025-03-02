use std::cmp::Ordering;
use std::collections::HashMap;

use swayipc::{Connection, Error, NodeType, Output, Workspace};

pub struct Info {
    group: i32,
    group_highest: i32,
    position: i32,
    position_highest: i32,
    is_alone: bool,
}

pub struct Manager {
    connection: Connection,
    workspaces: Vec<Workspace>,
    outputs: Vec<Output>,
    info: Info,
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
            info: Info {
                group: focused.num / 10,
                group_highest: outputs.len() as i32,
                position: focused.num % 10,
                position_highest: workspaces
                    .iter()
                    .filter(|w| w.num / 10 == focused.num / 10)
                    .last()
                    .unwrap()
                    .num
                    % 10,
                is_alone: focused_node.nodes.len() <= 1,
            },
            workspaces,
            outputs,
            connection,
        })
    }

    pub fn reorder(&mut self) -> Result<(), Error> {
        Self::reorder_opt(
            &mut self.connection,
            &self.workspaces.iter().map(Some).collect::<Vec<_>>(),
            &self.outputs,
        )
    }

    pub fn empty_at(&mut self, num: i32, prepend: bool) -> Result<(), Error> {
        let mut workspaces = self.workspaces.iter().map(Some).collect::<Vec<_>>();
        let index = self
            .workspaces
            .iter()
            .position(|w| w.num > num)
            .map(|p| p.saturating_sub(prepend as usize));

        if let Some(index) = index {
            workspaces.insert(index, None);
        } else {
            workspaces.push(None);
        }

        Self::reorder_opt(&mut self.connection, &workspaces, &self.outputs)
    }

    pub fn position_focus_next(&mut self) -> Result<(), Error> {
        let position = self.info.position + 1;

        let mut num = self.info.group * 10 + position;

        if position > self.info.position_highest {
            if !self.info.is_alone && self.info.position_highest != 9 {
                self.empty_at(num, false)?;
            } else {
                num = self.info.group * 10 + 1
            }
        }

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn position_focus_prev(&mut self) -> Result<(), Error> {
        let position = self.info.position - 1;

        let mut num = self.info.group * 10 + position;

        if position < 1 {
            if !self.info.is_alone && self.info.position_highest != 9 {
                num = self.info.group * 10 + 1;

                self.empty_at(num, true)?;
            } else {
                num = self.info.group * 10 + self.info.position_highest
            }
        }

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn position_focus_to(&mut self, position: i32) -> Result<(), Error> {
        let num = self.info.group * 10 + position.clamp(1, 9);

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn position_move_next(&mut self) -> Result<(), Error> {
        let position = self.info.position + 1;

        let mut num = self.info.group * 10 + position;

        if position > self.info.position_highest {
            if !self.info.is_alone && self.info.position_highest != 9 {
                self.empty_at(num, false)?;
            } else {
                num = self.info.group * 10 + 1
            }
        }

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn position_move_prev(&mut self) -> Result<(), Error> {
        let position = self.info.position - 1;

        let mut num = self.info.group * 10 + position;

        if position < 1 {
            if !self.info.is_alone && self.info.position_highest != 9 {
                num = self.info.group * 10 + 1;

                self.empty_at(num, true)?;
            } else {
                num = self.info.group * 10 + self.info.position_highest
            }
        }

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn position_move_to(&mut self, position: i32) -> Result<(), Error> {
        let num = self.info.group * 10 + position.clamp(1, 9);

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn group_focus_next(&mut self) -> Result<(), Error> {
        let mut group = self.info.group + 1;
        
        if group > self.info.group_highest {
            group = 1;
        }

        self.group_focus_to(group)
    }

    pub fn group_focus_prev(&mut self) -> Result<(), Error> {
        let mut group = self.info.group - 1;
        
        if group < 1 {
            group = self.info.group_highest;
        }

        self.group_focus_to(group)
    }

    pub fn group_focus_to(&mut self, group: i32) -> Result<(), Error> {
        let group = group.clamp(1, self.info.group_highest);

        let output = self.workspaces
            .iter()
            .find(|w| w.num / 10 == group)
            .map(|w| w.output.as_str());

        if let Some(output) = output {
            self.connection.run_command(format!("focus output {output}"))?;
        }

        let num = group * 10 + self.info.position;

        self.connection.run_command(format!("workspace number {num}"))?;

        Ok(())
    }

    pub fn group_move_next(&mut self) -> Result<(), Error> {
        let mut group = self.info.group + 1;
        
        if group > self.info.group_highest {
            group = 1;
        }

        self.group_move_to(group)
    }

    pub fn group_move_prev(&mut self) -> Result<(), Error> {
        let mut group = self.info.group - 1;
        
        if group < 1 {
            group = self.info.group_highest;
        }

        self.group_move_to(group)
    }

    pub fn group_move_to(&mut self, group: i32) -> Result<(), Error> {
        let group = group.clamp(1, self.info.group_highest);

        let output = self.workspaces
            .iter()
            .find(|w| w.num / 10 == group)
            .map(|w| w.output.as_str());
        
        if let Some(output) = output {
            self.connection.run_command(format!(
                "[con_id=__focused__] move container to output {output}, focus"
            ))?;
        }

        let num = group * 10 + self.info.position;

        self.connection.run_command(format!(
            "[con_id=__focused__] move container to workspace number {num}, focus"
        ))?;

        Ok(())
    }

    pub fn reorder_opt(
        connection: &mut Connection,
        workspaces: &[Option<&Workspace>],
        outputs: &[Output],
    ) -> Result<(), Error> {
        let map = outputs
            .iter()
            .enumerate()
            .map(|o| (o.1.name.as_str(), o.0))
            .collect::<HashMap<_, _>>();

        let mut reindex_down = vec![];
        let mut reindex_up = vec![];

        // reindex downwards
        for (w_idx, workspace) in workspaces.iter().enumerate() {
            let Some(workspace) = workspace else {
                continue;
            };
            let o_idx = map
                .get(workspace.output.as_str())
                .expect("workspace should have an output");

            let num = ((o_idx + 1) * 10 + w_idx + 1) as i32;
            let name = workspace.name.trim_start_matches(char::is_numeric);

            let source = if workspace.num < 0 {
                String::new()
            } else {
                workspace.num.to_string()
            };

            match num.cmp(&workspace.num) {
                Ordering::Less => reindex_down.push(format!("rename workspace '{source}{name}' to '{num}{name}'")),
                Ordering::Greater => reindex_up.push(format!("rename workspace '{source}{name}' to '{num}{name}'")),
                _ => {}
            }
        }

        for command in reindex_down.iter().chain(reindex_up.iter().rev()) {
            connection.run_command(command)?;
        }

        Ok(())
    }
}
