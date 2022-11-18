use std::os::raw::c_int;

use crate::{Boolean, Osrm, Status, general::{c_structs::{c_general_options::CGeneralOptions}, rs_structs::{general_options::GeneralOptions}}};

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
    pub(crate) general_options: GeneralOptions,
    pub(crate) steps: bool,
    pub(crate) alternatives: bool,
    pub(crate) number_of_alternatives: u32,
    pub(crate) annotations: bool,
    pub(crate) annotations_type: AnnotationsType,
    pub(crate) geometries: GeometriesType,
    pub(crate) overview: OverviewType,
    pub(crate) continue_straight: Option<bool>,
    pub(crate) waypoints: Option<Vec<u64>>,
}

impl RouteRequest {


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
