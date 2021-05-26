use std::os::raw::{c_char, c_double};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CWaypoint {
    pub(crate) hint: *const c_char,
    pub(crate) distance: c_double,
    pub(crate) name: *const c_char,
    pub(crate) location: [c_double; 2],
}