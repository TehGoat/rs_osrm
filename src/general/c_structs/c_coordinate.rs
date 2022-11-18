use std::os::raw::c_double;

use crate::general::rs_structs::coordinate::Coordinate;

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmCoordinate {
    pub(crate) latitude: c_double,
    pub(crate) longitude: c_double,
}

impl From<&Coordinate> for COsrmCoordinate {
    fn from(coordinate: &Coordinate) -> Self {
        COsrmCoordinate {
            latitude: coordinate.latitude, 
            longitude: coordinate.longitude
        } 
    }
}
