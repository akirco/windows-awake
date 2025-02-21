use std::{ thread, time::Duration };
use thiserror::Error;
use windows::{
    Win32::System::Power::{
        SetThreadExecutionState,
        SetSuspendState,
        ES_CONTINUOUS,
        ES_DISPLAY_REQUIRED,
        ES_SYSTEM_REQUIRED,
    },
    core::Error as WindowsError,
};

#[derive(Error, Debug)]
pub enum PowerError {
    #[error("Windows API error: {0}")] WindowsError(#[from] WindowsError),
    #[error("Invalid duration: {0}")] InvalidDuration(String),
    #[error("Failed to initiate system sleep: {0}")] SleepError(String),
}

#[derive(Debug, Default)]
pub struct PowerManager;

impl PowerManager {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// force system to sleep or hibernate
    ///
    /// # Arguments
    ///
    /// * `hibernate` - true is hibernate, false is sleep
    /// * `force` - true is force, false is allow applications to prevent
    /// * `disable_wake_events` - true is disable wake events
    pub fn force_sleep(
        &self,
        hibernate: bool,
        force: bool,
        disable_wake_events: bool
    ) -> Result<(), PowerError> {
        self.restore_default()?;

        thread::sleep(Duration::from_millis(100));

        // call SetSuspendState impl sleep or hibernate
        let result = unsafe {
            SetSuspendState(
                hibernate, // TRUE is hibernate, FALSE is sleep
                force, // TRUE is force, FALSE is allow applications to prevent
                disable_wake_events // TRUE is disable wake events
            )
        };

        if !result {
            Err(PowerError::SleepError("Failed to initiate sleep state".into()))
        } else {
            Ok(())
        }
    }

    /// keep system awake indefinitely
    ///
    /// This function will keep the system awake indefinitely until the `restore_default` function is called.
    pub fn keep_awake_indefinite(&self) -> Result<(), PowerError> {
        let state = ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED;

        let result = unsafe { SetThreadExecutionState(state) };
        if result.0 == 0 {
            Err(WindowsError::from_win32().into())
        } else {
            Ok(())
        }
    }

    /// keep system awake for a specified number of minutes
    ///
    /// param minutes: number of minutes to keep system awake
    pub fn keep_awake_for_minutes(&self, minutes: u32) -> Result<(), PowerError> {
        if minutes == 0 {
            return self.force_sleep(false, false, false);
        }

        if minutes > 1440 {
            return Err(PowerError::InvalidDuration("Duration cannot exceed 24 hours".to_string()));
        }

        self.keep_awake_indefinite()?;

        let seconds = u64::from(minutes) * 60;
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(seconds));
            let power_manager = PowerManager::new();
            let _ = power_manager.force_sleep(false, false, false);
        });

        Ok(())
    }

    /// restore default power management settings
    pub fn restore_default(&self) -> Result<(), PowerError> {
        let result = unsafe { SetThreadExecutionState(ES_CONTINUOUS) };
        if result.0 == 0 {
            Err(WindowsError::from_win32().into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_management() {
        let power_manager = PowerManager::new();

        assert!(power_manager.keep_awake_indefinite().is_ok());
        assert!(power_manager.restore_default().is_ok());

        assert!(power_manager.keep_awake_for_minutes(1).is_ok());
        thread::sleep(Duration::from_secs(65));

        assert!(power_manager.keep_awake_for_minutes(1441).is_err());
    }
}
