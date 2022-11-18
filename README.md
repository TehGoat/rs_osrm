# rs_osrm
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/rs_osrm.svg
[crates-url]: https://crates.io/crates/rs_osrm
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Rust wrapper for osrm

### Requeries that osrm's dependencies is installed

### How to use:
1. Create an EngineConfigBulter, pass path to .osrm file. You may change other settings, see osrm documentation.
2. Create a request object (ex: NearestRequest) using builder (ex: NearestRequestBuilder)
3. Call run on the request object and pass in osrm.

### Nearest example:
```rust
 use crate::{
    engine_config::engine_config_builder::EngineConfigBuilder,
    nearest_api::nearest_request_builder::NearestRequestBuilder, Status,
};

fn main() {
    let osrm_result = EngineConfigBuilder::new("<PATH TO .osrm FILE>")
        .set_use_shared_memory(false)
        .set_algorithm(crate::Algorithm::MLD)
        .build();

    match osrm_result {
        Ok(osrm) => {
            let request = NearestRequestBuilder::new(57.804404, 13.448601)
                .set_number_of_results(3)
                .build();

            match request {
                Ok(mut nearest_request) => {
                    let (status, nearest_result) = nearest_request.run(&osrm);

                    if status == Status::Ok {
                        if nearest_result.code.is_some() {
                            println!("code: {}", nearest_result.code.unwrap());
                        }

                        if nearest_result.waypoints.is_some() {
                            for waypoint in nearest_result.waypoints.unwrap() {
                                println!(
                                    "lat: {}, lon: {}, name: {}",
                                    waypoint.location[1], waypoint.location[0], waypoint.name
                                );
                            }
                        }
                    } else {
                        if nearest_result.code.is_some() {
                            println!("code: {}", nearest_result.code.unwrap());
                        }
                        if nearest_result.message.is_some() {
                            println!("message: {}", nearest_result.message.unwrap());
                        }
                    }
                }
                Err(request_error) => {
                    eprintln!("{request_error}");
                }
            }
        }
        Err(osrm_error) => {
            eprintln!("{osrm_error}");
        }
    }
}


```