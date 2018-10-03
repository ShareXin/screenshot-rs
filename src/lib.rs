// Simple library that allows for simple method of asking for screenshots from various Linux/BSD desktops

use std::env;
use std::fs;
use std::process::Command;

static SELECTION_TEMPORARY_FILE: &str = "/tmp/selection_tmp";

enum ScreenshotKind {
    Area,
    Window,
    Full,
}

enum SessionKind {
    Wayland,
    X11,
}

enum DesktopKind {
    GNOME,
    KDE,
    X11,
}

fn session_type() -> SessionKind {
    return match env::var("XDG_SESSION_TYPE") {
        Ok(ok) => match ok.to_lowercase().as_ref() {
            "wayland" => SessionKind::Wayland,
            _ => SessionKind::X11,
        },
        Err(_) => SessionKind::X11,
    };
}

fn screenshot_tool_selection(session: SessionKind) -> DesktopKind {
    return match session {
        SessionKind::Wayland => match Command::new("gnome-screenshot").arg("--version").spawn() {
            Ok(_) => DesktopKind::GNOME,
            Err(_) => match Command::new("spectacle").arg("--version").spawn() {
                Ok(_) => DesktopKind::KDE,
                Err(_) => panic!("Uncompatible Wayland desktop"),
            },
        },
        SessionKind::X11 => match Command::new("gnome-screenshot").arg("--version").spawn() {
            Ok(_) => DesktopKind::GNOME,
            Err(_) => match Command::new("spectacle").arg("--version").spawn() {
                Ok(_) => DesktopKind::KDE,
                Err(_) => match Command::new("scrot").arg("--version").spawn() {
                    Ok(_) => DesktopKind::X11,
                    Err(_) => panic!("Uncompatible X11 desktop (install scrot)"),
                },
            },
        },
    };
}

pub fn screenshot_area(file: String) {
    match screenshot_tool_selection(session_type()) {
        DesktopKind::GNOME => gnome(ScreenshotKind::Area, file),
        DesktopKind::KDE => kde(ScreenshotKind::Area, file),
        DesktopKind::X11 => scrot(ScreenshotKind::Area, file),
    }
}
pub fn screenshot_window(file: String) {
    match screenshot_tool_selection(session_type()) {
        DesktopKind::GNOME => gnome(ScreenshotKind::Window, file),
        DesktopKind::KDE => kde(ScreenshotKind::Window, file),
        DesktopKind::X11 => scrot(ScreenshotKind::Window, file),
    }
}
pub fn screenshot_full(file: String) {
    match screenshot_tool_selection(session_type()) {
        DesktopKind::GNOME => gnome(ScreenshotKind::Full, file),
        DesktopKind::KDE => kde(ScreenshotKind::Full, file),
        DesktopKind::X11 => scrot(ScreenshotKind::Full, file),
    }
}
fn gnome(option: ScreenshotKind, file: String) {
    match option {
        ScreenshotKind::Area => {
            Command::new("gnome-screenshot")
                .args(&["-f,", &SELECTION_TEMPORARY_FILE])
                .status()
                .expect("gnome-screenshot did not launch");
            let mut feh = Command::new("feh")
                .args(&[&SELECTION_TEMPORARY_FILE, "-F"])
                .spawn()
                .expect("'feh' did not launch to pause screen for selection");
            Command::new("gnome-screenshot")
                .args(&["-a", "-f", &file])
                .status()
                .expect("gnome-screenshot did not launch");
            fs::remove_file(SELECTION_TEMPORARY_FILE)
                .expect("Unable to remove temporary selection file");
            match feh.kill() {
                Ok(ok) => ok,
                Err(_) => println!("Unable to kill feh, must have already been closed")
            };
        }
        ScreenshotKind::Window => {
            Command::new("gnome-screenshot")
                .args(&["-w", "-e", "shadow", "-f", &file])
                .status()
                .expect("gnome-screenshot did not launch");
        }
        ScreenshotKind::Full => {
            Command::new("gnome-screenshot")
                .args(&["-f", &file])
                .status()
                .expect("gnome-screenshot did not launch");
        }
    };
}
fn kde(option: ScreenshotKind, file: String) {
    match option {
        ScreenshotKind::Area => {
            Command::new("spectacle")
                .args(&["-rbno,", &file])
                .status()
                .expect("spectacle did not launch");
        }
        ScreenshotKind::Window => {
            Command::new("spectacle")
                .args(&["-abno", &file])
                .status()
                .expect("spectacle did not launch");
        }
        ScreenshotKind::Full => {
            Command::new("spectacle")
                .args(&["-fbno", &file])
                .status()
                .expect("spectacle did not launch");
        }
    };
}
fn scrot(option: ScreenshotKind, file: String) {
    match option {
        ScreenshotKind::Area => {
            Command::new("scrot")
                .args(&[&SELECTION_TEMPORARY_FILE])
                .status()
                .expect("scrot did not launch");
            let mut feh = Command::new("feh")
                .args(&[&SELECTION_TEMPORARY_FILE, "-F"])
                .spawn()
                .expect("'feh' did not launch to pause screen for selection");
            Command::new("scrot")
                .args(&["--select", &file])
                .status()
                .expect("scrot did not launch");
            fs::remove_file(SELECTION_TEMPORARY_FILE)
                .expect("Unable to remove temporary selection file");
            match feh.kill() {
                Ok(ok) => ok,
                Err(_) => println!("Unable to kill feh, must have already been closed")
            };
        }
        ScreenshotKind::Window => {
            Command::new("scrot")
                .args(&["--border", "--focused", &file])
                .status()
                .expect("gnome-screenshot did not launch");
        }
        ScreenshotKind::Full => {
            Command::new("scrot")
                .args(&[&file])
                .status()
                .expect("gnome-screenshot did not launch");
        }
    };
}
