use std::ffi::CString;

use crate::general::{c_structs::{c_approach::Approach, c_bearing::Bearing}, rs_structs::{coordinate::Coordinate, general_options::{GeneralOptions, GeneralOptionsTrait}}, to_vec_ccoordinate};

use super::{Annotations, FallbackCoordinate, table_request::TableRequest};

pub struct TableRequestBuilder {
    general_options: GeneralOptions,
    sources: Option<Vec<i32>>,
    destinations: Option<Vec<i32>>,
    annotations: Annotations,
    fallback_speed: f64,
    fallback_coordinate: FallbackCoordinate,
    scale_factor: f64,
}

impl TableRequestBuilder {
    pub fn new(coordinates: &Vec<Coordinate>) -> TableRequestBuilder {
        TableRequestBuilder {
            general_options: GeneralOptions::new(coordinates),
            sources: None,
            destinations: None,
            annotations: Annotations::DURATION,
            fallback_speed: std::f64::MAX,
            fallback_coordinate: FallbackCoordinate::INPUT,
            scale_factor: 1.0,
        }
    }

    pub fn set_sources<'a>(&'a mut self, sources: Option<Vec<i32>>) -> &'a mut Self {
        self.sources = sources;
        self
    }

    pub fn set_destinations<'a>(&'a mut self, destinations: Option<Vec<i32>>) -> &'a mut Self {
        self.destinations = destinations;
        self
    }

    pub fn set_annotations<'a>(&'a mut self, annotations: Annotations) -> &'a mut Self {
        self.annotations = annotations;
        self
    }

    pub fn set_fallback_speed<'a>(&'a mut self, fallback_speed: f64) -> &'a mut Self {
        self.fallback_speed = fallback_speed;
        self
    }

    pub fn set_fallback_coordinate<'a>(&'a mut self, fallback_coordinate: FallbackCoordinate) -> &'a mut Self {
        self.fallback_coordinate = fallback_coordinate;
        self
    }

    pub fn set_scale_factor<'a>(&'a mut self, scale_factor: f64) -> &'a mut Self {
        self.scale_factor = scale_factor;
        self
    }

    pub fn build(&self) -> Result<TableRequest, String> {
        Ok(
            TableRequest {
                general_options: self.general_options.clone(),
                sources: self.sources.clone(),
                destinations: self.destinations.clone(),
                annotations: self.annotations.clone(),
                fallback_speed: self.fallback_speed,
                fallback_coordinate: self.fallback_coordinate.clone(),
                scale_factor: self.scale_factor,
            }
        )
    }
}

impl GeneralOptionsTrait for TableRequestBuilder{
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