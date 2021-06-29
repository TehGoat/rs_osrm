use std::ffi::CString;

use crate::{
    general::{
        c_structs::{c_approach::Approach, c_bearing::Bearing},
        rs_structs::{
            coordinate::Coordinate,
            general_options::{GeneralOptions, GeneralOptionsTrait},
        },
        to_vec_ccoordinate,
    },
    route_api::{AnnotationsType, GeometriesType, OverviewType},
};

use super::{match_request::MatchRequest, Gap};

pub struct MatchRequestBuilder {
    general_options: GeneralOptions,
    steps: bool,
    geometries: GeometriesType,
    annotations: bool,
    annotations_type: AnnotationsType,
    overview: OverviewType,
    timestamps: Option<Vec<i32>>,
    gaps: Gap,
    tidy: bool,
    waypoints: Option<Vec<i32>>,
}

impl MatchRequestBuilder {
    pub fn new(coordinates: &Vec<Coordinate>) -> MatchRequestBuilder {
        MatchRequestBuilder {
            general_options: GeneralOptions::new(coordinates),
            steps: false,
            geometries: GeometriesType::Polyline,
            annotations: false,
            annotations_type: AnnotationsType::None,
            overview: OverviewType::Simplified,
            timestamps: None,
            gaps: Gap::Split,
            tidy: false,
            waypoints: None,
        }
    }

    pub fn set_steps<'a>(&'a mut self, steps: bool) -> &'a mut Self {
        self.steps = steps;
        self
    }
    
    pub fn set_geometries<'a>(&'a mut self, geometries: GeometriesType) -> &'a mut Self {
        self.geometries = geometries;
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

    pub fn set_overview<'a>(&'a mut self, overview: OverviewType) -> &'a mut Self {
        self.overview = overview;
        self
    }

    pub fn set_timestamps<'a>(&'a mut self, timestamps: Option<Vec<i32>>) -> &'a mut Self {
        self.timestamps = timestamps;
        self
    }

    pub fn set_gaps<'a>(&'a mut self, gaps: Gap) -> &'a mut Self {
        self.gaps = gaps;
        self
    }

    pub fn set_tidy<'a>(&'a mut self, tidy: bool) -> &'a mut Self {
        self.tidy = tidy;
        self
    }

    pub fn set_waypoints<'a>(&'a mut self, waypoints: Option<Vec<i32>>) -> &'a mut Self {
        self.waypoints = waypoints;
        self
    }

    pub fn build(&self) -> MatchRequest {
        MatchRequest {
            general_options: self.general_options.clone(),
            steps: self.steps,
            geometries: self.geometries.clone(),
            annotations: self.annotations,
            annotations_type: self.annotations_type.clone(),
            overview: self.overview.clone(),
            timestamps: self.timestamps.clone(),
            gaps: self.gaps.clone(),
            tidy: self.tidy,
            waypoints: self.waypoints.clone(),
        }
    }
}

impl GeneralOptionsTrait for MatchRequestBuilder {
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
