//! Keystone Assembler Engine (www.keystone-engine.org) */
//! By Nguyen Anh Quynh <aquynh@gmail.com>, 2016 */
//! Rust bindings by Remco Verhoef <remco@dutchcoders.io>, 2016 */
//!
//! ```rust
//! extern crate keystone;
//! use keystone::{Keystone, Arch, OptionType};
//!
//! fn main() {
//!     let engine = Keystone::new(Arch::X86, keystone::MODE_32)
//!         .expect("Could not initialize Keystone engine");
//!     engine.option(OptionType::SYNTAX, keystone::OPT_SYNTAX_NASM)
//!         .expect("Could not set option to nasm syntax");
//!     let result = engine.asm("mov ah, 0x80".to_string(), 0)
//!         .expect("Could not assemble");
//! }
//! ```

#![doc(html_root_url="https://keystone/doc/here/v1")]

extern crate libkeystone_sys as ffi;
extern crate libc;

use std::fmt;
use std::ffi::CString;

pub use keystone_const::*;
pub use ffi::ks_handle;
pub use ffi::keystone_const;

#[derive(Debug, PartialEq)]
pub struct AsmResult {
    pub size: u32,
    pub stat_count: u32,
    pub bytes: Vec<u8>,
}

impl fmt::Display for AsmResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in &self.bytes {
            try!(f.write_fmt(format_args!("{:02x}", byte)));
        }

        Ok(())
    }
}

pub fn bindings_version() -> (u32, u32) {
    (API_MAJOR, API_MINOR)
}

/// Return tuple `(major, minor)` API version numbers.
pub fn version() -> (u32, u32) {
    let mut major: u32 = 0;
    let mut minor: u32 = 0;

    unsafe {
        ffi::ks_version(&mut major, &mut minor);
    }
    (major, minor)
}

/// Return tuple `(major, minor)` API version numbers.
pub fn arch_supported(arch: Arch) -> bool {
    unsafe { ffi::ks_arch_supported(arch) }
}

pub struct Keystone {
    handle: ks_handle,
}

impl Keystone {
    /// Create new instance of Keystone engine.
    pub fn new(arch: Arch, mode: Mode) -> Result<Keystone, Error> {
        if version() != bindings_version() {
            return Err(Error::VERSION);
        }

        let mut handle: ks_handle = 0;

        let err = unsafe { ffi::ks_open(arch, mode, &mut handle) };
        if err == Error::OK {
            Ok(Keystone { handle: handle })
        } else {
            Err(err)
        }
    }

    /// Report the last error number when some API function fail.
    pub fn error(&self) -> Result<(), Error> {
        let err = unsafe { ffi::ks_errno(self.handle) };
        if err == Error::OK { Ok(()) } else { Err(err) }
    }

    /// Set option for Keystone engine at runtime
    pub fn option(&self, option_type: OptionType, value: OptionValue) -> Result<(), Error> {
        let err = unsafe { ffi::ks_option(self.handle, option_type, value) };
        if err == Error::OK { Ok(()) } else { Err(err) }
    }

    /// Assemble a string given its the buffer, size, start address and number
    /// of instructions to be decoded.
    ///
    /// This API dynamically allocate memory to contain assembled instruction.
    /// Resulted array of bytes containing the machine code  is put into @*encoding
    pub fn asm(&self, str: String, address: u64) -> Result<AsmResult, Error> {
        let mut size: libc::size_t = 0;
        let mut stat_count: libc::size_t = 0;

        let s = CString::new(str).unwrap();
        let mut ptr: *mut libc::c_uchar = std::ptr::null_mut();

        let err = Error::from_bits_truncate(unsafe {
            ffi::ks_asm(
                self.handle,
                s.as_ptr(),
                address,
                &mut ptr,
                &mut size,
                &mut stat_count,
            )
        });

        if err == Error::OK {
            let bytes = unsafe { std::slice::from_raw_parts(ptr, size) };

            unsafe {
                ffi::ks_free(ptr);
            };

            Ok(AsmResult {
                size: size as u32,
                stat_count: stat_count as u32,
                bytes: From::from(&bytes[..]),
            })
        } else {
            let err = unsafe { ffi::ks_errno(self.handle) };
            Err(err)
        }
    }
}

impl Drop for Keystone {
    fn drop(&mut self) {
        unsafe { ffi::ks_close(self.handle) };
    }
}
