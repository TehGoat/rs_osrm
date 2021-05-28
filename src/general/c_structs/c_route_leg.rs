use std::{
    os::raw::{c_char, c_double, c_int},
};


use super::{c_annotation::COsrmAnnotation, c_step::COsrmStep};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmRouteLeg {
    pub(crate) annotation: *const COsrmAnnotation,
    pub(crate) duration: c_double,
    pub(crate) summary: *const c_char,
    pub(crate) weight: c_double,
    pub(crate) distance: c_double,
    pub(crate) steps: *const COsrmStep,
    pub(crate) number_of_steps: c_int
}