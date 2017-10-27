//! Keystone Assembler Engine (www.keystone-engine.org) */
//! By Nguyen Anh Quynh <aquynh@gmail.com>, 2016 */
//! Rust bindings by Remco Verhoef <remco@dutchcoders.io>, 2016 */
//!

extern crate libc;
#[macro_use]
extern crate bitflags;

pub mod keystone_const;

use std::fmt;
use std::ffi::CStr;
use std::os::raw::c_char;
use keystone_const::{Arch, Error, Mode, OptionType, OptionValue};

#[allow(non_camel_case_types)]
pub type ks_handle = libc::size_t;

#[link(name = "keystone")]
extern "C" {
    pub fn ks_version(major: *const u32, minor: *const u32) -> u32;
    pub fn ks_arch_supported(arch: Arch) -> bool;
    pub fn ks_open(arch: Arch, mode: Mode, engine: *mut ks_handle) -> Error;
    pub fn ks_asm(
        engine: ks_handle,
        string: *const c_char,
        address: u64,
        encoding: *mut *mut libc::c_uchar,
        encoding_size: *mut libc::size_t,
        stat_count: *mut libc::size_t,
    ) -> u32;
    pub fn ks_errno(engine: ks_handle) -> Error;
    pub fn ks_strerror(error_code: Error) -> *const c_char;
    pub fn ks_option(engine: ks_handle, opt_type: OptionType, value: OptionValue) -> Error;
    pub fn ks_close(engine: ks_handle);
    pub fn ks_free(encoding: *mut libc::c_uchar);
}

impl Error {
    pub fn msg(&self) -> String {
        error_msg(*self)
    }
}

/// Return a string describing given error code.
pub fn error_msg(error: Error) -> String {
    unsafe {
        CStr::from_ptr(ks_strerror(error))
            .to_string_lossy()
            .into_owned()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg())
    }
}
