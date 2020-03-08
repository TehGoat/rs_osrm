use crate::Osrm;
use std::os::raw::{c_int, c_void};
use crate::Status;

#[link(name = "c_osrm")]
extern {
    fn tile_result_destroy(result: *mut CTileResult);

    fn osrm_tile(osrm: *mut c_void, request: *mut CTileRequest, result: *mut *mut CTileResult) -> Status;
}

#[repr(C)]
struct CTileRequest {
    x: c_int,
    y: c_int,
    z: c_int,
}

impl CTileRequest {
    fn new(request: &mut TileRequest) -> CTileRequest {
        CTileRequest{
            x: request.x,
            y: request.y,
            z: request.z,
        }
    }
}

pub struct TileRequest {
    x: i32,
    y: i32,
    z: i32,
}

impl TileRequest {
    pub fn new(x: i32, y: i32, z: i32) -> TileRequest{
        TileRequest{
            x,
            y,
            z
        }
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, TileResult) {
        unsafe {
            let mut result: *mut CTileResult = std::ptr::null_mut();
            let result_ptr : *mut *mut CTileResult = &mut result;

            let status = osrm_tile(*osrm.config,
                                    &mut CTileRequest::new(self) as *mut CTileRequest,
                                    result_ptr);

            let converted_result = TileResult::new(&(*result));

            tile_result_destroy(result);

            (status, converted_result)

        }
    }
}

#[repr(C)]
struct CTileResult
{
    result: *const u8,
    string_length: c_int
}

pub struct TileResult {
    pub result: Vec<u8>
}

impl TileResult {
    fn new(c_result: &CTileResult) -> TileResult {

        let mut result = TileResult{
            result: Vec::new()
        };

        let converted_result = 
        unsafe {
            std::slice::from_raw_parts(c_result.result as *const u8, c_result.string_length as usize)
        };

        for value in converted_result {
            result.result.push(value.clone());
        }

        result

    }
}

