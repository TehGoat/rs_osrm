use std::os::raw::{c_double, c_int};

use crate::general::COsrmMetaData;

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmAnnotation {
    pub(crate) duration: *const c_double,
    pub(crate) distance: *const c_double,
    pub(crate) speed: *const c_double,
    pub(crate) weight: *const c_double,
    pub(crate) nodes: *const i64,
    pub(crate) datasources: *const c_int,
    pub(crate) metadata: *const COsrmMetaData,
    pub(crate) number_of_coordinates: c_int,
}