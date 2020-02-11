#![allow(dead_code)]

use std::os::raw::{c_void};
use crate::engine_config::EngineConfig;

pub mod engine_config;
pub mod nearest;


#[link(name = "c_osrm")]
extern {
    fn osrm_create(config: *mut c_void) -> *mut c_void;
    fn osrm_destroy(osrm: *mut c_void);
}



pub struct Osrm
{
    config: Box<*mut c_void>
}

impl Osrm {
    pub fn new(config: &EngineConfig) -> Osrm {
        unsafe {
            Osrm {
                config: Box::new(osrm_create(*config.config))
            }
        }
    }
}

impl Drop for Osrm {
    fn drop(&mut self) {
        unsafe {
            osrm_destroy(*self.config);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Osrm;
    use crate::engine_config::{EngineConfig, Algorithm};
    use crate::nearest::*;

    //noinspection RsBorrowChecker
    #[test]
    fn it_works() {
        let mut config = EngineConfig::new();
        config.set_storage_config("/home/tehkoza/osrm/sweden-latest.osrm");
        config.set_use_shared_memory(false);
        config.set_algorithm(Algorithm::MLD);
        let osrm = Osrm::new(&config);

        for _ in 0..1 {
            let mut request = NearestRequest::new(57.792316, 13.419483);
            request.number_of_results = 30;

            let result = request.run(&osrm);

            if result.0 == Status::Ok {
                if result.1.way_points.is_some() {
                    println!("code: {}", result.1.code.unwrap());
                }

                if result.1.way_points.is_some() {
                    for waypoint in result.1.way_points.unwrap() {
                        println!("lat: {}, lon: {}, name: {}", waypoint.location[1], waypoint.location[0], waypoint.name);
                    }
                }
            } else {
                if result.1.way_points.is_some() {
                    println!("code: {}", result.1.code.unwrap());
                }
                if result.1.message.is_some() {
                    println!("message: {}", result.1.message.unwrap());
                }
            }
            println!();
        }


        assert_eq!(1,1);
    }
}
