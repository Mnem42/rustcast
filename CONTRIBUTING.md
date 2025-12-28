# Welcome to the RustCast contributing guide!

Thank you for wanting to contribute to RustCast!

There are 2 areas you can work on:

1. Bug fixes
1. New Features
1. Help people in solving their github issues

For bug fixes, and helping people to solve their github issues: see
[https://github.com/unsecretised/rustcast/issues] For features, see
[The Planned Features in the README](README.md)

## Code Guidelines:

1. All code must be formatted with `cargo fmt`
1. Code must not be malicious or be intended to harm someones device
1. All features added must work as intended
1. Code must compile...
1. A video recording / screenshot would be an added bonus in getting your pull
   request merged faster.

## Codebase:

```
.
├── bundling # Bundling related stuff, ignore for the most bit
│   ├── entitlements.plist
│   ├── icon.icns
│   └── Info.plist
├── docs # Website and documentation related stuff. If something new is added to config, then modify this as well before PR-ing
├── Cargo.lock 
├── Cargo.toml
├── CONTRIBUTING.md
├── EXTENSIONS.md
├── LICENSE.md
├── README.md
└── src
    ├── app.rs        # Main app logic
    ├── calculator.rs # Calculator logic 
    ├── commands.rs   # Logic for different commands
    ├── config.rs     # Configuration related stuff
    ├── macos.rs      # Macos specific config
    ├── main.rs       # Start app
    └── utils.rs      # Common functions that are used across files
```
