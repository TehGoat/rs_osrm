use std::slice;

use crate::general::{Annotation, Step, c_string_to_option_string, c_structs::c_route_leg::COsrmRouteLeg};

#[derive(Debug)]
pub struct RouteLeg {
    pub annotation: Option<Annotation>,
    pub duration: f64,
    pub summary: Option<String>,
    pub weight: f64,
    pub distance: f64,
    pub steps: Vec<Step>,
}

impl From<&COsrmRouteLeg> for RouteLeg {
    fn from(leg: &COsrmRouteLeg) -> Self {
        RouteLeg {
            duration: leg.duration,
            summary: c_string_to_option_string(leg.summary),
            weight: leg.weight,
            distance: leg.distance,
            annotation: if leg.annotation != std::ptr::null_mut() {
                unsafe { (*leg.annotation).to_annotation() }.into()
            } else {
                None
            },
            steps: if leg.steps != std::ptr::null_mut() {
                unsafe { slice::from_raw_parts(leg.steps, leg.number_of_steps as usize).to_vec() }
                    .iter()
                    .map(|step| step.into())
                    .collect()
            } else {
                Vec::new() 
            },
        }
    }
}