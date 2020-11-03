#[cfg(windows)]
pub mod audio;

pub mod commands;
pub mod data;
pub mod graphics;
pub mod logging;

pub use logging::StrResult;
