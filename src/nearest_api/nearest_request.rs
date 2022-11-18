use std::os::raw::c_int;

use crate::Osrm;
use crate::Status;
use crate::general::c_structs::c_general_options::CGeneralOptions;
use crate::general::rs_structs::general_options::GeneralOptions;

use super::nearest_result::CNearestResult;
use super::nearest_result::NearestResult;
use super::{nearest_result_destroy, osrm_nearest};

#[repr(C)]
pub(crate) struct CNearestRequest {
    general_options: CGeneralOptions,
    number_of_results: c_int,
}

impl From<&mut NearestRequest> for CNearestRequest {
    fn from(request: &mut NearestRequest) -> Self {
        CNearestRequest {
            general_options: (&mut request.general_options).into(),
            number_of_results: request.number_of_results as c_int,
        }
    }
}

pub struct NearestRequest {
    pub(crate) general_options: GeneralOptions,
    pub(crate) number_of_results: i32,
}

impl NearestRequest {
    pub fn run(&mut self, osrm: &Osrm) -> (Status, NearestResult) {
        unsafe {
            let mut result: *mut CNearestResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CNearestResult = &mut result;

            let status = osrm_nearest(
                *osrm.config,
                &mut CNearestRequest::from(self) as *mut CNearestRequest,
                result_ptr,
            );

            let converted_result = NearestResult::new(&(*result));

            nearest_result_destroy(result);

            (status, converted_result)
        }
    }
}

