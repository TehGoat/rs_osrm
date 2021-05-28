use std::ffi::CString;
use std::os::raw::c_int;

use crate::Osrm;
use crate::Status;
use crate::general::c_structs::c_approach::Approach;
use crate::general::c_structs::c_bearing::Bearing;
use crate::general::c_structs::c_general_options::CGeneralOptions;
use crate::general::rs_structs::coordinate::Coordinate;
use crate::general::rs_structs::general_options::GeneralOptions;
use crate::general::rs_structs::general_options::GeneralOptionsTrait;
use crate::general::to_vec_ccoordinate;

use super::nearest_result::CNearestResult;
use super::nearest_result::NearestResult;
use super::nearest_result::{nearest_result_destroy, osrm_nearest};

#[repr(C)]
pub(crate) struct CNearestRequest {
    general_options: CGeneralOptions,
    number_of_results: c_int,
}

impl CNearestRequest {
    fn new(request: &mut NearestRequest) -> CNearestRequest {
        CNearestRequest {
            general_options: (&mut request.general_options).into(),
            number_of_results: request.number_of_results as c_int,
        }
    }
}

pub struct NearestRequest {
    general_options: GeneralOptions,
    number_of_results: i32,
}

impl NearestRequest {
    pub fn new(latitude: f64, longitude: f64) -> NearestRequest {
        NearestRequest {
            general_options: GeneralOptions::new(&vec![Coordinate {
                latitude,
                longitude,
            }]),
            number_of_results: 1,
        }
    }

    pub fn set_general_options<'a>(&'a mut self, general_options: GeneralOptions) -> &'a mut Self {
        self.general_options = general_options;
        self
    }

    pub fn get_general_options<'a>(&'a mut self) -> &'a mut GeneralOptions {
        &mut self.general_options
    }

    pub fn set_number_of_results<'a>(&'a mut self, number_of_results: i32) -> &'a mut Self {
        self.number_of_results = number_of_results;
        self
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, NearestResult) {
        unsafe {
            let mut result: *mut CNearestResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CNearestResult = &mut result;

            let status = osrm_nearest(
                *osrm.config,
                &mut CNearestRequest::new(self) as *mut CNearestRequest,
                result_ptr,
            );

            let converted_result = NearestResult::new(&(*result));

            nearest_result_destroy(result);

            (status, converted_result)
        }
    }
}

impl GeneralOptionsTrait for NearestRequest {
    fn set_coordinate<'a>(&'a mut self, coordinates: &Vec<Coordinate>) -> &'a mut Self {
        if coordinates.len() == 0 || coordinates.len() > 1 {
            panic!("Need one coordinate");
        }

        self.general_options.coordinate = to_vec_ccoordinate(coordinates);
        self
    }

    fn set_bearings<'a>(&'a mut self, bearings: Option<Vec<Option<Bearing>>>) -> &'a mut Self {
        self.general_options.bearings = bearings;
        self
    }

    fn set_radiuses<'a>(&'a mut self, radiuses: Option<Vec<Option<f64>>>) -> &'a mut Self {
        self.general_options.radiuses = radiuses;
        self
    }

    fn set_generate_hints<'a>(&'a mut self, generate_hints: bool) -> &'a mut Self {
        self.general_options.generate_hints = generate_hints;
        self
    }

    fn set_skip_waypoints<'a>(&'a mut self, skip_waypoints: bool) -> &'a mut Self {
        self.general_options.skip_waypoints = skip_waypoints;
        self
    }

    fn set_hints<'a>(&'a mut self, hints: Option<Vec<CString>>) -> &'a mut Self {
        self.general_options.hints = hints;
        self
    }

    fn set_approach<'a>(&'a mut self, approach: Option<Vec<Option<Approach>>>) -> &'a mut Self {
        self.general_options.approach = approach;
        self
    }

    fn set_exclude<'a>(&'a mut self, exclude: Option<Vec<CString>>) -> &'a mut Self {
        self.general_options.exclude = exclude;
        self
    }
}