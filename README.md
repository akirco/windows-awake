# Windows Power Manager

[中文说明](/README_ZH_CN.md)

A Rust library for managing Windows system power states, including sleep, hibernation, and wake prevention.

<!-- [![Crates.io](https://img.shields.io/crates/v/windows_awake)](https://crates.io/crates/windows_awake)
[![Documentation](https://docs.rs/windows_awake/badge.svg)](https://docs.rs/windows_awake)
[![License](https://img.shields.io/crates/l/windows_awake)](LICENSE) -->

## Features

- 🔒 Prevent system sleep and display timeout
- ⏰ Schedule system sleep with countdown
- 💤 Force system sleep or hibernation
- 🔄 Restore default power settings
- ⚡ Low-level Windows power management API integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
windows-awake = {git="https://github.com/akirco/windows-awake.git"，branch = "master"}
```

## Quick Start

```rust
use windows_awake::PowerManager;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let power_manager = PowerManager::new();

    // Keep system awake for 30 minutes
    power_manager.keep_awake_for_minutes(30)?;

    // Or keep system awake indefinitely
    power_manager.keep_awake_indefinite()?;

    // Put system to sleep immediately
    power_manager.force_sleep(false, false, false)?;

    Ok(())
}
```

## Usage Examples

### Keep System Awake

```rust
use windows_awake::PowerManager;

let power_manager = PowerManager::new();

// Prevent system sleep indefinitely
power_manager.keep_awake_indefinite()?;

// Keep system awake for specific duration (in minutes)
power_manager.keep_awake_for_minutes(60)?; // Keep awake for 1 hour
```

### Control System Sleep States

```rust
use windows_awake::PowerManager;

let power_manager = PowerManager::new();

// Normal sleep mode
power_manager.force_sleep(false, false, false)?;

// Hibernation mode
power_manager.force_sleep(true, false, false)?;

// Force sleep (bypass application blocks)
power_manager.force_sleep(false, true, true)?;
```

### Restore Default Settings

```rust
use windows_awake::PowerManager;

let power_manager = PowerManager::new();

// Restore system default power settings
power_manager.restore_default()?;
```

## Interactive Demo

The library includes an interactive demo application that showcases all features. Run it using:

```bash
cargo run --example power_management_demo
```

## API Reference

### PowerManager

- `new()` - Create a new PowerManager instance
- `keep_awake_indefinite()` - Prevent system sleep indefinitely
- `keep_awake_for_minutes(minutes: u32)` - Prevent sleep for specified duration
- `force_sleep(hibernate: bool, force: bool, disable_wake_events: bool)` - Control system sleep state
- `restore_default()` - Restore default power settings

## Error Handling

The library uses a custom `PowerError` enum for error handling:

```rust
pub enum PowerError {
    WindowsError(windows::core::Error),
    InvalidDuration(String),
    SleepError(String),
}
```

## Requirements

- Windows operating system
- Rust 1.56 or later
- Administrator privileges may be required for some operations

## Common Issues and Solutions

1. **Sleep/Hibernate Not Working**
   - Ensure your system has hibernation enabled
   - Run the application with administrator privileges
   - Check if other applications are blocking sleep state

2. **Permission Errors**
   - Run the application as administrator
   - Check Windows power settings

3. **Wake Prevention Not Working**
   - Verify no other applications are modifying power settings
   - Check system power policy settings

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Version History

- 0.1.0 (2025-02-21)
  - Initial release
  - Basic power management features
  - Interactive demo application

## Credits

Created by akirco on 2025-02-21

## Support

If you encounter any issues or need help, please:
1. Check the [Issues](https://github.com/akirco/windows-awake/issues) page
2. Create a new issue if your problem isn't already listed
3. Include your Windows version and Rust version when reporting issues

## Safety Notes

⚠️ **Important:**
- Always save your work before forcing sleep or hibernation
- Be cautious with `force_sleep` when using `force = true`
- Consider system-wide impacts when preventing sleep for long periods