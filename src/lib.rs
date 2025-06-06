#![warn(
    clippy::err_expect
)]

pub mod io;
pub mod error;
pub mod version;
pub mod raw_repr;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "ui")]
pub mod ui;

#[cfg(feature="async")]
pub mod task_util;