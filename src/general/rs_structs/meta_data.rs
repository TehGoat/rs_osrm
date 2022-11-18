use std::slice;

use crate::general::{c_string_to_string, c_structs::c_meta_data::COsrmMetaData};


#[derive(Debug)]
pub struct MetaData {
    datasource_names: Vec<String>,
}

impl From<&COsrmMetaData> for MetaData {
    fn from(c_meta_data: &COsrmMetaData) -> Self {
        MetaData {
            datasource_names: if c_meta_data.datasource_names != std::ptr::null_mut() {
                unsafe {
                    slice::from_raw_parts(
                        c_meta_data.datasource_names,
                        c_meta_data.number_of_datasource_names as usize,
                    )
                    .to_vec()
                }
                .iter()
                .map(|data| c_string_to_string(*data))
                .collect()
            } else {
                Vec::new()
            },
        }
    }
}