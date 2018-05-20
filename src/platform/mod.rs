#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use platform::linux::apply_theme;
