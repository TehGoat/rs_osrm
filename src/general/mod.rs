use crate::Boolean;
use core::slice;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_int, c_short};

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
            longitude
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

impl COsrmIntersections {
    pub(crate) fn to_intersections(&self) -> Intersections {
        let mut intersection = Intersections {
            location: self.location.to_coordinate(),
            bearings: Vec::new(),
            classes: Vec::new(),
            entry: Vec::new(),
            intersection_in: self.intersection_in,
            intersection_out: self.intersection_out,
            lanes: Vec::new(),
        };

        if self.bearings != std::ptr::null_mut() {
            intersection.bearings = unsafe {
                slice::from_raw_parts(self.bearings, self.number_of_bearings as usize).to_vec()
            };
        }

        if self.classes != std::ptr::null_mut() {
            let classes_vec: Vec<*const c_char> = unsafe {
                slice::from_raw_parts(self.classes, self.number_of_classes as usize).to_vec()
            };

            for class in classes_vec {
                intersection.classes.push(c_string_to_string(class));
            }
        }

        if self.entry != std::ptr::null_mut() {
            let boolean_vec = unsafe {
                slice::from_raw_parts(self.entry, self.number_of_entries as usize).to_vec()
            };

            for class in boolean_vec {
                intersection.entry.push(class == Boolean::TRUE);
            }
        }

        if self.lanes != std::ptr::null_mut() {
            let lanes_vec = unsafe {
                slice::from_raw_parts(self.lanes, self.number_of_lanes as usize).to_vec()
            };

            for lane in &lanes_vec {
                intersection.lanes.push(lane.into());
            }
        }

        intersection
    }
}

#[derive(Debug)]
pub struct Intersections {
    pub location: Coordinate,
    pub bearings: Vec<i32>,
    pub classes: Vec<String>,
    pub entry: Vec<bool>,
    pub intersection_in: i32,
    pub intersection_out: i32,
    pub lanes: Vec<Lanes>,
}

impl From<&COsrmIntersections> for Intersections {
    fn from(c_intersection: &COsrmIntersections) -> Self {
        Intersections {
            location: c_intersection.location.to_coordinate(),
            intersection_in: c_intersection.intersection_in,
            intersection_out: c_intersection.intersection_out,
            bearings: if c_intersection.bearings != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_intersection.bearings,
                        c_intersection.number_of_bearings as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            classes: if c_intersection.classes != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_intersection.classes,
                        c_intersection.number_of_classes as usize,
                    )
                }
                .iter()
                .map(|class| c_string_to_string(*class))
                .collect()
            } else {
                Vec::new()
            },
            entry: if c_intersection.entry != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_intersection.entry,
                        c_intersection.number_of_entries as usize,
                    )
                }
                .iter()
                .map(|entry| *entry == Boolean::TRUE)
                .collect()
            } else {
                Vec::new()
            },
            lanes: if c_intersection.lanes != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_intersection.lanes,
                        c_intersection.number_of_lanes as usize,
                    )
                }
                .iter()
                .map(|lane| lane.into())
                .collect()
            } else {
                Vec::new()
            },
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
                    .map(|intersection| intersection.to_intersections())
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

impl COsrmAnnotation {
    pub(crate) fn to_annotation(&self) -> Annotation {
        let mut annotation = Annotation {
            duration: Vec::new(),
            distance: Vec::new(),
            speed: Vec::new(),
            weight: Vec::new(),
            nodes: Vec::new(),
            datasources: Vec::new(),
            metadata: None,
        };

        if self.duration != std::ptr::null_mut() {
            annotation.duration = unsafe {
                slice::from_raw_parts(self.duration, (self.number_of_coordinates + 1) as usize)
                    .to_vec()
            };
        }

        if self.distance != std::ptr::null_mut() {
            annotation.distance = unsafe {
                slice::from_raw_parts(self.distance, (self.number_of_coordinates + 1) as usize)
                    .to_vec()
            };
        }

        if self.speed != std::ptr::null_mut() {
            annotation.speed = unsafe {
                slice::from_raw_parts(self.speed, (self.number_of_coordinates + 1) as usize)
                    .to_vec()
            };
        }

        if self.weight != std::ptr::null_mut() {
            annotation.weight = unsafe {
                slice::from_raw_parts(self.weight, (self.number_of_coordinates + 1) as usize)
                    .to_vec()
            };
        }

        if self.nodes != std::ptr::null_mut() {
            annotation.nodes = unsafe {
                slice::from_raw_parts(self.nodes, (self.number_of_coordinates + 1) as usize)
                    .to_vec()
            };
        }

        if self.datasources != std::ptr::null_mut() {
            annotation.datasources = unsafe {
                slice::from_raw_parts(self.datasources, (self.number_of_coordinates + 1) as usize)
                    .to_vec()
            };
        }

        if self.metadata != std::ptr::null_mut() {
            annotation.metadata = Option::from(unsafe { (*self.metadata).to_meta_data() });
        }

        annotation
    }
}

#[derive(Debug)]
pub struct Annotation {
    pub duration: Vec<f64>,
    pub distance: Vec<f64>,
    pub speed: Vec<f64>,
    pub weight: Vec<f64>,
    pub nodes: Vec<i64>,
    pub datasources: Vec<i32>,
    pub metadata: Option<MetaData>,
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
