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

    fn set_max_locations_trip(config: *mut c_void, value: i32);
    fn get_max_locations_trip(config: *mut c_void) -> i32;

    fn set_max_locations_viaroute(config: *mut c_void, value: i32);
    fn get_max_locations_viaroute(config: *mut c_void) -> i32;

    fn set_max_locations_distance_table(config: *mut c_void, value: i32);
    fn get_max_locations_distance_table(config: *mut c_void) -> i32;

    fn set_max_locations_map_matching(config: *mut c_void, value: i32);
    fn get_max_locations_map_matching(config: *mut c_void) -> i32;

    fn set_max_radius_map_matching(config: *mut c_void, value: f64);
    fn get_max_radius_map_matching(config: *mut c_void) -> f64;

    fn set_max_results_nearest(config: *mut c_void, value: i32);
    fn get_max_results_nearest(config: *mut c_void) -> i32;

    fn set_max_alternatives(config: *mut c_void, value: i32);
    fn get_max_alternatives(config: *mut c_void) -> i32;

    fn set_use_shared_memory(config: *mut c_void, value: i32);
    fn get_use_shared_memory(config: *mut c_void) -> i32;

    fn set_memory_file(config: *mut c_void, path: *const c_char);
    fn get_memory_file(config: *mut c_void) -> *const c_char;

    fn set_use_mmap(config: *mut c_void, value: i32);
    fn get_use_mmap(config: *mut c_void) -> i32;

    fn set_algorithm(config: *mut c_void, value: Algorithm);
    fn get_algorithm(config: *mut c_void) -> Algorithm;

    fn set_verbosity(config: *mut c_void, path: *const c_char);
    fn get_verbosity(config: *mut c_void) -> *const c_char;

    fn set_dataset_name(config: *mut c_void, path: *const c_char);
    fn get_dataset_name(config: *mut c_void) -> *const c_char;
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

    pub fn set_max_locations_trip(&mut self, value: i32){
        unsafe {
            set_max_locations_trip(*self.config, value);
        }
    }

    pub fn get_max_locations_trip(&mut self) -> i32{
        unsafe {
            get_max_locations_trip(*self.config)
        }
    }

    pub fn set_max_locations_viaroute(&mut self, value: i32){
        unsafe {
            set_max_locations_viaroute(*self.config, value);
        }
    }

    pub fn get_max_locations_viaroute(&mut self) -> i32{
        unsafe {
            get_max_locations_viaroute(*self.config)
        }
    }

    pub fn set_max_locations_distance_table(&mut self, value: i32){
        unsafe {
            set_max_locations_distance_table(*self.config, value);
        }
    }

    pub fn get_max_locations_distance_table(&mut self) -> i32{
        unsafe {
            get_max_locations_distance_table(*self.config)
        }
    }

    pub fn set_max_locations_map_matching(&mut self, value: i32){
        unsafe {
            set_max_locations_map_matching(*self.config, value);
        }
    }

    pub fn get_max_locations_map_matching(&mut self) -> i32{
        unsafe {
            get_max_locations_map_matching(*self.config)
        }
    }

    pub fn set_max_radius_map_matching(&mut self, value: f64){
        unsafe {
            set_max_radius_map_matching(*self.config, value);
        }
    }

    pub fn get_max_radius_map_matching(&mut self) -> f64{
        unsafe {
            get_max_radius_map_matching(*self.config)
        }
    }

    pub fn set_max_results_nearest(&mut self, value: i32){
        unsafe {
            set_max_results_nearest(*self.config, value);
        }
    }

    pub fn get_max_results_nearest(&mut self) -> i32{
        unsafe {
            get_max_results_nearest(*self.config)
        }
    }

    pub fn set_max_alternatives(&mut self, value: i32){
        unsafe {
            set_max_alternatives(*self.config, value);
        }
    }

    pub fn get_max_alternatives(&mut self) -> i32{
        unsafe {
            get_max_alternatives(*self.config)
        }
    }

    pub fn set_use_shared_memory(&mut self, value: bool){
        unsafe {
            set_use_shared_memory(*self.config, value as i32);
        }
    }

    pub fn get_use_shared_memory(&mut self) -> i32{
        unsafe {
            get_use_shared_memory(*self.config)
        }
    }

    pub fn set_memory_file(&mut self, path: &str){
        unsafe {
            set_memory_file(*self.config, CString::new(path).unwrap().as_ptr());
        }
    }

    pub fn get_memory_file(&mut self) -> &str{
        let c_buf: *const c_char = unsafe {
            get_memory_file(*self.config)
        };

        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };

        c_str.to_str().unwrap()

    }

    pub fn set_use_mmap(&mut self, value: i32){
        unsafe {
            set_use_mmap(*self.config, value);
        }
    }

    pub fn get_use_mmap(&mut self) -> i32{
        unsafe {
            get_use_mmap(*self.config)
        }
    }

    pub fn set_algorithm(&mut self, value: Algorithm){
        unsafe {
            set_algorithm(*self.config, value);
        }
    }

    pub fn get_algorithm(&mut self) -> Algorithm{
        unsafe {
            get_algorithm(*self.config)
        }
    }

    pub fn set_verbosity(&mut self, path: &str){
        unsafe {
            set_verbosity(*self.config, CString::new(path).unwrap().as_ptr());
        }
    }

    pub fn get_verbosity(&mut self) -> &str{
        let c_buf: *const c_char = unsafe {
            get_verbosity(*self.config)
        };

        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };

        c_str.to_str().unwrap()

    }

    pub fn set_dataset_name(&mut self, path: &str){
        unsafe {
            set_dataset_name(*self.config, CString::new(path).unwrap().as_ptr());
        }
    }

    pub fn get_dataset_name(&mut self) -> &str{
        let c_buf: *const c_char = unsafe {
            get_dataset_name(*self.config)
        };

        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };

        c_str.to_str().unwrap()

    }
}

impl Drop for EngineConfig {
    fn drop(&mut self) {
        unsafe {
            engine_config_destroy(*self.config);
        }
    }
}