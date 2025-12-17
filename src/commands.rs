use std::process::Command;

use arboard::{Clipboard, ImageData};
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSURL;

use crate::{config::Config, utils::get_time_since_epoch};

#[derive(Debug, Clone)]
pub enum Function {
    OpenApp(String),
    RunShellCommand(Vec<String>),
    RandomVar(i32),
    GoogleSearch(String),
    OpenPrefPane,
    Quit,
}

impl Function {
    pub fn execute(&self, config: &Config) {
        match self {
            Function::OpenApp(path) => {
                NSWorkspace::new().openURL(&NSURL::fileURLWithPath(
                    &objc2_foundation::NSString::from_str(path),
                ));
            }
            Function::RunShellCommand(shell_command) => {
                Command::new("sh")
                    .arg("-c")
                    .arg(shell_command.join(" "))
                    .status()
                    .ok();
            }
            Function::RandomVar(var) => {
                Clipboard::new()
                    .unwrap()
                    .set_text(var.to_string())
                    .unwrap_or(());
            }

            Function::GoogleSearch(query_string) => {
                let query_args = query_string.replace(" ", "+");
                let query = config.search_url.replace("%s", &query_args);
                NSWorkspace::new().openURL(
                    &NSURL::URLWithString_relativeToURL(
                        &objc2_foundation::NSString::from_str(&query),
                        None,
                    )
                    .unwrap(),
                );
            }

            Function::OpenPrefPane => {
                Command::new("open")
                    .arg(
                        std::env::var("HOME").unwrap_or("".to_string())
                            + "/.config/rustcast/config.toml",
                    )
                    .spawn()
                    .ok();
            }
            Function::Quit => std::process::exit(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClipboardContent {
    pub content_type: ClipBoardContentType,
    pub copied_since_epoch: i64,
}

#[derive(Debug, Clone)]
pub enum ClipBoardContentType {
    Text(String),
    Image(ImageData<'static>),
}

impl PartialEq for ClipBoardContentType {
    fn eq(&self, other: &Self) -> bool {
        if let Self::Text(a) = self
            && let Self::Text(b) = other
        {
            return a == b;
        } else if let Self::Image(image_data) = self
            && let Self::Image(other_image_data) = other
        {
            return image_data.bytes == other_image_data.bytes;
        }
        false
    }
}

impl ClipboardContent {
    pub fn from_content_type(content_type: ClipBoardContentType) -> ClipboardContent {
        Self {
            content_type,
            copied_since_epoch: get_time_since_epoch(),
        }
    }
}
