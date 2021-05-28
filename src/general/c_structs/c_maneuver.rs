use std::os::raw::{c_char, c_int};

use crate::general::COsrmCoordinate;

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmManeuver {
    pub(crate) bearing_before: c_int,
    pub(crate) bearing_after: c_int,
    pub(crate) coordinate: COsrmCoordinate,
    pub(crate) maneuver_type: *const c_char,
    pub(crate) modifer: *const c_char,
}