# sworker

Workspace utility and manager for sway.  
sworker groups all workspaces according to their output with an increment of 10.
All workspaces in a group are indexed in order starting from 1.

<details><summary><b>Example sway config</b></summary>

```bash
# Move focus between workspaces
bindsym Mod4+1 exec sworker focus 1
bindsym Mod4+2 exec sworker focus 2
bindsym Mod4+3 exec sworker focus 3
bindsym Mod4+4 exec sworker focus 4
bindsym Mod4+5 exec sworker focus 5
bindsym Mod4+6 exec sworker focus 6
bindsym Mod4+7 exec sworker focus 7
bindsym Mod4+8 exec sworker focus 8
bindsym Mod4+9 exec sworker focus 9

# Move containers between workspaces
bindsym Mod4+Shift+1 exec sworker move 1
bindsym Mod4+Shift+2 exec sworker move 2
bindsym Mod4+Shift+3 exec sworker move 3
bindsym Mod4+Shift+4 exec sworker move 4
bindsym Mod4+Shift+5 exec sworker move 5
bindsym Mod4+Shift+6 exec sworker move 6
bindsym Mod4+Shift+7 exec sworker move 7
bindsym Mod4+Shift+8 exec sworker move 8
bindsym Mod4+Shift+9 exec sworker move 9

# Move focus between outputs
bindsym Mod4+Ctrl+1 exec sworker focus-group 1
bindsym Mod4+Ctrl+2 exec sworker focus-group 2
bindsym Mod4+Ctrl+3 exec sworker focus-group 3
bindsym Mod4+Ctrl+4 exec sworker focus-group 4
bindsym Mod4+Ctrl+5 exec sworker focus-group 5
bindsym Mod4+Ctrl+6 exec sworker focus-group 6
bindsym Mod4+Ctrl+7 exec sworker focus-group 7
bindsym Mod4+Ctrl+8 exec sworker focus-group 8
bindsym Mod4+Ctrl+9 exec sworker focus-group 9

# Move containers to other outputs
bindsym Mod4+Alt+1 exec sworker move-group 1
bindsym Mod4+Alt+2 exec sworker move-group 2
bindsym Mod4+Alt+3 exec sworker move-group 3
bindsym Mod4+Alt+4 exec sworker move-group 4
bindsym Mod4+Alt+5 exec sworker move-group 5
bindsym Mod4+Alt+6 exec sworker move-group 6
bindsym Mod4+Alt+7 exec sworker move-group 7
bindsym Mod4+Alt+8 exec sworker move-group 8
bindsym Mod4+Alt+9 exec sworker move-group 9

bindsym Mod4+Ctrl+Left exec sworker focus prev
bindsym Mod4+Ctrl+Right exec sworker focus next
bindsym Mod4+Ctrl+Up exec sworker focus-group next
bindsym Mod4+Ctrl+Down exec sworker focus-group prev

bindsym Mod4+Alt+Left exec sworker move prev
bindsym Mod4+Alt+Right exec sworker move next
bindsym Mod4+Alt+Up exec sworker move-group next
bindsym Mod4+Alt+Down exec sworker move-group prev

bindsym Mod4+Prior exec sworker move prev
bindsym Mod4+Next exec sworker move next
bindsym Mod4+Shift+Prior exec sworker move-group next
bindsym Mod4+Shift+Next exec sworker move-group prev

# Insert a new workspace after or before the focused one
bindsym Mod4+plus exec sworker focus --new next
bindsym Mod4+minus exec sworker focus --new prev
bindsym Mod4+Shift+plus exec sworker move --new next
bindsym Mod4+Shift+minus exec sworker move --new prev
```

</details>

## How it works
`sworker` is focused on dynamic workspace creation and stable group and workspace numbers.

A workspace number is read as group and position: `23` is the third workspace of the second group.

### Groups and outputs
Every output gets its own group, in the order the outputs are arranged: top to bottom, then left to right.
A group reserves ten numbers, so group 1 holds `11` to `19`, group 2 holds `21` to `29`, and so on.
Groups are never created by hand, they come and go with the outputs.

`sworker focus-group` focuses another group, `sworker move-group` moves the focused window to it.
Both take `next`, `prev` or a number from `1-9`:

- `next` and `prev` wrap around at the first and last group
- a number higher than the last group selects the last one, no group is created
- the position inside the group is kept, so going from `12` to group 3 lands on `32`
- if that position does not exist in the target group yet, it is created at its end

Since a group belongs to an output, both commands are also the way to switch screens.

#### When an output goes away
`sworker` never puts more than nine workspaces in a group, sway can when an output is disconnected.
Sway moves its workspaces to a remaining output, filling leftover workspaces of that output.
If the workspaces moved exceed nine, an additional group is added, therefore no workspaces are squashed together.

Two outputs, five workspaces on the first and six on the second:

```
   output 1            output 2
[11] .. [15]         [21] .. [26]
   group 1              group 2
```

After unplugging output 2, all eleven workspaces sit on output 1:

```
              output 1
[11] .. [19]          [21][22]
   group 1             group 2
```

Nothing about this is permanent.
As soon as the workspaces are spread over the outputs again, the numbering closes up and every output is back to a single group.

### Workspaces
Inside a group the workspaces are numbered `1` to `9` in the order sway lists them, without gaps.
Because of this, `sworker focus 2` always means "the second workspace of the current group".
It does not matter which output is focused or how many workspaces exist, so the same keybind keeps doing the same thing.

Workspaces are not set up in advance, they are created as soon as a position is requested that isn't there yet.
Sway removes a workspace again once its last window is gone, and `sworker` renumbers everything that follows, so the numbering closes up and stays gap-free.

`sworker focus` focuses a workspace in the current group, `sworker move` moves the focused window to it.
Both take `next`, `prev` or a number from `1-9`:

- a number higher than the current workspace count creates a new workspace at the end
- `next` and `prev` wrap around at the first and last workspace
- before wrapping, a new workspace is created instead: `focus` does so if the current workspace is not empty, `move` if the window isn't alone in it
- `--new` inserts a new workspace at the target position instead of using the one already there, pushing that workspace and every one after it one position up

A group is limited to the nine positions `x1` to `x9`.
Once it is full, no further workspace is inserted and the one already at the target position is used instead.

### Names are kept
Only the leading number of a workspace name is rewritten, the rest is left untouched.
That is what makes `sworker` work alongside tools that name workspaces themselves, such as [sworkstyle](https://github.com/Lyr-7D1h/swayest_workstyle).

## Daemon
Starting the daemon with `sworker daemon` will continuously reorder all workspaces.

# Similar tools
- [swaysome](https://gitlab.com/hyask/swaysome) by skia
- [sway-workspace-manager](https://github.com/oati/sway-workspace-manager) by oati
