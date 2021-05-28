use std::slice;

use crate::{Boolean, general::{Coordinate, Lanes, c_string_to_string, c_structs::c_intersections::COsrmIntersections}};


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