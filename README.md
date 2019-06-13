# screenshot-rs

Simple library that allows for simple method of asking for screenshots from various Linux/BSD desktops

## Features
- Works with some Wayland desktops
  - Plasma
  - GNOME
  - Sway
- Works with all X11 desktops (with *scrot* installed as fallback)

## Screenshot tools required (at least one)
- spectacle (works with KDE Plasma, possibly LXQT)
- gnome-screenshot (works with GNOME, Unity, Budgie, Cinnamon, etc)
- scrot (works with anything with an X server, except WSL or Bash for Ubuntu for Windows)

### Freezing screen for area screenshots
Uses *feh* if available to take a screenshot of the full screen, open it, and select an area of that screenshot to freeze the screen

## How it works
Checks '$XDG_SESSION_TYPE' for either Wayland or X11, and checks for available screenshotting applications.

### Methods
#### screenshot_area(file: String, freeze: Bool)
- file will be the path and filename you want for your screenshot, in png format
- Takes an area screenshot, meaning you can select an area of your screen to take a screenshot of
- If 'freeze' is true, then it will use *feh* to freeze the screen in place to take an area screenshot
#### screenshot_window(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes an window screenshot, meaning the currently used window will be screenshotted
#### screenshot_full(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes a screenshot of an entire screen(s)

## Changelog
### [0.1.5] - 2019-06-13
- Experimental Sway support

### [0.1.4] - 2019-06-09
- Experimental macOS Support

### [0.1.3] - 2019-06-09
- A typo

### [0.1.2] - 2018-12-07
- *feh* is no longer a requirement for "area" screenshots, was used to "freeze" the screen

### [0.1.1] - 2018-10-03
- Made enum ScreenshotKind public

### [0.1.0] - 2018-10-01
- First version (created for [ShareXin](https://github.com/ShareXin/ShareXin))
