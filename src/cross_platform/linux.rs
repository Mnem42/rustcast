use std::{fs, path::Path, process::Command, thread};

use freedesktop_desktop_entry::DesktopEntry;
use glob::glob;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    app::{
        apps::{App, AppCommand},
        tile::elm::default_app_paths,
    },
    config::Config,
};

pub fn get_installed_linux_apps(config: &Config) -> Vec<App> {
    let paths = default_app_paths();
    let store_icons = config.theme.show_icons;

    let apps: Vec<App> = paths
        .par_iter()
        .map(|path| get_installed_apps_glob(path, store_icons))
        .flatten()
        .collect();
    //index_dirs_from_config(&mut apps);

    apps
}

fn get_installed_apps_glob(path: &str, store_icons: bool) -> Vec<App> {
    if path.contains("*") {
        glob(path)
            .unwrap()
            .flatten()
            .flat_map(|entry| get_installed_apps(entry.to_str().unwrap(), store_icons))
            .collect()
    } else {
        get_installed_apps(path, store_icons)
    }
}

fn get_installed_apps(path: &str, store_icons: bool) -> Vec<App> {
    let mut apps = Vec::new();
    let dir = Path::new(path);

    if !dir.exists() || !dir.is_dir() {
        return apps;
    }

    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
            continue;
        }

        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        let Ok(de) = DesktopEntry::from_str(path.as_path(), &content, None::<&[String]>) else {
            continue;
        };

        if de.no_display() || de.hidden() {
            continue;
        }

        let Some(name) = de.desktop_entry("Name") else {
            continue;
        };
        let desc = de.desktop_entry("Comment").unwrap_or("");
        let Some(exec) = de.exec() else { continue };

        let exec = exec.to_string();
        let mut parts = exec.split_whitespace().filter(|p| !p.starts_with("%"));

        let Some(cmd) = parts.next() else { continue };

        let args = parts.map(str::to_owned).collect::<Vec<_>>().join(" ");

        // TODO: load icons
        let icon = if store_icons {
            de.icon().map(str::to_owned)
        } else {
            None
        };

        apps.push(App {
            icons: None,
            name: name.to_string(),
            name_lc: name.to_lowercase(),
            desc: desc.to_string(),
            open_command: AppCommand::Function(crate::commands::Function::RunShellCommand(
                cmd.to_string(),
                args,
            )),
        });
    }

    apps
}

pub fn open_url(url: &str) {
    let url = url.to_owned();
    thread::spawn(move || {
        Command::new("xdg-open").arg(url).spawn().unwrap();
    });
}
