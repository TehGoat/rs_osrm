use std::ffi::CStr;
use std::os::raw::{c_char};

use self::c_structs::c_coordinate::COsrmCoordinate;
use self::rs_structs::coordinate::Coordinate;

pub mod c_structs;
pub mod rs_structs;

pub(crate) fn to_vec_ccoordinate(coordinates: &Vec<Coordinate>) -> Vec<COsrmCoordinate> {
    let mut return_vec = Vec::new();
    for coordinate in coordinates {
        return_vec.push(coordinate.into());
    }

    return_vec
}

pub(crate) fn c_string_to_string(c_string: *const c_char) -> String {
    if c_string == std::ptr::null_mut() {
        return "".to_string();
    }

    let c_str: &CStr = unsafe { CStr::from_ptr(c_string) };
    let c_str_slice: &str = c_str.to_str().unwrap();

    c_str_slice.to_owned()
}

pub(crate) fn c_string_to_option_string(c_string: *const c_char) -> Option<String> {
    if c_string == std::ptr::null_mut() {
        return None;
    }

    let c_str: &CStr = unsafe { CStr::from_ptr(c_string) };
    let c_str_slice: &str = c_str.to_str().unwrap();

    Option::from(c_str_slice.to_owned())
}
