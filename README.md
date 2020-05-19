# rs_osrm
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/rs_osrm.svg
[crates-url]: https://crates.io/crates/rs_osrm
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Rust wrapper for osrm

### Requeries that osrm's dependencies is installed

### How to use:
1. Create an EngineConfig, pass path to .osrm file. You may change other settings, see osrm documentation.
2. Create an Osrm via Osrm::new and pass in the config.
3. Create a request object (ex: NearestRequest), recomended to always use ::new to get correct default values.
4. Call run on the request object and pass in osrm.

### Nearest example:
```rust
use rs_osrm::{EngineConfig, Osrm, Algorithm, Status, nearest::NearestRequest};

fn main() {
    let mut config = EngineConfig::new("<PATH TO .osrm file>");
    config.use_shared_memory = false;
    config.algorithm = Algorithm::MLD;
    let osrm = Osrm::new(&mut config);
    
    let mut request = NearestRequest::new(57.804404, 13.448601);
    request.number_of_results =  3;

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

}


```