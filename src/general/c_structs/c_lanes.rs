use std::os::raw::{c_char, c_int};

use crate::Boolean;


#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmLanes {
    pub(crate) indications: *const *const c_char,
    pub(crate) number_of_indications: c_int,
    pub(crate) valid: Boolean,
}