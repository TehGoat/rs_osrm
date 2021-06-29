use std::os::raw::c_int;

#[repr(C)]
pub(crate) struct CTileResult {
    result: *const u8,
    string_length: c_int,
}

pub struct TileResult {
    pub result: Vec<u8>,
}

impl TileResult {
    pub(crate) fn new(c_result: &CTileResult) -> TileResult {
        let mut result = TileResult { result: Vec::new() };

        let converted_result = unsafe {
            std::slice::from_raw_parts(
                c_result.result as *const u8,
                c_result.string_length as usize,
            )
        };

        for value in converted_result {
            result.result.push(value.clone());
        }

        result
    }
}