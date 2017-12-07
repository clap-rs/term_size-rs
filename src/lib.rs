// The following was originally taken and adapated from exa source
// repo: https://github.com/ogham/exa
// commit: b9eb364823d0d4f9085eb220233c704a13d0f611
// license: MIT - Copyright (c) 2014 Benjamin Sago

//! System calls for getting the terminal size.
//!
//! Getting the terminal size is performed using an ioctl command that takes
//! the file handle to the terminal -- which in this case, is stdout -- and
//! populates a structure containing the values.
//!
//! The size is needed when the user wants the output formatted into columns:
//! the default grid view, or the hybrid grid-details view.

#[cfg(not(target_os = "windows"))]
extern crate libc;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate winapi;

// A facade to allow exposing functions depending on the platform
mod platform;
pub use platform::{dimensions, dimensions_stdout, dimensions_stdin, dimensions_stderr};