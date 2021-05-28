use crate::Boolean;
use core::slice;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_short};

use self::c_structs::c_intersections::COsrmIntersections;
use self::rs_structs::intersections::Intersections;

pub mod c_structs;
pub mod rs_structs;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bearing {
    pub bearing: c_short,
    pub range: c_short,
}

#[repr(C)]
#[derive(Clone)]
pub enum Approach {
    UNRESTRICTED,
    CURB,
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmCoordinate {
    pub(crate) latitude: c_double,
    pub(crate) longitude: c_double,
}

impl COsrmCoordinate {
    pub(crate) fn to_coordinate(&self) -> Coordinate {
        Coordinate {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }

    pub(crate) fn to_ccoordinate(&self) -> COsrmCoordinate {
        COsrmCoordinate {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

pub(crate) fn to_vec_ccoordinate(coordinates: &Vec<Coordinate>) -> Vec<COsrmCoordinate> {
    let mut return_vec = Vec::new();
    for coordinate in coordinates {
        return_vec.push(coordinate.to_ccoordinate());
    }

    return_vec
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmLanes {
    pub(crate) indications: *const *const c_char,
    pub(crate) number_of_indications: c_int,
    pub(crate) valid: Boolean,
}
#[derive(Debug)]
pub struct Lanes {
    pub indications: Vec<String>,
    pub valid: bool,
}

impl From<&COsrmLanes> for Lanes {
    fn from(c_lanes: &COsrmLanes) -> Self {
        Lanes {
            indications: unsafe {
                slice::from_raw_parts(c_lanes.indications, c_lanes.number_of_indications as usize)
            }
            .iter()
            .map(|indication| c_string_to_string(*indication))
            .collect(),
            valid: c_lanes.valid == Boolean::TRUE,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmManeuver {
    pub(crate) bearing_before: c_int,
    pub(crate) bearing_after: c_int,
    pub(crate) coordinate: COsrmCoordinate,
    pub(crate) maneuver_type: *const c_char,
    pub(crate) modifer: *const c_char,
}

impl COsrmManeuver {
    pub(crate) fn to_maneuver(&self) -> Maneuver {
        Maneuver {
            bearing_before: self.bearing_before,
            bearing_after: self.bearing_after,
            coordinate: self.coordinate.to_coordinate(),
            maneuver_type: c_string_to_string(self.maneuver_type),
            modifer: c_string_to_option_string(self.modifer),
        }
    }
}

#[derive(Debug)]
pub struct Maneuver {
    pub bearing_before: i32,
    pub bearing_after: i32,
    pub coordinate: Coordinate,
    pub maneuver_type: String,
    pub modifer: Option<String>,
}

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

#[derive(Debug)]
pub struct Step {
    pub distance: f64,
    pub duration: f64,
    pub geometry: Option<String>,
    pub weight: c_double,
    pub name: Option<String>,
    pub reference: Option<String>,
    pub pronunciation: Option<String>,
    pub exits: Option<String>,
    pub mode: Option<String>,
    pub metadata: Option<Maneuver>,
    pub intersections: Vec<Intersections>,
    pub rotary_name: Option<String>,
    pub rotary_pronunciation: Option<String>,
    pub driving_side: Option<String>,
}

impl From<&COsrmStep> for Step {
    fn from(c_step: &COsrmStep) -> Self {
        Step {
            distance: c_step.distance,
            duration: c_step.duration,
            geometry: c_string_to_option_string(c_step.geometry),
            weight: c_step.weight,
            name: c_string_to_option_string(c_step.name),
            reference: c_string_to_option_string(c_step.reference),
            pronunciation: c_string_to_option_string(c_step.pronunciation),
            exits: c_string_to_option_string(c_step.exits),
            mode: c_string_to_option_string(c_step.exits),
            rotary_name: c_string_to_option_string(c_step.exits),
            rotary_pronunciation: c_string_to_option_string(c_step.exits),
            driving_side: c_string_to_option_string(c_step.exits),
            metadata: if c_step.metadata != std::ptr::null_mut() {
                unsafe { Option::from((*c_step.metadata).to_maneuver()) }.into()
            } else {
                None
            },
            intersections: if c_step.intersections != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_step.intersections,
                        c_step.number_of_intersections as usize,
                    )
                    .iter()
                    .map(|intersection| intersection.into())
                    .collect()
                }
            } else {
                Vec::new()
            },
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmMetaData {
    pub(crate) datasource_names: *const *const c_char,
    pub(crate) number_of_datasource_names: c_int,
}

impl COsrmMetaData {
    pub(crate) fn to_meta_data(&self) -> MetaData {
        let mut meta_data = MetaData {
            datasource_names: Vec::new(),
        };

        if self.datasource_names != std::ptr::null_mut() {
            let datasources_vec = unsafe {
                slice::from_raw_parts(
                    self.datasource_names,
                    self.number_of_datasource_names as usize,
                )
                .to_vec()
            };

            for datasource in datasources_vec {
                meta_data
                    .datasource_names
                    .push(c_string_to_string(datasource));
            }
        }

        meta_data
    }
}

#[derive(Debug)]
pub struct MetaData {
    datasource_names: Vec<String>,
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
