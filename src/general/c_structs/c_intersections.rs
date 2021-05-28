use std::os::raw::{c_char, c_int};

use crate::{Boolean, general::{COsrmCoordinate, COsrmLanes}};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmIntersections {
    pub(crate) location: COsrmCoordinate,
    pub(crate) bearings: *const c_int,
    pub(crate) number_of_bearings: c_int,
    pub(crate) classes: *const *const c_char,
    pub(crate) number_of_classes: c_int,
    pub(crate) entry: *const Boolean,
    pub(crate) number_of_entries: c_int,
    pub(crate) intersection_in: c_int,
    pub(crate) intersection_out: c_int,
    pub(crate) lanes: *const COsrmLanes,
    pub(crate) number_of_lanes: c_int,
}