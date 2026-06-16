# swb021bk_driver

yo, this is just a small rust thing to make my cheap tablet usable on linux
it reads evdev input, remaps buttons, and lets you switch modes from a tray icon.

## what it does
- grabs the tablet input
- remaps buttons to actions (undo, redo, brush, erase etc)
- supports presets per app (kinda WIP)
- tray icon to enable/disable/switch stuff

## supported apps
- Rnote
- Libresprite
- Krita

## notes
- linux only
- requires input group for user
- any wm/de
