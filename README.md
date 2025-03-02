# sworker

Workspace utility and manager for sway.  
sworker groups all workspaces according to their output with an increment of 10.
All workspaces in a group are indexed in order starting from 1.

`sworker` is also compatible with sworkstyle.

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
