use rs_osrm::nearest::*;
use rs_osrm::engine_config::EngineConfig;
use rs_osrm::engine_config::Algorithm;
use rs_osrm::nearest::{NearestRequest, Status};
use rs_osrm::Osrm;

//noinspection RsBorrowChecker
fn main() {
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

}