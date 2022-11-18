use std::ffi::CString;

use crate::general::{c_structs::{c_approach::Approach, c_bearing::Bearing}, rs_structs::{coordinate::Coordinate, general_options::{GeneralOptions, GeneralOptionsTrait}}, to_vec_ccoordinate};

use super::{AnnotationsType, GeometriesType, OverviewType, route_request::RouteRequest};

pub struct RouteRequestBuilder {
    general_options: GeneralOptions,
    steps: bool,
    alternatives: bool,
    number_of_alternatives: u32,
    annotations: bool,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType,
    continue_straight: Option<bool>,
    waypoints: Option<Vec<u64>>,
}

impl RouteRequestBuilder {
    pub fn new(coordinates: &Vec<Coordinate>) -> RouteRequestBuilder {
        RouteRequestBuilder {
            general_options: GeneralOptions::new(coordinates),
            steps: false,
            alternatives: false,
            number_of_alternatives: 0,
            annotations: false,
            annotations_type: AnnotationsType::None,
            geometries: GeometriesType::Polyline,
            overview: OverviewType::Simplified,
            continue_straight: None,
            waypoints: None,
        }
    }

    pub fn set_steps<'a>(&'a mut self, steps: bool) -> &'a mut Self {
        self.steps = steps;
        self
    }

    pub fn set_alternatives<'a>(&'a mut self, alternatives: bool) -> &'a mut Self {
        self.alternatives = alternatives;
        self
    }

    pub fn set_number_of_alternatives<'a>(&'a mut self, number_of_alternatives: u32) -> &'a mut Self {
        self.number_of_alternatives = number_of_alternatives;
        self
    }

    pub fn set_annotations<'a>(&'a mut self, annotations: bool) -> &'a mut Self {
        self.annotations = annotations;
        self
    }

    pub fn set_annotations_type<'a>(&'a mut self, annotations_type: AnnotationsType) -> &'a mut Self {
        self.annotations_type = annotations_type;
        self
    }
    
    pub fn set_geometries<'a>(&'a mut self, geometries: GeometriesType) -> &'a mut Self {
        self.geometries = geometries;
        self
    }

    pub fn set_overview<'a>(&'a mut self, overview: OverviewType) -> &'a mut Self {
        self.overview = overview;
        self
    }

    pub fn set_continue_straight<'a>(&'a mut self, continue_straight: Option<bool>) -> &'a mut Self {
        self.continue_straight = continue_straight;
        self
    }

    pub fn set_waypoints<'a>(&'a mut self, waypoints: Option<Vec<u64>>) -> &'a mut Self {
        self.waypoints = waypoints;
        self
    }

    pub fn build(&self) -> Result<RouteRequest, String> {
        Ok(
            RouteRequest {
                general_options: self.general_options.clone(),
                steps: self.steps,
                alternatives: self.alternatives,
                number_of_alternatives: self.number_of_alternatives,
                annotations: self.annotations,
                annotations_type: self.annotations_type.clone(),
                geometries: self.geometries.clone(),
                overview: self.overview.clone(),
                continue_straight: self.continue_straight,
                waypoints: self.waypoints.clone(),
            }
        )
    }
}

impl GeneralOptionsTrait for RouteRequestBuilder {
    fn set_coordinate<'a>(&'a mut self, coordinates: &Vec<Coordinate>) -> &'a mut Self {
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
