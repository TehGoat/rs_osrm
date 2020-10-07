# rsc_osrm
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/rs_osrm.svg
[crates-url]: https://crates.io/crates/rsc_osrm
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Rust wrapper for osrm

### Requeries that osrm's dependencies is installed
to link final binary, you need:
0. libosrm.a
1. libboost_filesystem.so
2. libboost_iostreams.so
3. libboost_thread.so
4. libboost_system.so

### How to use:
1. Create an EngineConfig, pass path to .osrm file. You may change other settings, see osrm documentation.
2. Create an Osrm via Osrm::new and pass in the config.
3. Create a request object (ex: NearestRequest), recomended to always use ::new to get correct default values.
4. Call run on the request object and pass in osrm.

### Nearest example:
```rust
use rsc_osrm::{EngineConfig, Osrm, Algorithm, Status, route::RouteRequest, general::Coordinate};

fn main() {
    let mut config = EngineConfig::new("<path to your .osrm file>");
    config.use_shared_memory = false;
    config.algorithm = Algorithm::MLD;
    let osrm = Osrm::new(&mut config).unwrap();
    let coords = vec!(Coordinate{latitude:12.98657118,longitude:77.56644753}, Coordinate{latitude:12.97436012,longitude:77.62567071});
    let mut request = RouteRequest::new(&coords);
    let (status,result) = request.run(&osrm);
    match status{
        Status::Ok => {
            let route0 = &result.routes[0];
            println!("eta: {}, eda: {}, geometry: {}",route0.duration, route0.distance, (route0.geometry).as_ref().unwrap());
        }
        _ => println!("call osrm failed"),
    }
}

```
