use std::os::raw::{c_char, c_int};


#[repr(C)]
#[derive(Clone)]
pub(crate) struct COsrmMetaData {
    pub(crate) datasource_names: *const *const c_char,
    pub(crate) number_of_datasource_names: c_int,
}