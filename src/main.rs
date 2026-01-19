mod app;
mod calculator;
mod clipboard;
mod commands;
mod config;
#[cfg(target_os = "macos")]
mod styles;
mod unit_conversion;
mod utils;

mod cross_platform;

// import from utils
use crate::utils::{create_config_file_if_not_exists, get_config_file_path, read_config_file};

use crate::app::tile::Tile;

use global_hotkey::GlobalHotKeyManager;

fn main() -> iced::Result {
    #[cfg(target_os = "macos")]
    cross_platform::macos::set_activation_policy_accessory();

    let file_path = get_config_file_path();
    let config = read_config_file(&file_path).unwrap();
    create_config_file_if_not_exists(&file_path, &config).unwrap();

    let manager = GlobalHotKeyManager::new().unwrap();

    let show_hide = config.toggle_hotkey.parse().unwrap();

    // Hotkeys are stored as a vec so that hyperkey support can be added later
    let hotkeys = vec![show_hide];

    manager
        .register_all(&hotkeys)
        .expect("Unable to register hotkey");

    iced::daemon(
        move || Tile::new(show_hide, &config),
        Tile::update,
        Tile::view,
    )
    .subscription(Tile::subscription)
    .theme(Tile::theme)
    .run()
}
