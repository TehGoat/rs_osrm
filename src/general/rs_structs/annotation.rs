use std::slice;

use crate::general::c_structs::c_annotation::COsrmAnnotation;

use super::meta_data::MetaData;

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

impl From<&COsrmAnnotation> for Annotation {
    fn from(c_annotation: &COsrmAnnotation) -> Self {
        Annotation {
            duration: if c_annotation.duration != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_annotation.duration,
                        (c_annotation.number_of_coordinates + 1) as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            distance: if c_annotation.distance != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_annotation.distance,
                        (c_annotation.number_of_coordinates + 1) as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            speed: if c_annotation.speed != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_annotation.speed,
                        (c_annotation.number_of_coordinates + 1) as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            weight: if c_annotation.weight != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_annotation.weight,
                        (c_annotation.number_of_coordinates + 1) as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            nodes: if c_annotation.nodes != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_annotation.nodes,
                        (c_annotation.number_of_coordinates + 1) as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            datasources: if c_annotation.datasources != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_annotation.datasources,
                        (c_annotation.number_of_coordinates + 1) as usize,
                    )
                    .to_vec()
                }
            } else {
                Vec::new()
            },
            metadata: if c_annotation.metadata != std::ptr::null_mut() {
                Option::from(unsafe { let data: MetaData = (&(*c_annotation.metadata)).into(); data })
            } else {
                None
            },
        }
    }
}