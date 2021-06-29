use std::os::raw::c_int;

use crate::{Osrm, Status};

use super::{tile_result::{CTileResult, TileResult}, tile_result_destroy, osrm_tile};

#[repr(C)]
pub(crate) struct CTileRequest {
    x: c_int,
    y: c_int,
    z: c_int,
}

impl CTileRequest {
    fn new(request: &mut TileRequest) -> CTileRequest {
        CTileRequest {
            x: request.x,
            y: request.y,
            z: request.z,
        }
    }
}

pub struct TileRequest {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl TileRequest {
    pub fn new(x: i32, y: i32, z: i32) -> TileRequest {
        TileRequest { x, y, z }
    }

    pub fn run(&mut self, osrm: &Osrm) -> (Status, TileResult) {
        unsafe {
            let mut result: *mut CTileResult = std::ptr::null_mut();
            let result_ptr: *mut *mut CTileResult = &mut result;

            let status = osrm_tile(
                *osrm.config,
                &mut CTileRequest::new(self) as *mut CTileRequest,
                result_ptr,
            );

            let converted_result = TileResult::new(&(*result));

            tile_result_destroy(result);

            (status, converted_result)
        }
    }
}