use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
	let src_dir = Path::new("mandelbrot/metal");
    let metal_path = src_dir.join("mandelbrot.metal");
    let metallib_path = src_dir.join("mandelbrot.metallib");
	let air_path = src_dir.join("mandelbrot.air");

    // Only compile if the .metal file is newer than the .metallib file
    if metallib_path.exists() && fs::metadata(metal_path.clone()).unwrap().modified().unwrap() <= fs::metadata(metallib_path.clone()).unwrap().modified().unwrap() {
        return;
    }

    // Compile the .metal file to a .metallib file
    Command::new("xcrun")
        .args(&["metal", "-c", metal_path.to_str().unwrap(), "-o", air_path.to_str().unwrap()])
        .status()
        .unwrap();

    Command::new("xcrun")
        .args(&["metallib", air_path.to_str().unwrap(), "-o", metallib_path.to_str().unwrap()])
        .status()
        .unwrap();

    // Tell Cargo to rerun this script if the .metal file has changed
    println!("cargo:rerun-if-changed={}", metal_path.to_str().unwrap());
}