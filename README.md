# screenshot-rs

Simple library that allows for simple method of asking for screenshots from various Linux/BSD desktops

## Features
- Works with some Wayland desktops
  - GNOME
  - Plasma
- Works with all X11 desktops (with *scrot* installed as fallback)

## Screenshot tools uses
- gnome-screenshot (works with GNOME, Unity, Budgie, Cinnamon, etc)
- spectacle (works with KDE Plasma, possibly LXQT)
- scrot (works with anything with an X server, except WSL or Bash for Ubuntu for Windows)

## How it works
Checks '$XDG_SESSION_TYPE' for either Wayland or X11, and checks for installed screenshotting applications.

### Methods
#### screenshot_area(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes an area screenshot, meaning you can select an area of your screen to take a screenshot of
#### screenshot_window(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes an window screenshot, meaning the currently used window will be screenshotted
#### screenshot_full(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes a screenshot of an entire screen(s)

## Changelog
### [0.1.1] - 2018-10-03
- Made enum ScreenshotKind public

### [0.1.0] - 2018-10-01
- First version (created for [ShareXin](https://github.com/ShareXin/ShareXin))
