use std::cmp::Ordering;
use std::collections::BTreeMap;

use swayipc::{Output, Workspace};

/// A struct to manage the numbering of workspaces.
pub struct Numberer(BTreeMap<i64, i32>);

impl Numberer {
    pub fn new(workspaces: &[Workspace], outputs: &[Output]) -> Self {
        let mut numberer = Self(BTreeMap::new());

        let mut group = 1;

        for output in Self::rect_ordered_outputs(outputs) {
            for (w_idx, workspace) in workspaces.iter().filter(|ws| ws.output == output.name).enumerate() {
                let position = w_idx + 1;

                // additional groups per output should also start with a position of 1: position / 10
                let num = group * 10 + position + (position / 10);

                numberer.0.insert(workspace.id, num as i32);
            }

            group = (numberer.0.values().max().unwrap_or(&0) / 10 + 1) as usize;
        }

        numberer
    }

    pub fn prepend_at(&mut self, num: i32) -> i32 {
        for (_, ws_num) in self.0.iter_mut() {
            if *ws_num >= num {
                *ws_num += 1;
            }
        }

        num
    }

    pub fn append_at(&mut self, num: i32) -> i32 {
        for (_, ws_num) in self.0.iter_mut() {
            if *ws_num > num {
                *ws_num += 1;
            }
        }

        num + 1
    }

    pub fn reorder(&self, connection: &mut swayipc::Connection) -> Result<(), swayipc::Error> {
        let mut reindex_up = vec![];
        let mut reindex_down = vec![];

        for workspace in connection.get_workspaces()? {
            let Some(num) = self.0.get(&workspace.id) else {
                continue;
            };
            let name = workspace.name.trim_start_matches(char::is_numeric);

            let source = if workspace.num < 0 {
                String::new()
            } else {
                workspace.num.to_string()
            };

            reindex_up.push(format!("rename workspace '{source}{name}' to '1{num}{name}'"));
            reindex_down.push(format!("rename workspace '1{num}{name}' to '{num}{name}'"));
        }

        let command = reindex_up.iter()
            .chain(reindex_down.iter())
            .map(|s| &**s)
            .collect::<Vec<&str>>()
            .join("; ");

        if !command.is_empty() {
            connection.run_command(command)?;
        }

        Ok(())
    }

    fn rect_ordered_outputs(outputs: &[Output]) -> Vec<&Output> {
        let mut outputs = outputs.iter().collect::<Vec<_>>();

        outputs.sort_by(|o1, o2| match o1.rect.y.cmp(&o2.rect.y) {
            Ordering::Equal => o1.rect.x.cmp(&o2.rect.x),
            x => x,
        });

        outputs
    }
}
