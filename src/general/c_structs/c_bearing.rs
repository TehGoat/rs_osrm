use std::os::raw::c_short;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bearing {
    pub bearing: c_short,
    pub range: c_short,
}