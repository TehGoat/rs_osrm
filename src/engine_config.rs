#![allow(dead_code)]

use std::os::raw::{c_void, c_char};
use std::ffi::{CString, CStr};

#[link(name = "c_osrm")]
extern {
    fn engine_config_create() -> *mut c_void;
    fn engine_config_destroy(config: *mut c_void);

    fn engine_config_is_valid(config: *mut c_void) -> i32;

    fn set_storage_config(config: *mut c_void, path: *const c_char);
    fn get_storage_config(config: *mut c_void) -> *const c_char;

    fn set_use_shared_memory(config: *mut c_void, value: i32);
    fn get_use_shared_memory(config: *mut c_void) -> i32;

    fn set_algorithm(config: *mut c_void, value: Algorithm);
    fn get_algorithm(config: *mut c_void) -> Algorithm;
}

#[repr(u8)]
pub enum Algorithm {
    CH = 0,
    CoreCH = 1, // Deprecated, will be removed in v6.0
    MLD = 2
}

pub struct EngineConfig
{
    pub config: Box<*mut c_void>
}

impl EngineConfig {
    pub fn new() -> EngineConfig {
        unsafe {
            EngineConfig {
                config: Box::new(engine_config_create())
            }
        }
    }

    pub fn engine_config_is_valid(&mut self) -> bool {
        unsafe {
            engine_config_is_valid(*self.config) == 1
        }
    }

    pub fn set_storage_config(&mut self, path: &str){
        unsafe {
            set_storage_config(*self.config, CString::new(path).unwrap().as_ptr());
        }
    }

    pub fn get_storage_config(&mut self) -> &str{
        let c_buf: *const c_char = unsafe {
            get_storage_config(*self.config)
        };

        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };

        c_str.to_str().unwrap()

    }

    pub fn set_use_shared_memory(&mut self, value: bool){
        unsafe {
            set_use_shared_memory(*self.config, value as i32);
        }
    }

    pub fn set_algorithm(&mut self, value: Algorithm){
        unsafe {
            set_algorithm(*self.config, value);
        }
    }
}

impl Drop for EngineConfig {
    fn drop(&mut self) {
        unsafe {
            engine_config_destroy(*self.config);
        }
    }
}