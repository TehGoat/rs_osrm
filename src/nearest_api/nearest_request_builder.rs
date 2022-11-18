use std::ffi::CString;

use crate::general::{c_structs::{c_approach::Approach, c_bearing::Bearing}, rs_structs::{coordinate::Coordinate, general_options::{GeneralOptions, GeneralOptionsTrait}}, to_vec_ccoordinate};

use super::nearest_request::NearestRequest;

pub struct NearestRequestBuilder {
    general_options: GeneralOptions,
    number_of_results: i32,
}

impl NearestRequestBuilder {
    pub fn new(lat: f64, lng: f64) -> NearestRequestBuilder {
        NearestRequestBuilder {
            general_options: GeneralOptions::new(&vec![Coordinate { latitude: lat, longitude: lng}]),
            number_of_results: 1
        }
    }

    pub fn set_number_of_results<'a>(&'a mut self, number_of_results: i32) -> &'a mut Self {
        self.number_of_results = number_of_results;
        self
    }

    pub fn build(&self) -> Result<NearestRequest, String> {
        Ok(NearestRequest {
            general_options: self.general_options.clone(),
            number_of_results: self.number_of_results,
        })
    }
    
}

impl GeneralOptionsTrait for NearestRequestBuilder {
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