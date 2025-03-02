use std::cmp::Ordering;
use std::collections::BTreeMap;

use swayipc::{Output, Workspace};

/// A struct to manage the numbering of workspaces.
pub struct Numberer(BTreeMap<i64, i32>);

impl Numberer {
    pub fn new(workspaces: &[Workspace], outputs: &[Output]) -> Self {
        let mut numberer = Self(BTreeMap::new());

        for (o_idx, output) in outputs.iter().enumerate() {
            for (w_idx, workspace) in workspaces.iter().filter(|ws| ws.output == output.name).enumerate() {
                let num = (o_idx + 1) * 10 + (w_idx + 1);
                numberer.0.insert(workspace.id, num as i32);
            }
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
        let mut reindex_down = vec![];
        let mut reindex_up = vec![];

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
