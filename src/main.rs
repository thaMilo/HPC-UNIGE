#[macro_use]
extern crate bincode;

use crate::mandelbrot::clap_arguments;
use crate::mandelbrot::mandelbrot_analysis::*;
use crate::mandelbrot::mandelbrot_frame::MandelBrotFrame;

use std::fs::File;
use bincode::serialize_into;
use std::io::BufWriter;
mod mandelbrot;

fn main() {
    let matches = clap_arguments::get_clap_arguments();
    let frame = MandelBrotFrame::new(
        matches
            .get_one::<String>("minx")
            .unwrap()
            .parse::<i32>()
            .expect("minx is not a number"),
        matches
            .get_one::<String>("maxx")
            .unwrap()
            .parse::<i32>()
            .expect("maxx is not a number"),
        matches
            .get_one::<String>("miny")
            .unwrap()
            .parse::<i32>()
            .expect("miny is not a number"),
        matches
            .get_one::<String>("maxy")
            .unwrap()
            .parse::<i32>()
            .expect("maxy is not a number"),
        matches
            .get_one::<String>("resolution")
            .unwrap()
            .parse::<i32>()
            .expect("resolution is not a number"),
        matches
            .get_one::<String>("degree")
            .unwrap()
            .parse::<i32>()
            .expect("degree is not a number"),
        matches
            .get_one::<String>("iterations")
            .unwrap()
            .parse::<i32>()
            .expect("iterations is not a number"),
    );

    // --save data to binary file--
    // let (seq_data, _seq_allocation_time, seq_computation_time) = frame.compute_set();
    // let mut f = BufWriter::new(File::create("./test.bin").unwrap());
    // serialize_into(&mut f, &seq_data).unwrap();

    if matches.get_one::<String>("sequential-rust").unwrap() == "run" {
        let (seq_data, _seq_allocation_time, seq_computation_time) = frame.compute_set();

        let mut error = MandelBrotError{
            error_vector: vec![],
            divergent_pixels: 0,
            mean: 0.0,
            variance: 0,
            accuracy: 0,
        };
        
        if matches.get_one::<String>("error").unwrap() == "run" {
            error = compute_error(&seq_data).unwrap();
            if matches.get_one::<String>("visualize-error").unwrap() == "run" {
                let file_path = format!(
                    "{}{}{}",
                    String::from("./output_error/").to_string(),
                    matches.get_one::<String>("name").unwrap(),
                    String::from("_rust_error.ppm").to_string()
                );
                let _ = frame.visualize(&error.error_vector, file_path.as_str(), true);
            }
        }

        let infos = MandelBrotSimulationInfo {
            simulation_name: matches.get_one::<String>("name").unwrap().to_string(),
            method: "sequential-rust".to_string(),
            execution_time: seq_computation_time.as_secs_f64(),
            resolution: frame.resolution,
            iterations: frame.iterations,
            accuracy: error.accuracy,
        };

        if matches.get_one::<String>("visualize").unwrap() == "run" {
            let file_path = format!(
                "{}{}{}",
                String::from("./output_ppm/").to_string(),
                matches.get_one::<String>("name").unwrap(),
                String::from("_rust_sequential.ppm").to_string()
            );
            let _ = frame.visualize(&seq_data, file_path.as_str(), false);
        }

        let _ = save_results(infos, "./output_csv/mandelbrot_analysis.csv");
    }

     if matches.get_one::<String>("metal").unwrap() == "run" {
        let (metal_data, _metal_allocation_time, metal_computation_time) = frame.compute_metal();

        let mut error = MandelBrotError{
            error_vector: vec![],
            divergent_pixels: 0,
            mean: 0.0,
            variance: 0,
            accuracy: 0,
        };
        
        if matches.get_one::<String>("error").unwrap() == "run" {
            error = compute_error(&metal_data).unwrap();
            if matches.get_one::<String>("visualize-error").unwrap() == "run" {
                let file_path = format!(
                    "{}{}{}",
                    String::from("./output_error/").to_string(),
                    matches.get_one::<String>("name").unwrap(),
                    String::from("_rust_error.ppm").to_string()
                );
                let _ = frame.visualize(&error.error_vector, file_path.as_str(), true);
            }
        }

        let infos = MandelBrotSimulationInfo {
            simulation_name: matches.get_one::<String>("name").unwrap().to_string(),
            method: "metal-rust".to_string(),
            execution_time: metal_computation_time.as_secs_f64(),
            resolution: frame.resolution,
            iterations: frame.iterations,
            accuracy: error.accuracy,
        };

        if matches.get_one::<String>("visualize").unwrap() == "run" {
            let file_path = format!(
                "{}{}{}",
                String::from("./output_ppm/").to_string(),
                matches.get_one::<String>("name").unwrap(),
                String::from("_rust_metal.ppm").to_string()
            );
            let _ = frame.visualize(&metal_data, file_path.as_str(), false);
        }

        let _ = save_results(infos, "./output_csv/mandelbrot_analysis.csv");
    }

}
