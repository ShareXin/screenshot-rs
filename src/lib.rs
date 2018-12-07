// Simple library that allows for simple method of asking for screenshots from various Linux/BSD desktops

use std::env;
use std::fs;
use std::process::Command;

static SELECTION_TEMPORARY_FILE: &str = "/tmp/selection-tmp.png";

pub enum ScreenshotKind {
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

pub fn screenshot_area(file: String, freeze: bool) {
    match screenshot_tool_selection(session_type()) {
        DesktopKind::GNOME => gnome(ScreenshotKind::Area, file, freeze),
        DesktopKind::KDE => kde(ScreenshotKind::Area, file),
        DesktopKind::X11 => scrot(ScreenshotKind::Area, file, freeze),
    }
}
pub fn screenshot_window(file: String) {
    match screenshot_tool_selection(session_type()) {
        DesktopKind::GNOME => gnome(ScreenshotKind::Window, file, false),
        DesktopKind::KDE => kde(ScreenshotKind::Window, file),
        DesktopKind::X11 => scrot(ScreenshotKind::Window, file, false),
    }
}
pub fn screenshot_full(file: String) {
    match screenshot_tool_selection(session_type()) {
        DesktopKind::GNOME => gnome(ScreenshotKind::Full, file, false),
        DesktopKind::KDE => kde(ScreenshotKind::Full, file),
        DesktopKind::X11 => scrot(ScreenshotKind::Full, file, false),
    }
}
fn gnome(option: ScreenshotKind, file: String, freeze: bool) {
    match option {
        ScreenshotKind::Area => {
            let mut feh = match Command::new("feh").arg("--version").spawn() {
                Ok(_) => {
                    Command::new("gnome-screenshot")
                        .args(&["-f", SELECTION_TEMPORARY_FILE])
                        .output()
                        .expect("gnome-screenshot did not launch");
                    Command::new("feh")
                        .args(&[SELECTION_TEMPORARY_FILE, "-F"])
                        .spawn()
                        .expect("'feh' did not launch to pause screen for selection")
                }
                Err(_) => Command::new("sh")
                    .arg("-c")
                    .arg("echo Feh does not exist")
                    .spawn()
                    .unwrap(),
            };
            Command::new("gnome-screenshot")
                .args(&["-a", "-f", &file])
                .output()
                .expect("gnome-screenshot did not launch");
            if freeze {
                match fs::remove_file(SELECTION_TEMPORARY_FILE) {
                    Ok(ok) => ok,
                    Err(_) => eprintln!("Unable to remove temporary selection file"),
                };
                match feh.kill() {
                    Ok(ok) => ok,
                    Err(_) => eprintln!("Unable to kill feh, must have already been closed"),
                };
            }
        }
        ScreenshotKind::Window => {
            Command::new("gnome-screenshot")
                .args(&["-w", "-e", "shadow", "-f", &file])
                .output()
                .expect("gnome-screenshot did not launch");
        }
        ScreenshotKind::Full => {
            Command::new("gnome-screenshot")
                .args(&["-f", &file])
                .output()
                .expect("gnome-screenshot did not launch");
        }
    };
}
fn kde(option: ScreenshotKind, file: String) {
    match option {
        ScreenshotKind::Area => {
            Command::new("spectacle")
                .args(&["-rbno,", &file])
                .output()
                .expect("spectacle did not launch");
        }
        ScreenshotKind::Window => {
            Command::new("spectacle")
                .args(&["-abno", &file])
                .output()
                .expect("spectacle did not launch");
        }
        ScreenshotKind::Full => {
            Command::new("spectacle")
                .args(&["-fbno", &file])
                .output()
                .expect("spectacle did not launch");
        }
    };
}
fn scrot(option: ScreenshotKind, file: String, freeze: bool) {
    match option {
        ScreenshotKind::Area => {
            let mut feh = match Command::new("feh").arg("--version").spawn() {
                Ok(_) => {
                    Command::new("scrot")
                        .arg(SELECTION_TEMPORARY_FILE)
                        .output()
                        .expect("scrot did not launch");
                    Command::new("feh")
                        .args(&[SELECTION_TEMPORARY_FILE, "-F"])
                        .spawn()
                        .expect("'feh' did not launch to pause screen for selection")
                }
                Err(_) => Command::new("sh")
                    .arg("-c")
                    .arg("echo Feh does not exist")
                    .spawn()
                    .unwrap(),
            };
            Command::new("scrot")
                .args(&["--select", &file])
                .output()
                .expect("scrot did not launch");
            if freeze {
                match fs::remove_file(SELECTION_TEMPORARY_FILE) {
                    Ok(ok) => ok,
                    Err(_) => eprintln!("Unable to remove temporary selection file"),
                };
                match feh.kill() {
                    Ok(ok) => ok,
                    Err(_) => eprintln!("Unable to kill feh, must have already been closed"),
                };
            }
        }
        ScreenshotKind::Window => {
            Command::new("scrot")
                .args(&["--border", "--focused", &file])
                .output()
                .expect("gnome-screenshot did not launch");
        }
        ScreenshotKind::Full => {
            Command::new("scrot")
                .args(&[&file])
                .output()
                .expect("gnome-screenshot did not launch");
        }
    };
}
