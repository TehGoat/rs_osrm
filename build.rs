use cmake::Config;

fn main() {

    let dst = Config::new("c_osrm")
        .build_target("")
        .build();

    println!("cargo:rustc-link-search={}/build", dst.display());
    println!("cargo:rustc-flags=-lstdc++");
}