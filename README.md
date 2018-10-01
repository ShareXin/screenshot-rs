# screenshot-rs

Simple library that allows for simple method of asking for screenshots from various desktops

## Features
- Works with some Wayland desktops
  - GNOME
  - Plasma
  - swaywm
- Works with all X11 desktops
- Choose your own file destination

## Screenshot tools uses
- flameshot (works with GNOME (Wayland/X11), KDE Plasma(Wayland/X11), and other X11 desktops)
- gnome-screenshot (works with GNOME, Unity, Budgie, Cinnamon, etc)
- spectacle (works with KDE Plasma, possibly LXQT)
- swaygrab (works with swaywm)
- scrot (works with anything with an X server, except WSL or Bash for Ubuntu for Windows)

## How it works
Checks either loginctl show-session $(loginctl | grep $(whoami) | awk '{print $1}') -p Type (systemd)
or '$XDG_SESSION_TYPE' for either Wayland or X11, and checks for installed screenshotting applications.

## So simple it doesn't need documentation
### Methods
#### screenshot_area(file: String, freeze: boolean)
- file will be the path and filename you want for your screenshot, in png format
- Takes an area screenshot, meaning you can select an area of your screen to take a screenshot of
- freeze will freeze a user's screen using an available image viewer (feh preferred) so that they can take a screenshot of what they saw that prompted them to take a screenshot
#### screenshot_window(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes an window screenshot, meaning the currently used window will be screenshotted
#### screenshot_full(file: String)
- file will be the path and filename you want for your screenshot, in png format
- Takes a screenshot of an entire screen(s)

## Changelog
### [0.1.0] - 2018-10-01
- First version (created for [ShareXin](https://github.com/ShareXin/ShareXin))
