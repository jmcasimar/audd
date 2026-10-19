//! Deterministic time support for reproducible outputs.
//!
//! By default `now()` returns `Utc::now()`. When deterministic mode is enabled,
//! all timestamps emitted by the resolution engine and the CLI use a fixed
//! instant, making byte-for-byte reproducible runs possible.

use chrono::{DateTime, Utc};
use std::sync::OnceLock;

static FIXED_TIME: OnceLock<Option<DateTime<Utc>>> = OnceLock::new();

/// Return the current timestamp.
///
/// If deterministic mode has been enabled, returns the fixed timestamp;
/// otherwise returns `Utc::now()`.
pub fn now() -> DateTime<Utc> {
    FIXED_TIME.get().and_then(|t| *t).unwrap_or_else(Utc::now)
}

/// Enable deterministic mode using the Unix epoch as the fixed timestamp.
pub fn enable_deterministic_mode() {
    let _ = FIXED_TIME.set(Some(DateTime::UNIX_EPOCH));
}

/// Set a specific fixed timestamp for deterministic mode.
pub fn set_fixed_time(time: DateTime<Utc>) {
    let _ = FIXED_TIME.set(Some(time));
}

/// Disable deterministic mode and return to wall-clock time.
pub fn disable_deterministic_mode() {
    let _ = FIXED_TIME.set(None);
}

/// Returns true if deterministic mode is currently enabled.
pub fn is_deterministic() -> bool {
    FIXED_TIME.get().and_then(|t| *t).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_mode() {
        let fixed = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        set_fixed_time(fixed);
        assert_eq!(now(), fixed);
        assert!(is_deterministic());
    }
}
