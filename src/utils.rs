//! This has all the utility functions that rustcast uses
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
    thread,
};

use iced::widget::image::Handle;
use icns::IconFamily;
use image::RgbaImage;

use crate::{app::App, commands::Function};
#[cfg(target_os = "macos")]
use {
    crate::macos::get_installed_macos_apps, objc2_app_kit::NSWorkspace, objc2_foundation::NSURL,
    std::os::unix::fs::PermissionsExt,
};
#[cfg(target_os = "windows")]
use {crate::windows::get_installed_windows_apps, std::process::Command};
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSURL;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    app::apps::{App, AppCommand},
    commands::Function,
};

/// The default error log path (works only on unix systems, and must be changed for windows
/// support)
const ERR_LOG_PATH: &str = "/tmp/rustscan-err.log";

/// This logs an error to the error log file
pub(crate) fn log_error(msg: &str) {
    if let Ok(mut file) = File::options().create(true).append(true).open(ERR_LOG_PATH) {
        let _ = file.write_all(msg.as_bytes()).ok();
    }
}

/// This logs an error to the error log file, and exits the program
pub(crate) fn log_error_and_exit(msg: &str) {
    log_error(msg);
    exit(-1)
}

/// This converts an icns file to an iced image handle
pub(crate) fn handle_from_icns(path: &Path) -> Option<Handle> {
    let data = std::fs::read(path).ok()?;
    let family = IconFamily::read(std::io::Cursor::new(&data)).ok()?;

    let icon_type = family.available_icons();

    let icon = family.get_icon_with_type(*icon_type.first()?).ok()?;
    let image = RgbaImage::from_raw(
        icon.width() as u32,
        icon.height() as u32,
        icon.data().to_vec(),
    )?;
    Some(Handle::from_rgba(
        image.width(),
        image.height(),
        image.into_raw(),
    ))
}

