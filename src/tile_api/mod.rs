use std::ffi::c_void;

use crate::Status;

use self::{tile_request::CTileRequest, tile_result::CTileResult};

pub mod tile_request;
pub mod tile_result;

#[link(name = "c_osrm")]
extern "C" {
    fn tile_result_destroy(result: *mut CTileResult);

    fn osrm_tile(
        osrm: *mut c_void,
        request: *mut CTileRequest,
        result: *mut *mut CTileResult,
    ) -> Status;
}