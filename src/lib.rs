#![allow(dead_code)]

use core::fmt::Display;
use std::ffi::{CString};
use std::{fmt, os::raw::{c_void, c_char, c_int, c_double}};

pub mod nearest;
pub mod general;
pub mod table;
pub mod route;
pub mod match_api;
pub mod trip;
pub mod tile;


#[link(name = "c_osrm")]
extern {
    fn osrm_create(config: *const CEngineConfig) -> *mut c_void;
    fn osrm_destroy(osrm: *mut c_void);
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Status
{
    Ok = 0,
    Error = 1
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result { 
        if self.eq(&Status::Ok) {
            write!(f, "Ok")
        } else {
            write!(f, "Error")
        }
    }

}

#[repr(C)]
#[derive(Clone)]
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
pub(crate) struct CEngineConfig{
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


impl CEngineConfig {
    pub(crate) fn new(path: &CString) -> CEngineConfig {
        CEngineConfig {
            storage_config: path.as_ptr(),
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
}

pub struct EngineConfig{
    pub storage_config: String,
    pub c_storage_config: CString,
    pub max_locations_trip: i32,
    pub max_locations_viaroute: i32,
    pub max_locations_distance_table: i32,
    pub max_locations_map_matching: i32,
    pub max_radius_map_matching: f64,
    pub max_results_nearest: i32,
    pub max_alternatives: i32,
    pub use_shared_memory: bool,
    pub memory_file: Option<String>,
    c_memory_file: CString,
    pub use_mmap: bool,
    pub algorithm: Algorithm,
    pub verbosity: Option<String>,
    c_verbosity: CString,
    pub dataset_name: Option<String>,
    c_dataset_name: CString,
}

impl EngineConfig {
    pub fn new(path: &str) -> EngineConfig {
        EngineConfig {
            storage_config: path.to_string(),
            c_storage_config: CString::new(path.clone()).unwrap(),
            max_locations_trip: -1,
            max_locations_viaroute: -1,
            max_locations_distance_table: -1,
            max_locations_map_matching: -1,
            max_radius_map_matching: -1.0,
            max_results_nearest: -1,
            max_alternatives: 3,
            use_shared_memory: true,
            memory_file: None,
            c_memory_file: CString::default(),
            use_mmap: true,
            algorithm: Algorithm::CH,
            verbosity: None,
            c_verbosity: CString::default(),
            dataset_name: None,
            c_dataset_name: CString::default(),
        }
    }

    pub(crate) fn to_cengine_config(&mut self) -> CEngineConfig {
        self.c_storage_config = CString::new(self.storage_config.clone()).unwrap();
        let mut cengine_config = CEngineConfig::new(&self.c_storage_config );

        cengine_config.max_alternatives = self.max_alternatives;
        cengine_config.max_locations_viaroute = self.max_locations_viaroute;
        cengine_config.max_locations_distance_table = self.max_locations_distance_table;
        cengine_config.max_locations_map_matching = self.max_locations_map_matching;
        cengine_config.max_radius_map_matching = self.max_radius_map_matching;
        cengine_config.max_results_nearest = self.max_results_nearest;


        if self.use_shared_memory {
            cengine_config.use_shared_memory = Boolean::TRUE;
        } else {
            cengine_config.use_shared_memory = Boolean::FALSE;
        }

        match  &self.memory_file {
            Some(memory_file_string) => {
                self.c_memory_file = CString::new(memory_file_string.clone()).unwrap();
                
                cengine_config.memory_file = self.c_memory_file.as_ptr();
            },
            None => {}
        }

        if self.use_mmap {
            cengine_config.use_mmap = Boolean::TRUE;
        } else {
            cengine_config.use_mmap = Boolean::FALSE;
        }

        cengine_config.algorithm = self.algorithm.clone();

        match  &self.verbosity {
            Some(verbosity_string) => {
                self.c_verbosity = CString::new(verbosity_string.clone()).unwrap();

                cengine_config.verbosity = self.c_verbosity.as_ptr();
            },
            None => {}
        }

        match  &self.dataset_name {
            Some(dataset_name_string) => {
                self.c_dataset_name= CString::new(dataset_name_string.clone())
                .expect("to_CEngineConfig::dataset_name::new failed");

                cengine_config.dataset_name = self.c_dataset_name.as_ptr();
            },
            None => {}
        }



        cengine_config
    }
}

pub struct Osrm
{
    config: Box<*mut c_void>
}

impl Osrm {
    pub fn new(config: &mut EngineConfig) -> Osrm {
        let c_engine_config = config.to_cengine_config();
        unsafe {
            Osrm {
                config: Box::new(osrm_create(&c_engine_config as *const CEngineConfig))
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

unsafe impl Send for Osrm {
    
}

#[cfg(test)]
mod tests {
    use crate::general::Coordinate;
use crate::{Osrm, EngineConfig, Algorithm, Status};
    use crate::{nearest::NearestRequest, route::RouteRequest, table::TableRequest, match_api::MatchRequest, trip::TripRequest, tile::TileRequest};
    use std::{io::Write, fs::File};
    
   #[test]
   fn nearest_test() {
       let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
       config.use_shared_memory = false;
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
        config.use_shared_memory = false;
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
       config.use_shared_memory = false;
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

    #[test]
    fn match_test() {
       let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
       config.use_shared_memory = false;
       config.algorithm = Algorithm::MLD;
       let osrm = Osrm::new(&mut config);

        let mut request = MatchRequest::new(&vec![
            Coordinate{
                latitude: 57.781092,
                longitude: 13.429555,
            },
            Coordinate{
                latitude: 57.781123,
                longitude: 13.427192,
            }
        ]);

        request.run(&osrm);

        let result = request.run(&osrm);

        if result.0 == Status::Ok {
            if result.1.code.is_some() {
                println!("code: {}", result.1.code.unwrap());
            }
            
            for waypoint in result.1.tracepoints {
                println!("lat: {}, lon: {}, name: {}", waypoint.location[1], waypoint.location[0], waypoint.name);
            }

            for route in result.1.matchings {
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

    #[test]
    fn trip_test() {
       let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
       config.use_shared_memory = false;
       config.algorithm = Algorithm::MLD;
       let osrm = Osrm::new(&mut config);

        let mut request = TripRequest::new(&vec![
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

            for route in result.1.trips {
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

    #[test]
    fn tile_test() {
       let mut config = EngineConfig::new("/home/tehkoza/osrm/sweden-latest.osrm");
       config.use_shared_memory = false;
       config.algorithm = Algorithm::MLD;
       let osrm = Osrm::new(&mut config);

        let mut request = TileRequest::new(35342,19818, 16);

        request.run(&osrm);

        let result = request.run(&osrm);

        if result.0 == Status::Ok {
            println!("Tile Ok!");
            let mut buffer = File::create("/home/tehkoza/foo.txt").expect("Faile to create file!");
            
            buffer.write(&result.1.result).expect("Failed to write data!");
        } else {
            println!("Tile Failed!");
        }
        assert_eq!(1,1);
    }

}