/// This gets all the installed apps in the given directory
///
/// the directories are defined in [`crate::app::tile::Tile::new`]
pub(crate) fn get_installed_apps(dir: impl AsRef<Path>, store_icons: bool) -> Vec<App> {
    let entries: Vec<_> = fs::read_dir(dir.as_ref())
        .unwrap_or_else(|x| {
            log_error_and_exit(&x.to_string());
            exit(-1)
        })
        .filter_map(|x| x.ok())
        .collect();

    entries
        .into_par_iter()
        .filter_map(|x| {
            let file_type = x.file_type().unwrap_or_else(|e| {
                log_error(&e.to_string());
                exit(-1)
            });
            if !file_type.is_dir() {
                return None;
            }

            let file_name_os = x.file_name();
            let file_name = file_name_os.into_string().unwrap_or_else(|e| {
                log_error(e.to_str().unwrap_or(""));
                exit(-1)
            });
            if !file_name.ends_with(".app") {
                return None;
            }

            let path = x.path();
            let path_str = path.to_str().map(|x| x.to_string()).unwrap_or_else(|| {
                log_error("Unable to get file_name");
                exit(-1)
            });

            let icons = if store_icons {
                match fs::read_to_string(format!("{}/Contents/Info.plist", path_str)).map(
                    |content| {
                        let icon_line = content
                            .lines()
                            .scan(false, |expect_next, line| {
                                if *expect_next {
                                    *expect_next = false;
                                    // Return this line to the iterator
                                    return Some(Some(line));
                                }

                                if line.trim() == "<key>CFBundleIconFile</key>" {
                                    *expect_next = true;
                                }

                                // For lines that are not the one after the key, return None to skip
                                Some(None)
                            })
                            .flatten() // remove the Nones
                            .next()
                            .map(|x| {
                                x.trim()
                                    .strip_prefix("<string>")
                                    .unwrap_or("")
                                    .strip_suffix("</string>")
                                    .unwrap_or("")
                            });

                        handle_from_icns(Path::new(&format!(
                            "{}/Contents/Resources/{}",
                            path_str,
                            icon_line.unwrap_or("AppIcon.icns")
                        )))
                    },
                ) {
                    Ok(Some(a)) => Some(a),
                    _ => {
                        // Fallback method
                        let direntry = fs::read_dir(format!("{}/Contents/Resources", path_str))
                            .into_iter()
                            .flatten()
                            .filter_map(|x| {
                                let file = x.ok()?;
                                let name = file.file_name();
                                let file_name = name.to_str()?;
                                if file_name.ends_with(".icns") {
                                    Some(file.path())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<PathBuf>>();

                        if direntry.len() > 1 {
                            let icns_vec = direntry
                                .iter()
                                .filter(|x| x.ends_with("AppIcon.icns"))
                                .collect::<Vec<&PathBuf>>();
                            handle_from_icns(icns_vec.first().unwrap_or(&&PathBuf::new()))
                        } else if !direntry.is_empty() {
                            handle_from_icns(direntry.first().unwrap_or(&PathBuf::new()))
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            };

            let name = file_name.strip_suffix(".app").unwrap().to_string();
            Some(App {
                open_command: AppCommand::Function(Function::OpenApp(path_str)),
                desc: "Application".to_string(),
                icons,
                name_lc: name.to_lowercase(),
                name,
            })
        })
        .collect()
}

pub fn open_settings() {
    thread::spawn(move || {
        NSWorkspace::new().openURL(&NSURL::fileURLWithPath(
            &objc2_foundation::NSString::from_str(
                &(std::env::var("HOME").unwrap_or("".to_string())
                    + "/.config/rustcast/config.toml"),
            ),
        ));
    });
}

pub fn open_url(url: &str) {
    let url = url.to_owned();
    thread::spawn(move || {
        NSWorkspace::new().openURL(
            &NSURL::URLWithString_relativeToURL(&objc2_foundation::NSString::from_str(&url), None)
                .unwrap(),
        );
    });
}

pub fn is_valid_url(s: &str) -> bool {
    s.ends_with(".com")
        || s.ends_with(".net")
        || s.ends_with(".org")
        || s.ends_with(".edu")
        || s.ends_with(".gov")
        || s.ends_with(".io")
        || s.ends_with(".co")
        || s.ends_with(".me")
        || s.ends_with(".app")
        || s.ends_with(".dev")
}

pub fn get_config_installation_dir() -> String {
    if cfg!(target_os = "windows") {
        std::env::var("LOCALAPPDATA").unwrap()
    } else {
        std::env::var("HOME").unwrap()
    }
}

pub fn get_config_file_path() -> String {
    let home = get_config_installation_dir();

    if cfg!(target_os = "windows") {
        home + "\\rustcast\\config.toml"
    } else {
        home + "/.config/rustcast/config.toml"
    }
}
use crate::config::Config;

pub fn read_config_file(file_path: &str) -> Result<Config, std::io::Error> {
    let config: Config = match std::fs::read_to_string(file_path) {
        Ok(a) => toml::from_str(&a).unwrap(),
        Err(_) => Config::default(),
    };

    Ok(config)
}

pub fn create_config_file_if_not_exists(
    file_path: &str,
    config: &Config,
) -> Result<(), std::io::Error> {
    // check if file exists
    if let Ok(exists) = std::fs::metadata(file_path)
        && exists.is_file()
    {
        return Ok(());
    }

    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::write(
        file_path,
        toml::to_string(&config).unwrap_or_else(|x| x.to_string()),
    )
    .unwrap();

    Ok(())
}

pub fn open_application(path: &str) {
    thread::spawn(move || {
        #[cfg(target_os = "windows")]
        {
            println!("Opening application: {}", path);

            Command::new("powershell")
                .arg(format!("Start-Process '{}'", path))
                .status()
                .ok();
        }

        #[cfg(target_os = "macos")]
        {
            NSWorkspace::new().openURL(&NSURL::fileURLWithPath(
                &objc2_foundation::NSString::from_str(path),
            ));
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open").arg(path).status().ok();
        }
    };
}

pub fn index_dirs_from_config(apps: &mut Vec<App>) -> bool {
    let path = get_config_file_path();
    let config = read_config_file(&path);

    // if config is not valid return false otherwise unwrap config so it is usable
    let config = match config {
        Ok(config) => config,
        Err(err) => {
            println!("Error reading config file: {}", err);
            return false;
        }
    };

    if config.index_dirs.is_empty() {
        return false;
    }

    config.index_dirs.clone().iter().for_each(|dir| {
        // check if dir exists
        if !Path::new(dir).exists() {
            println!("Directory {} does not exist", dir);
            return;
        }

        let paths = fs::read_dir(dir).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let metadata = fs::metadata(&path).unwrap();

            #[cfg(target_os = "windows")]
            let is_executable =
                metadata.is_file() && path.extension().and_then(|s| s.to_str()) == Some("exe");

            #[cfg(target_os = "macos")]
            let is_executable = {
                (metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0))
                    || path.extension().and_then(|s| s.to_str()) == Some("app")
            };

            if is_executable {
                let display_name = path.file_name().unwrap().to_string_lossy().to_string();
                apps.push(App {
                    open_command: Function::OpenApp(path.to_string_lossy().to_string()),
                    name: display_name.clone(),
                    name_lc: display_name.clone().to_lowercase(),
                    icons: None,
                });
            }
        }
    });

    true
}

pub fn get_installed_apps(config: &Config) -> Vec<App> {
    #[cfg(target_os = "macos")]
    {
        get_installed_macos_apps(config)
    }

    #[cfg(target_os = "windows")]
    {
        get_installed_windows_apps()
    }
}
