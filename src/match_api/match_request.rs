use std::os::raw::c_int;

use crate::{Boolean, Osrm, Status, general::{
        c_structs::{c_general_options::CGeneralOptions},
        rs_structs::{
            general_options::GeneralOptions,
        },
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
    pub(crate) general_options: GeneralOptions,
    pub(crate) steps: bool,
    pub(crate) geometries: GeometriesType,
    pub(crate) annotations: bool,
    pub(crate) annotations_type: AnnotationsType,
    pub(crate) overview: OverviewType,
    pub(crate) timestamps: Option<Vec<i32>>,
    pub(crate) gaps: Gap,
    pub(crate) tidy: bool,
    pub(crate) waypoints: Option<Vec<i32>>,
}

impl MatchRequest {
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


