use crate::{Boolean, Osrm, Status, general::{c_structs::c_general_options::CGeneralOptions, rs_structs::{general_options::GeneralOptions}}, route_api::{AnnotationsType, GeometriesType, OverviewType}};

use super::{trip_end, trip_result::{CTripResult, TripResult}, trip_result_destroy, trip_start,osrm_trip };

#[repr(C)]
pub(crate) struct CTripRequest {
    general_options: CGeneralOptions,
    roundtrip: Boolean,
    source: trip_start,
    destination: trip_end,
    steps: Boolean,
    annotations: Boolean,
    annotations_type: AnnotationsType,
    geometries: GeometriesType,
    overview: OverviewType,
}

impl CTripRequest {
    fn new(request: &mut TripRequest) -> CTripRequest {
        CTripRequest {
            general_options: (&mut request.general_options).into(),
            roundtrip: Boolean::from(request.roundtrip),
            source: request.source.clone(),
            destination: request.destination.clone(),
            steps: Boolean::from(request.steps),
            annotations: Boolean::from(request.annotations),
            annotations_type: request.annotations_type.clone(),
            geometries: request.geometries.clone(),
            overview: request.overview.clone(),
        }
    }
}

pub struct TripRequest {
    pub(crate) general_options: GeneralOptions,
    pub(crate) roundtrip: bool,
    pub(crate) source: trip_start,
    pub(crate) destination: trip_end,
    pub(crate) steps: bool,
    pub(crate) annotations: bool,
    pub(crate) annotations_type: AnnotationsType,
    pub(crate) geometries: GeometriesType,
    pub(crate) overview: OverviewType,
}

impl TripRequest {
    pub fn run(&mut self, osrm: &Osrm) -> (Status, TripResult) {
        unsafe {
            let mut result: *mut CTripResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CTripResult = &mut result;

            let status = osrm_trip(
                *osrm.config,
                &mut CTripRequest::new(self) as *mut CTripRequest,
                result_ptr,
            );

            let converted_result = TripResult::new(&(*result));

            trip_result_destroy(result);

            (status, converted_result)
        }
    }
}