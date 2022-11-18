use std::os::raw::{c_char, c_double, c_int};

use super::{c_intersections::COsrmIntersections, c_maneuver::COsrmManeuver};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmStep {
    pub(crate) distance: c_double,
    pub(crate) duration: c_double,
    pub(crate) geometry: *const c_char,
    pub(crate) weight: c_double,
    pub(crate) name: *const c_char,
    pub(crate) reference: *const c_char,
    pub(crate) pronunciation: *const c_char,
    pub(crate) exits: *const c_char,
    pub(crate) mode: *const c_char,
    pub(crate) metadata: *const COsrmManeuver,
    pub(crate) intersections: *const COsrmIntersections,
    pub(crate) number_of_intersections: c_int,
    pub(crate) rotary_name: *const c_char,
    pub(crate) rotary_pronunciation: *const c_char,
    pub(crate) driving_side: *const c_char,
}