//! Low-level bindings to libsystemd (and similar) libraries
//!
//! Items in this module correspond to systemd functions/types that are documented by the systemd
//! (`sd_*`) man pages.

#![warn(rust_2018_idioms)]

pub use libc::{clockid_t, gid_t, pid_t, siginfo_t, signalfd_siginfo, size_t, uid_t};
pub use std::os::raw::{c_char, c_int, c_uint, c_void};

pub mod daemon;
pub mod event;
pub mod id128;
#[cfg(feature = "journal")]
pub mod journal;
pub mod login;

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct const_iovec {
    pub iov_base: *const c_void,
    pub iov_len: size_t,
}

pub fn array_to_iovecs(args: &[&str]) -> Vec<const_iovec> {
    args.iter()
        .map(|d| const_iovec {
            iov_base: d.as_ptr() as *const c_void,
            iov_len: d.len() as size_t,
        })
        .collect()
}

#[cfg(feature = "bus")]
pub mod bus;
