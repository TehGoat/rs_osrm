use std::{os::raw::c_double, slice};

use crate::general::{c_string_to_option_string, c_structs::c_step::COsrmStep};

use super::{intersections::Intersections, maneuver::Maneuver};


#[derive(Debug)]
pub struct Step {
    pub distance: f64,
    pub duration: f64,
    pub geometry: Option<String>,
    pub weight: c_double,
    pub name: Option<String>,
    pub reference: Option<String>,
    pub pronunciation: Option<String>,
    pub exits: i32,
    pub mode: Option<String>,
    pub maneuver: Option<Maneuver>,
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
            exits: c_step.exits,
            mode: c_string_to_option_string(c_step.mode),
            rotary_name: c_string_to_option_string(c_step.rotary_name),
            rotary_pronunciation: c_string_to_option_string(c_step.rotary_pronunciation),
            driving_side: c_string_to_option_string(c_step.driving_side),
            maneuver: if c_step.metadata != std::ptr::null_mut() {
                unsafe {let maneuver: Maneuver = (&(*c_step.metadata)).into(); maneuver }.into()
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

