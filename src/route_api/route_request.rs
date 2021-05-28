use std::{ffi::CString, os::raw::c_int};

use crate::{Boolean, Osrm, Status, general::{c_structs::{c_approach::Approach, c_bearing::Bearing, c_general_options::CGeneralOptions}, rs_structs::{coordinate::Coordinate, general_options::{GeneralOptions, GeneralOptionsTrait}}, to_vec_ccoordinate}};

use super::{
    osrm_route,
    route_result::{CRouteResult, RouteResult},
    route_result_destroy, AnnotationsType, ContinueStraight, GeometriesType, OverviewType,
};

#[repr(C)]
pub(crate) struct CRouteRequest {
    general_options: CGeneralOptions,
    steps: Boolean,
    alternatives: Boolean,
    number_of_alternatives: u32,
    annotations: Boolean,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType,
    continue_straight: ContinueStraight,
    waypoints: *const u64,
    number_of_waypoints: i32,
}

impl From<&mut RouteRequest> for CRouteRequest {
    fn from(request: &mut RouteRequest) -> Self {
        CRouteRequest {
            general_options: (&mut request.general_options).into(),
            steps: Boolean::from(request.steps),
            alternatives: Boolean::from(request.alternatives),
            number_of_alternatives: request.number_of_alternatives,
            annotations: Boolean::from(request.annotations),
            annotations_type: request.annotations_type.clone(),
            geometries: request.geometries.clone(),
            overview: request.overview.clone(),
            continue_straight: if request.continue_straight.is_some() {
                if request.continue_straight.unwrap() {
                    ContinueStraight::ContinueStraightTrue
                } else {
                    ContinueStraight::ContinueStraightFalse
                }
            } else {
                ContinueStraight::ContinueStraightNone
            },
            waypoints: if request.waypoints.is_some() {
                request.waypoints.as_ref().unwrap().as_ptr()
            } else {
                std::ptr::null()
            },
            number_of_waypoints: if request.waypoints.is_some() {
                request.waypoints.as_ref().unwrap().len() as c_int
            } else {
                0
            },
        }
    }
}

pub struct RouteRequest {
    pub general_options: GeneralOptions,
    pub steps: bool,
    pub alternatives: bool,
    pub number_of_alternatives: u32,
    pub annotations: bool,
    pub annotations_type: AnnotationsType,
    pub geometries: GeometriesType,
    pub overview: OverviewType,
    pub continue_straight: Option<bool>,
    pub waypoints: Option<Vec<u64>>,
}

impl RouteRequest {
    pub fn new(coordinates: &Vec<Coordinate>) -> RouteRequest {
        RouteRequest {
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

    pub fn run(&mut self, osrm: &Osrm) -> (Status, RouteResult) {
        unsafe {
            let mut result: *mut CRouteResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CRouteResult = &mut result;

            let status = osrm_route(
                *osrm.config,
                &mut CRouteRequest::from(self) as *mut CRouteRequest,
                result_ptr,
            );

            let converted_result = RouteResult::from(&(*result));

            route_result_destroy(result);

            (status, converted_result)
        }
    }
}

impl GeneralOptionsTrait for RouteRequest {
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
