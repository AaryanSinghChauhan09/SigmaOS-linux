//! SigmaOS linux component.
//!
//! Rust implementation of `archlinux/linux`: the Arch Linux kernel
//! packaging tree — kernel Makefile version handling, `.config` fragment
//! merging, and kernel package metadata.
//!
//! Upstream is the full kernel git mirror + Arch PKGBUILD packaging.
//! This port implements the packaging-side logic: kernel version
//! extraction from `Makefile`, config fragment application, and the
//! kernel pkgver/pkgrel model Arch uses (`6.x.y` + `pkgrel`).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod config;
pub mod error;
pub mod version;

pub use config::{ConfigFragment, apply_fragments, parse_fragment};
pub use error::KernelError;
pub use version::{KernelVersion, kernel_version_from_makefile, pkgrel_bump};
