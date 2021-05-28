use std::ffi::CString;

use crate::general::{
    c_structs::{c_approach::Approach, c_bearing::Bearing},
    to_vec_ccoordinate, COsrmCoordinate, Coordinate,
};

pub trait GeneralOptionsTrait {
    fn set_coordinate<'a>(&'a mut self, coordinates: &Vec<Coordinate>) -> &'a mut Self; 

    fn set_bearings<'a>(&'a mut self, bearings: Option<Vec<Option<Bearing>>>) -> &'a mut Self;

    fn set_radiuses<'a>(&'a mut self, radiuses: Option<Vec<Option<f64>>>) -> &'a mut Self;

    fn set_generate_hints<'a>(&'a mut self, generate_hints: bool) -> &'a mut Self;

    fn set_skip_waypoints<'a>(&'a mut self, skip_waypoints: bool) -> &'a mut Self;

    fn set_hints<'a>(&'a mut self, hints: Option<Vec<CString>>) -> &'a mut Self;

    fn set_approach<'a>(&'a mut self, approach: Option<Vec<Option<Approach>>>) -> &'a mut Self;

    fn set_exclude<'a>(&'a mut self, exclude: Option<Vec<CString>>) -> &'a mut Self;
}

#[derive(Clone)]
pub struct GeneralOptions {
    pub(crate) coordinate: Vec<COsrmCoordinate>,
    pub(crate) bearings: Option<Vec<Option<Bearing>>>,
    pub(crate) bearings_t: Vec<*const Bearing>,
    pub(crate) radiuses: Option<Vec<Option<f64>>>,
    pub(crate) radiuses_t: Vec<*const f64>,
    pub(crate) generate_hints: bool,
    pub(crate) skip_waypoints: bool,
    pub(crate) hints: Option<Vec<CString>>,
    pub(crate) approach: Option<Vec<Option<Approach>>>,
    pub(crate) approach_t: Vec<*const Approach>,
    pub(crate) exclude: Option<Vec<CString>>,
}

impl GeneralOptions {
    pub fn new(coordinates: &Vec<Coordinate>) -> GeneralOptions {
        GeneralOptions {
            coordinate: to_vec_ccoordinate(&coordinates),
            bearings: None,
            bearings_t: vec![],
            radiuses: None,
            radiuses_t: vec![],
            generate_hints: true,
            skip_waypoints: false,
            hints: None,
            approach: None,
            approach_t: vec![],
            exclude: None,
        }
    }
}

impl GeneralOptionsTrait for GeneralOptions {

    fn set_coordinate<'a>(&'a mut self, coordinates: &Vec<Coordinate>) -> &'a mut Self {
        self.coordinate = to_vec_ccoordinate(&coordinates);
        self
    }

    fn set_bearings<'a>(&'a mut self, bearings: Option<Vec<Option<Bearing>>>) -> &'a mut Self {
        self.bearings = bearings;
        self
    }

    fn set_radiuses<'a>(&'a mut self, radiuses: Option<Vec<Option<f64>>>) -> &'a mut Self {
        self.radiuses = radiuses;
        self
    }

    fn set_generate_hints<'a>(&'a mut self, generate_hints: bool) -> &'a mut Self {
        self.generate_hints = generate_hints;
        self
    }

    fn set_skip_waypoints<'a>(&'a mut self, skip_waypoints: bool) -> &'a mut Self {
        self.skip_waypoints = skip_waypoints;
        self
    }

    fn set_hints<'a>(&'a mut self, hints: Option<Vec<CString>>) -> &'a mut Self {
        self.hints = hints;
        self
    }

    fn set_approach<'a>(&'a mut self, approach: Option<Vec<Option<Approach>>>) -> &'a mut Self {
        self.approach = approach;
        self
    }

    fn set_exclude<'a>(&'a mut self, exclude: Option<Vec<CString>>) -> &'a mut Self {
        self.exclude = exclude;
        self
    }
}
