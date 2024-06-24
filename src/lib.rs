//! Provides constant functions to compute for `ioctl(2)` identifiers.
//!
//! Currently, this only supports Linux. The long term goal is to support `ioctl` identifiers for other Unix-like
//! operating systems, including FreeBSD and macOS.
//!
//! ## Usage Notes
//! [`IoctlId`] is an alias for the type to pass into the `ioctl(2)` request. This is either a `u32` or `u64`, depending
//! on the target OS and architecture.
//!
//! The [`io()`], [`ior()`], [`iow()`], and [`iowr()`] functions take a type parameter, similar to the `_IO()`,
//! `_IOR()`, `_IOW()`, and `_IOWR()` macros.
//!
//! For example, the following C code:
//! ```c
//! struct my_ioctl_data {
//!     unsigned int a;
//! };
//!
//! #define MY_IOCTL _IOR(0x12, 0x34, struct my_ioctl_data)
//! ```
//!
//! Would be written in Rust as:
//! ```rust
//! use ioctl_id::*;
//!
//! #[repr(C)]
//! struct MyIoctlData {
//!    a: u32,
//! }
//!
//! const MY_IOCTL: IoctlId = ior::<MyIoctlData>(0x12, 0x34);
//! ```
#![no_std]
#![warn(missing_docs)]

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(all(
    test,
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "powerpc",
        target_arch = "sparc",
        target_arch = "sparc64"
    ))
))]
mod ccompat_tests_generic;
