use std::ffi::CString;

use crate::{general::{c_structs::{c_approach::Approach, c_bearing::Bearing}, rs_structs::{coordinate::Coordinate, general_options::{GeneralOptions, GeneralOptionsTrait}}, to_vec_ccoordinate}, route_api::{AnnotationsType, GeometriesType, OverviewType}};

use super::{trip_end, trip_request::TripRequest, trip_start};

pub struct TripRequestBuilder {
    general_options: GeneralOptions,
    roundtrip: bool,
    source: trip_start,
    destination: trip_end,
    steps: bool,
    annotations: bool,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType,
}

impl TripRequestBuilder {
    pub fn new(coordinates: &Vec<Coordinate>) -> TripRequestBuilder {
        TripRequestBuilder {
            general_options: GeneralOptions::new(coordinates),
            roundtrip: true,
            source: trip_start::StartAny,
            destination: trip_end::EndAny,
            steps: false,
            annotations: false,
            annotations_type: AnnotationsType::None,
            geometries: GeometriesType::Polyline,
            overview: OverviewType::Simplified,
        }
    }

    pub fn set_roundtrip<'a>(&'a mut self, roundtrip: bool) -> &'a mut Self {
        self.roundtrip = roundtrip;
        self
    }

    pub fn set_sources<'a>(&'a mut self, sources: trip_start) -> &'a mut Self {
        self.source = sources;
        self
    }

    pub fn set_destinations<'a>(&'a mut self, destinations: trip_end) -> &'a mut Self {
        self.destination = destinations;
        self
    }

    pub fn set_steps<'a>(&'a mut self, steps: bool) -> &'a mut Self {
        self.steps = steps;
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

    pub fn build(&self) -> Result<TripRequest, String> {
        Ok (
            TripRequest {
                general_options: self.general_options.clone(),
                roundtrip: self.roundtrip,
                source: self.source.clone(),
                destination: self.destination.clone(),
                steps: self.steps,
                annotations: self.annotations,
                annotations_type: self.annotations_type.clone(),
                geometries: self.geometries.clone(),
                overview: self.overview.clone(),
            }
        )
    }
}

impl GeneralOptionsTrait for TripRequestBuilder{
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