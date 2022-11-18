use std::{ffi::CString, os::raw::{c_char, c_double, c_int}};

use crate::{Algorithm, Boolean};

#[repr(C)]
pub(crate) struct CEngineConfig {
    pub storage_config: *const c_char,
    pub max_locations_trip: c_int,
    pub max_locations_viaroute: c_int,
    pub max_locations_distance_table: c_int,
    pub max_locations_map_matching: c_int,
    pub max_radius_map_matching: c_double,
    pub max_results_nearest: c_int,
    pub max_alternatives: c_int,
    pub use_shared_memory: Boolean,
    pub memory_file: *const c_char,
    pub use_mmap: Boolean,
    pub algorithm: Algorithm,
    pub verbosity: *const c_char,
    pub dataset_name: *const c_char,
}

impl CEngineConfig {
    pub(crate) fn new(path: &CString) -> CEngineConfig {
        CEngineConfig {
            storage_config: path.as_ptr(),
            max_locations_trip: -1,
            max_locations_viaroute: -1,
            max_locations_distance_table: -1,
            max_locations_map_matching: -1,
            max_radius_map_matching: -1.0,
            max_results_nearest: -1,
            max_alternatives: 3,
            use_shared_memory: Boolean::TRUE,
            memory_file: std::ptr::null(),
            use_mmap: Boolean::TRUE,
            algorithm: Algorithm::CH,
            verbosity: std::ptr::null(),
            dataset_name: std::ptr::null(),
        }
    }
}