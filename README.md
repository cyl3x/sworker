# sworker

Workspace utility and manager for sway.  
sworker groups all workspaces according to their output with an increment of 10.
All workspaces in a group are indexed in order starting from 1.

`sworker` is also compatible with sworkstyle.

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
```

</details>

## Focus commands
With `sworker focus` it is possible to focus a workspace in the current group.  
Valid values are `next`, `prev` or a number from `1-9`.  
If `next` or `prev` is given, the focus will be wrapped at the start or end.
Before wrapping, a empty workspace is created if the focused workspace is not empty.

With `sworker focus-group` it is possible to focus another group.  
Valid values are `next`, `prev` or a number from `1-9`. 
If `next` or `prev` is given, the focus will be wrapped at the start or end.

## Move commands
With `sworker move` it is possible to move the focused window in the current group.  
Valid values are `next`, `prev` or a number from `1-9`.  
If `next` or `prev` is given, the window will be wrapped at the start or end.
Before wrapping, a new workspace is created if the focused window isn't alone in it's workspace.

With `sworker move-group` it is possible to move the focused window to another group.  
Valid values are `next`, `prev` or a number from `1-9`. 
If `next` or `prev` is given, the window will be wrapped at the start or end.

## Daemon
Starting the daemon with `sworker daemon` will continuously reorder all workspaces.

# Similar tools
- [swaysome](https://gitlab.com/hyask/swaysome) by skia
- [sway-workspace-manager](https://github.com/oati/sway-workspace-manager) by oati
