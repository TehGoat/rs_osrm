#![allow(dead_code)]

use std::os::raw::{c_void, c_char, c_int, c_double};

pub mod nearest;
pub mod general;
pub mod table;
pub mod route;


#[link(name = "c_osrm")]
extern {
    fn osrm_create(config: *mut EngineConfig) -> *mut c_void;
    fn osrm_destroy(osrm: *mut c_void);
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Status
{
    Ok = 0,
    Error = 1
}

#[repr(C)]
pub enum Algorithm {
    CH = 0,
    CoreCH = 1, // Deprecated, will be removed in v6.0
    MLD = 2
}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub enum Boolean {
    FALSE = 0,
    TRUE = 1
}

impl Boolean {
    fn from(value: bool) -> Boolean {
        match value {
            true=> Boolean::TRUE,
            false => Boolean::FALSE
        }
    }
}

#[repr(C)]
pub struct EngineConfig{
    pub storage_config: *const c_char,
    pub max_locations_trip: c_int,
    pub max_locations_viaroute: c_int,
    pub max_locations_distance_table: c_int,
    pub max_locations_map_matching: c_int,
    pub max_radius_map_matching: c_double,
    pub max_results_nearest: c_int,
    pub max_alternatives: c_int,
    pub use_shared_memory: Boolean,
    pub memory_file: *const c_char,
    pub use_mmap: Boolean,
    pub algorithm: Algorithm,
    pub verbosity: *const c_char,
    pub dataset_name: *const c_char
}

impl EngineConfig {
    pub fn new(path: &str) -> EngineConfig {
        EngineConfig {
            storage_config: path.as_ptr() as *const i8,
            max_locations_trip: -1,
            max_locations_viaroute: -1,
            max_locations_distance_table: -1,
            max_locations_map_matching: -1,
            max_radius_map_matching: -1.0,
            max_results_nearest: -1,
            max_alternatives: 3,
            use_shared_memory: Boolean::TRUE,
            memory_file: std::ptr::null(),
            use_mmap: Boolean::TRUE,
            algorithm: Algorithm::CH,
            verbosity: std::ptr::null(),
            dataset_name: std::ptr::null(),
        }
    }

    pub fn set_storage_config(&mut self, path: &str) {
        self.storage_config = path.as_ptr() as *mut i8;
    }

    pub fn set_verbosity(&mut self, path: Option<&str>) {
        match path {
            Some(path) => {
                self.verbosity = path.as_ptr() as *mut i8;
            },
            None => {
                self.verbosity = std::ptr::null();
            }
        }
    }

    pub fn set_dataset_name(&mut self, path: Option<&str>) {
        match path {
            Some(path) => {
                self.dataset_name = path.as_ptr() as *mut i8;
            },
            None => {
                self.dataset_name = std::ptr::null();
            }
        }
    }
}

pub struct Osrm
{
    config: Box<*mut c_void>
}

impl Osrm {
    pub fn new(config: &mut EngineConfig) -> Osrm {
        unsafe {
            Osrm {
                config: Box::new(osrm_create(config as *mut EngineConfig))
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
    use crate::general::Coordinate;
use crate::{Osrm, EngineConfig, Boolean, Algorithm, Status};
    use crate::{nearest::NearestRequest, route::RouteRequest, table::TableRequest};
    
   #[test]
   fn nearest_test() {
       let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
       config.use_shared_memory = Boolean::FALSE;
       config.algorithm = Algorithm::MLD;
       let osrm = Osrm::new(&mut config);

       let mut request = NearestRequest::new(57.804404, 13.448601);
       request.number_of_results =  1;

       let result = request.run(&osrm);

       if result.0 == Status::Ok {
           if result.1.code.is_some() {
               println!("code: {}", result.1.code.unwrap());
           }

           if result.1.waypoints.is_some() {
               for waypoint in result.1.waypoints.unwrap() {
                   println!("lat: {}, lon: {}, name: {}", waypoint.location[1], waypoint.location[0], waypoint.name);
               }
           }
       } else {
           if result.1.code.is_some() {
               println!("code: {}", result.1.code.unwrap());
           }
           if result.1.message.is_some() {
               println!("message: {}", result.1.message.unwrap());
           }
       }

       assert_eq!(1,1);
   }

    #[test]
    fn table_test() {
        let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
        config.use_shared_memory = Boolean::FALSE;
        config.algorithm = Algorithm::MLD;
        let osrm = Osrm::new(&mut config);

        let mut request = TableRequest::new(&vec![
            Coordinate{
                latitude: 57.804404,
                longitude: 13.448601,
            },
            Coordinate{
                latitude: 57.772140,
                longitude: 13.408126,
            },
            Coordinate{
                latitude: 57.672140,
                longitude: 13.408126,
            }
        ]);

        let result = request.run(&osrm);


        if result.0 == Status::Ok {
            if result.1.code.is_some() {
                println!("code: {}", result.1.code.unwrap());
            }
            
            if result.1.durations.is_some() {
                for durations in result.1.durations.as_ref().unwrap() {
                        print!("[");
                        for duration in durations {
                            print!(" {} ", duration)
                        }
                    println!("]");
                }
            }
            
        } else {
            if result.1.code.is_some() {
                println!("code: {}", result.1.code.unwrap());
            }
            if result.1.message.is_some() {
                println!("message: {}", result.1.message.unwrap());
            }
        }

        assert_eq!(1,1);
    }


    #[test]
    fn route_test() {
       let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
       config.use_shared_memory = Boolean::FALSE;
       config.algorithm = Algorithm::MLD;
       let osrm = Osrm::new(&mut config);

        let mut request = RouteRequest::new(&vec![
            Coordinate{
                latitude: 57.804404,
                longitude: 13.448601,
            },
            Coordinate{
                latitude: 58.672140,
                longitude: 13.408126,
            },
            Coordinate{
                latitude: 57.772140,
                longitude: 13.408126,
            },
            Coordinate{
                latitude: 57.672140,
                longitude: 13.408126,
            }
        ]);

        request.run(&osrm);

        let result = request.run(&osrm);

        if result.0 == Status::Ok {
            if result.1.code.is_some() {
                println!("code: {}", result.1.code.unwrap());
            }
            
            for waypoint in result.1.waypoints {
                println!("lat: {}, lon: {}, name: {}", waypoint.location[1], waypoint.location[0], waypoint.name);
            }

            for route in result.1.routes {
                println!("duration: {}, distance: {}, weight: {}", route.duration, route.distance, route.weight);
            }
            
        } else {
            if result.1.code.is_some() {
                println!("code: {}", result.1.code.unwrap());
            }
            if result.1.message.is_some() {
                println!("message: {}", result.1.message.unwrap());
            }
        }
        assert_eq!(1,1);
    }
}
