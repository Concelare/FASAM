
/// The logging tiers
///
/// # Tiers
/// * `Info` - Information that is not an error, warning or debug
/// * `Warning` - Any warnings that do not break the system but could affect something
/// * `Debug` - Developer useful information
/// * `Error` - Any errors that have occurred and could have caused something to break
///
/// # Usage
/// ```
/// use crate::log_tier::LogTier;
///
/// LogTier::Info
/// ```
#[derive(PartialEq, Eq)]
pub enum LogTier {
    Info,
    Warning,
    Debug,
    Error
}