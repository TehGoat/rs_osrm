use std::{
    os::raw::{c_char, c_double, c_int},
};

use crate::{
    general::{rs_structs::general_options::GeneralOptions, COsrmCoordinate},
    Boolean,
};

use super::{c_approach::Approach, c_bearing::Bearing};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct CGeneralOptions {
    pub(crate) coordinate: *const COsrmCoordinate,
    pub(crate) number_of_coordinates: c_int,
    pub(crate) bearings: *const *const Bearing,
    pub(crate) radiuses: *const *const c_double,
    pub(crate) generate_hints: Boolean,
    pub(crate) skip_waypoints: Boolean,
    pub(crate) hints: *const *const c_char,
    pub(crate) approach: *const *const Approach,
    pub(crate) exclude: *const *const c_char,
    pub(crate) number_of_excludes: c_int,
}

impl From<&mut GeneralOptions> for CGeneralOptions {
    fn from(option: &mut GeneralOptions) -> Self {
        let mut general_c_option = CGeneralOptions {
            coordinate: option.coordinate.as_ptr(),
            number_of_coordinates: option.coordinate.len() as c_int,
            bearings: std::ptr::null(),
            radiuses: std::ptr::null(),
            generate_hints: Boolean::from(option.generate_hints),
            skip_waypoints: Boolean::from(option.skip_waypoints),
            hints: std::ptr::null(),
            approach: std::ptr::null(),
            exclude: std::ptr::null(),
            number_of_excludes: 0,
        };

        if let Some(bearings) = &option.bearings {
            option.bearings_t = bearings
                .iter()
                .map(|bearing| match bearing {
                    Some(it) => it,
                    None => std::ptr::null(),
                })
                .collect();
            general_c_option.bearings = option.bearings_t.as_ptr();
        }

        if let Some(radiuses) = &option.radiuses {
            option.radiuses_t = radiuses
                .iter()
                .map(|radius| match radius {
                    Some(it) => it,
                    None => std::ptr::null(),
                })
                .collect();
            general_c_option.radiuses = option.radiuses_t.as_ptr();
        }

        if let Some(hints) = &option.hints {
           general_c_option.hints = hints.as_ptr() as *const *const c_char; 
        }

        if let Some(approach) = &option.approach {
            option.approach_t = approach
                .iter()
                .map(|approach| match approach {
                    Some(it) => it,
                    None => std::ptr::null(),
                })
                .collect();

            general_c_option.approach = option.approach_t.as_ptr();
        }

        if let Some(exclude) = &option.exclude {
           general_c_option.exclude = exclude.as_ptr() as *const *const c_char;
           general_c_option.number_of_excludes = exclude.len() as c_int; 
        }

        general_c_option
    }
}
