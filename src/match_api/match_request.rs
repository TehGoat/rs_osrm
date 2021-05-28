use std::{ffi::CString, os::raw::c_int};

use crate::{Boolean, Osrm, Status, general::{
        c_structs::{c_approach::Approach, c_bearing::Bearing, c_general_options::CGeneralOptions},
        rs_structs::{
            coordinate::Coordinate,
            general_options::{GeneralOptions, GeneralOptionsTrait},
        },
        to_vec_ccoordinate,
    }, route_api::{AnnotationsType, GeometriesType, OverviewType}};

use super::{
    match_result::{CMatchResult, MatchResult},
    match_result_destroy, osrm_match, Gap,
};

#[repr(C)]
pub(crate) struct CMatchRequest {
    general_options: CGeneralOptions,
    steps: Boolean,
    geometries: GeometriesType,
    annotations: Boolean,
    annotations_type: AnnotationsType,
    overview: OverviewType,
    timestamps: *const c_int,
    gaps: Gap,
    tidy: Boolean,
    waypoints: *const c_int,
    number_of_waypoints: c_int,
}

impl From<&mut MatchRequest> for CMatchRequest {
    fn from(request: &mut MatchRequest) -> Self {
        CMatchRequest {
            general_options: (&mut request.general_options).into(),
            steps: Boolean::from(request.steps),
            geometries: request.geometries.clone(),
            annotations: Boolean::from(request.annotations),
            annotations_type: request.annotations_type.clone(),
            overview: request.overview.clone(),
            timestamps: match &request.timestamps {
                Some(timestamps) => timestamps.as_ptr(),
                None => std::ptr::null(),
            },
            gaps: request.gaps.clone(),
            tidy: Boolean::from(request.tidy),
            waypoints: match &request.waypoints {
                Some(waypoints) => waypoints.as_ptr(),
                None => std::ptr::null(),
            },
            number_of_waypoints: match &request.waypoints {
                Some(waypoints) => waypoints.len() as c_int,
                None => 0,
            },
        }
    }
}

pub struct MatchRequest {
    pub general_options: GeneralOptions,
    pub steps: bool,
    pub geometries: GeometriesType,
    pub annotations: bool,
    pub annotations_type: AnnotationsType,
    pub overview: OverviewType,
    pub timestamps: Option<Vec<i32>>,
    pub gaps: Gap,
    pub tidy: bool,
    pub waypoints: Option<Vec<i32>>,
}

impl MatchRequest {
    pub fn new(coordinates: &Vec<Coordinate>) -> MatchRequest {
        MatchRequest {
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

    pub fn run(&mut self, osrm: &Osrm) -> (Status, MatchResult) {
        unsafe {
            let mut result: *mut CMatchResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CMatchResult = &mut result;

            let status = osrm_match(
                *osrm.config,
                &mut CMatchRequest::from(self) as *mut CMatchRequest,
                result_ptr,
            );

            let converted_result = MatchResult::from(&(*result));

            match_result_destroy(result);

            (status, converted_result)
        }
    }
}

impl GeneralOptionsTrait for MatchRequest {
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
