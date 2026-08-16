use std::collections::BTreeMap;

use swayipc::{Output, Workspace};

use crate::{NUMBERS_PER_GROUP, POSITIONS_PER_GROUP};

const TEMP_PREFIX: &str = "999";

/// The number a workspace that does not exist yet is relocated from, i.e. one beyond every real one.
const UNNUMBERED: i32 = i32::MAX;

/// A struct to manage the numbering of workspaces.
pub(crate) struct Numberer(BTreeMap<i64, i32>);

impl Numberer {
    /// Number every workspace after the position it holds on its output.
    ///
    /// The workspaces are taken in the order sway reports them.
    pub(crate) fn new(workspaces: &[Workspace], outputs: &[Output]) -> Self {
        let mut numberer = Self(BTreeMap::new());
        let mut group = 1;

        // Outputs are numbered as they are placed, top to bottom and left to right.
        let mut outputs = outputs.iter().collect::<Vec<_>>();
        outputs.sort_by(|left, right| left.rect.y.cmp(&right.rect.y).then(left.rect.x.cmp(&right.rect.x)));

        for output in outputs {
            let mut index = 0;

            for workspace in workspaces.iter().filter(|workspace| workspace.output == output.name) {
                // An output with more workspaces than a group holds continues in the
                // next group, again starting at position 1.
                let num = (group + index / POSITIONS_PER_GROUP) * NUMBERS_PER_GROUP + index % POSITIONS_PER_GROUP + 1;

                numberer.0.insert(workspace.id, num);
                index += 1;
            }

            // Skip every group this output took, so the next one starts on a free group.
            if index > 0 {
                group += (index - 1) / POSITIONS_PER_GROUP + 1;
            }
        }

        numberer
    }

    /// Renumber the workspace at `from` to `to`, shifting everything in between the other way.
    ///
    /// The position `from` gives up is the one `to` takes, so the group keeps its size and this
    /// also fits a group that is already full. A `from` no workspace holds gives up nothing and
    /// grows the group by the position it frees instead.
    pub(crate) fn relocate(&mut self, from: i32, to: i32) -> i32 {
        for ws_num in self.0.values_mut() {
            if *ws_num == from {
                *ws_num = to;
            } else if *ws_num >= to && *ws_num < from {
                *ws_num += 1;
            } else if *ws_num > from && *ws_num <= to {
                *ws_num -= 1;
            }
        }

        to
    }

    /// Free `num` by pushing it and everything after it one position up.
    pub(crate) fn prepend_at(&mut self, num: i32) -> i32 {
        self.relocate(UNNUMBERED, num)
    }

    /// Free the position after `num` by pushing everything after it one position up.
    pub(crate) fn append_at(&mut self, num: i32) -> i32 {
        self.relocate(UNNUMBERED, num + 1)
    }

    /// The commands renaming every workspace that is not numbered as [`Self::new`] determined.
    pub(crate) fn rename_commands(&self, workspaces: &[Workspace]) -> Vec<String> {
        let mut reindex_up = Vec::new();
        let mut reindex_down = Vec::new();

        for workspace in workspaces {
            let Some(&num) = self.0.get(&workspace.id) else {
                continue;
            };

            if workspace.num == num {
                continue;
            }

            let name = workspace.name.trim_start_matches(|char: char| char.is_ascii_digit());

            // A workspace that cannot be addressed would take another one with it, so it rather keeps the number it has.
            let Some(quote) = quote(name) else {
                continue;
            };

            let source = if workspace.num < 0 {
                String::new()
            } else {
                workspace.num.to_string()
            };

            reindex_up.push(format!(
                "rename workspace {quote}{source}{name}{quote} to {quote}{TEMP_PREFIX}{num}{name}{quote}"
            ));
            reindex_down.push(format!(
                "rename workspace {quote}{TEMP_PREFIX}{num}{name}{quote} to {quote}{num}{name}{quote}"
            ));
        }

        reindex_up.append(&mut reindex_down);

        reindex_up
    }
}

/// The quote character `name` has to be wrapped in for a sway command.
///
/// Sway keeps backslashes instead of unescaping them, so a quote can only be avoided
/// rather than escaped, and a trailing odd run of them swallows the closing quote.
fn quote(name: &str) -> Option<char> {
    if name.chars().rev().take_while(|char| *char == '\\').count() % 2 == 1 {
        None
    } else if !name.contains('\'') {
        Some('\'')
    } else if !name.contains('"') {
        Some('"')
    } else {
        None
    }
}
