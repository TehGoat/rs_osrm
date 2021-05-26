#![allow(dead_code)]

use core::fmt::Display;
use std::ffi::CStr;
use std::{
    fmt,
    os::raw::{c_char, c_void},
};

use engine_config::c_engine_config::CEngineConfig;

pub mod engine_config;
pub mod general;
pub mod match_api;
pub mod nearest;
pub mod route;
pub mod table;
pub mod tile;
pub mod trip;

#[link(name = "c_osrm")]
extern "C" {
    fn osrm_create(config: *const CEngineConfig, return_value: *mut *mut COSRM);
    fn osrm_destroy_error_message(error_message: *const c_char);
    fn osrm_destroy(osrm: *mut c_void);
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COSRM {
    pub(crate) obj: *mut c_void,
    pub(crate) error_message: *mut c_char,
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Status {
    Ok = 0,
    Error = 1,
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        if self.eq(&Status::Ok) {
            write!(f, "Ok")
        } else {
            write!(f, "Error")
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub enum Algorithm {
    CH = 0,
    CoreCH = 1, // Deprecated, will be removed in v6.0
    MLD = 2,
}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub enum Boolean {
    FALSE = 0,
    TRUE = 1,
}

impl Boolean {
    fn from(value: bool) -> Boolean {
        match value {
            true => Boolean::TRUE,
            false => Boolean::FALSE,
        }
    }
}

pub struct Osrm {
    config: Box<*mut c_void>,
}

impl Osrm {
    pub(crate) fn new(c_engine_config: CEngineConfig) -> Result<Osrm, String> {
        unsafe {
            let mut result: *mut COSRM = std::ptr::null_mut();
            let result_ptr: *mut *mut COSRM = &mut result;
            osrm_create(&c_engine_config as *const CEngineConfig, result_ptr);

            if (*result).error_message != std::ptr::null_mut() {
                let c_name_buf: *const c_char = (*result).error_message;
                let c_name_str: &CStr = CStr::from_ptr(c_name_buf);

                match c_name_str.to_str() {
                    Ok(ok) => {
                        let name_str_slice = ok.to_string();

                        osrm_destroy_error_message((*result).error_message);

                        return Err(name_str_slice);
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            }

            Ok(Osrm {
                config: Box::new((*result).obj),
            })
        }
    }
}

impl Drop for Osrm {
    fn drop(&mut self) {
        unsafe {
            osrm_destroy(*self.config);
        }
    }
}

unsafe impl Send for Osrm {}

#[cfg(test)]
mod tests {
    use crate::{Osrm, engine_config::engine_config_builder::EngineConfigBuilder, nearest::NearestRequest};

    #[test]
    fn it_works() {
        let osrm = EngineConfigBuilder::new("/home/ronny/osrm/sweden-latest.osrm")
            .set_algorithm(crate::Algorithm::MLD)
            .set_use_shared_memory(false)
            .build();

        assert_eq!(osrm.is_ok(), true);

        asd(osrm.ok().unwrap());
    }


    fn asd(osrm: Osrm) {
        let mut asd = NearestRequest::new(53.342, 14.234234);
        let asd = asd.run(&osrm);

        print!("{}\n{:?}", asd.0, asd.1);
        
    }
}
