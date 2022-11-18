use std::os::raw::{c_double, c_int};

use crate::{Osrm, Status, general::{c_structs::c_general_options::CGeneralOptions, rs_structs::general_options::GeneralOptions}};

use super::{Annotations, FallbackCoordinate, table_result::{CTableResult, TableResult}, table_result_destroy, osrm_table};

#[repr(C)]
pub(crate) struct CTableRequest {
    general_options: CGeneralOptions,
    sources: *const c_int,
    number_of_sources: c_int,
    destinations: *const c_int,
    number_of_destinations: c_int,
    annotations: Annotations,
    fallback_speed: c_double,
    fallback_coordinate: FallbackCoordinate,
    scale_factor: c_double,
}

impl CTableRequest {
    fn new(request: &mut TableRequest) -> CTableRequest {
        let mut c_request = CTableRequest {
            general_options: (&mut request.general_options).into(),
            sources: std::ptr::null(),
            number_of_sources: 0,
            destinations: std::ptr::null(),
            number_of_destinations: 0,
            annotations: request.annotations.clone(),
            fallback_speed: request.fallback_speed,
            fallback_coordinate: request.fallback_coordinate.clone(),
            scale_factor: request.scale_factor,
        };

        if request.sources.is_some() {
            let sources = request.sources.as_ref().unwrap();
            c_request.sources = sources.as_ptr();
            c_request.number_of_sources = sources.len() as c_int;
        }

        if request.destinations.is_some() {
            let destinations = request.destinations.as_ref().unwrap();
            c_request.destinations = destinations.as_ptr();
            c_request.number_of_destinations = destinations.len() as c_int;
        }

        c_request
    }
}

pub struct TableRequest {
    pub(crate) general_options: GeneralOptions,
    pub(crate) sources: Option<Vec<i32>>,
    pub(crate) destinations: Option<Vec<i32>>,
    pub(crate) annotations: Annotations,
    pub(crate) fallback_speed: f64,
    pub(crate) fallback_coordinate: FallbackCoordinate,
    pub(crate) scale_factor: f64,
}

impl TableRequest {
    pub fn run(&mut self, osrm: &Osrm) -> (Status, TableResult) {
        unsafe {
            let mut result: *mut CTableResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CTableResult = &mut result;

            let status = osrm_table(
                *osrm.config,
                &mut CTableRequest::new(self) as *mut CTableRequest,
                result_ptr,
            );

            let converted_result = TableResult::new(&(*result));

            table_result_destroy(result);

            (status, converted_result)
        }
    }
}
