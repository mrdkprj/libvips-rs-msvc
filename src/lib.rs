#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#[macro_use]
extern crate num_derive;
extern crate num_traits;

pub mod bindings;
pub mod error;
mod image;
mod image_ext;
pub mod operator;
pub mod ops;
pub mod utils;
pub mod voption;

use error::Error;
pub use image::*;
use std::ffi::*;
pub type Result<T> = std::result::Result<T, error::Error>;

pub struct Vips;

/// That's the main type of this crate. Use it to initialize the system
impl Vips {
    pub fn init(name: &str, detect_leak: bool) -> Result<()> {
        let cstring = utils::new_c_string(name);
        if let Ok(c_name) = cstring {
            let res = unsafe { bindings::vips_init(c_name.as_ptr()) };
            let result = if res == 0 {
                Ok(())
            } else {
                Err(Error::InitializationError("Failed to init libvips"))
            };
            unsafe {
                if detect_leak {
                    bindings::vips_leak_set(1);
                };
            }
            result
        } else {
            Err(Error::InitializationError("Failed to convert rust string to C string"))
        }
    }

    pub fn progress_set(flag: bool) {
        unsafe {
            bindings::vips_progress_set(if flag { 1 } else { 0 });
        }
    }

    pub fn get_disc_threshold() -> u64 {
        unsafe { bindings::vips_get_disc_threshold() }
    }

    pub fn version_string() -> Result<String> {
        unsafe {
            let version = CStr::from_ptr(bindings::vips_version_string());
            let version_str = version
                .to_str()
                .map_err(|_| Error::InitializationError("Error initializing string"))?;
            Ok(version_str.to_string())
        }
    }

    pub fn thread_shutdown() {
        unsafe {
            bindings::vips_thread_shutdown();
        }
    }

    pub fn error_buffer() -> Result<String> {
        unsafe {
            let buffer = CStr::from_ptr(bindings::vips_error_buffer());
            let buffer_str = buffer
                .to_str()
                .map_err(|_| Error::InitializationError("Error initializing string"))?;
            Ok(buffer_str.to_string())
        }
    }

    pub fn error(domain: &str, error: &str) -> Result<()> {
        unsafe {
            let c_str_error = utils::new_c_string(error)?;
            let c_str_domain = utils::new_c_string(domain)?;
            bindings::vips_error(
                c_str_domain.as_ptr(),
                c_str_error.as_ptr(),
            );
            Ok(())
        }
    }

    pub fn error_system(code: i32, domain: &str, error: &str) -> Result<()> {
        unsafe {
            let c_str_error = utils::new_c_string(error)?;
            let c_str_domain = utils::new_c_string(domain)?;
            bindings::vips_error_system(
                code,
                c_str_domain.as_ptr(),
                c_str_error.as_ptr(),
            );
            Ok(())
        }
    }

    pub fn freeze_error_buffer() {
        unsafe {
            bindings::vips_error_freeze();
        }
    }

    pub fn error_clear() {
        unsafe {
            bindings::vips_error_clear();
        }
    }

    pub fn error_thaw() {
        unsafe {
            bindings::vips_error_thaw();
        }
    }

    pub fn error_exit(error: &str) -> Result<()> {
        unsafe {
            let c_str_error = utils::new_c_string(error)?;
            bindings::vips_error_exit(c_str_error.as_ptr());
        }
    }

    pub fn cache_print() {
        unsafe {
            bindings::vips_cache_print();
        }
    }

    pub fn cache_set_max(max: i32) {
        unsafe {
            bindings::vips_cache_set_max(max);
        }
    }

    pub fn cache_set_max_mem(max: u64) {
        unsafe {
            bindings::vips_cache_set_max_mem(max);
        }
    }

    pub fn cache_get_max() -> i32 {
        unsafe { bindings::vips_cache_get_max() }
    }

    pub fn cache_get_max_mem() -> u64 {
        unsafe { bindings::vips_cache_get_max_mem() }
    }

    pub fn cache_get_size() -> i32 {
        unsafe { bindings::vips_cache_get_size() }
    }

    pub fn cache_set_max_files(max: i32) {
        unsafe {
            bindings::vips_cache_set_max_files(max);
        }
    }

    pub fn cache_get_max_files() -> i32 {
        unsafe { bindings::vips_cache_get_max_files() }
    }

    pub fn vips_cache_set_dump(flag: bool) {
        unsafe {
            bindings::vips_cache_set_dump(if flag { 1 } else { 0 });
        }
    }

    pub fn vips_cache_set_trace(flag: bool) {
        unsafe {
            bindings::vips_cache_set_trace(if flag { 1 } else { 0 });
        }
    }

    /// set the number of worker threads for vips to operate
    pub fn concurrency_set(max: i32) {
        unsafe {
            bindings::vips_concurrency_set(max);
        }
    }

    /// get the number of worker threads that vips is operating
    pub fn concurrency_get() -> i32 {
        unsafe { bindings::vips_concurrency_get() }
    }

    pub fn tracked_get_mem() -> u64 {
        unsafe { bindings::vips_tracked_get_mem() }
    }

    pub fn tracked_get_mem_highwater() -> u64 {
        unsafe { bindings::vips_tracked_get_mem_highwater() }
    }

    pub fn tracked_get_allocs() -> i32 {
        unsafe { bindings::vips_tracked_get_allocs() }
    }

    pub fn pipe_read_limit_set(limit: i64) {
        unsafe {
            bindings::vips_pipe_read_limit_set(limit);
        }
    }

    pub fn shutdown() {
        unsafe {
            bindings::vips_shutdown();
        }
    }
}
