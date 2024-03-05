use crate::mandelbrot::clap_arguments;
use crate::mandelbrot::mandelbrot_analysis::*;
use crate::mandelbrot::mandelbrot_frame::MandelBrotFrame;
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

    if matches.get_one::<String>("sequential-rust").unwrap() == "run" {
        let (seq_data, _seq_allocation_time, seq_computation_time) = frame.compute_set();

        let infos = MandelBrotSimulationInfo {
            simulation_name: matches.get_one::<String>("name").unwrap().to_string(),
            method: "sequential-rust".to_string(),
            execution_time: seq_computation_time.as_secs_f64(),
            resolution: frame.resolution,
            iterations: frame.iterations,
        };

        if matches.get_one::<String>("visualize").unwrap() != "no_path" {
            let file_path = format!(
                "{}{}{}",
                String::from("./output_ppm/").to_string(),
                matches.get_one::<String>("name").unwrap(),
                String::from("_rust_sequential.ppm").to_string()
            );
            let _ = frame.visualize(&seq_data, file_path.as_str());
        }

        let _ = save_results(infos, "./output_csv/mandelbrot_analysis.csv");
    }

     if matches.get_one::<String>("metal").unwrap() == "run" {
        let (seq_data, _seq_allocation_time, seq_computation_time) = frame.compute_set();

        let infos = MandelBrotSimulationInfo {
            simulation_name: matches.get_one::<String>("name").unwrap().to_string(),
            method: "metal-rust".to_string(),
            execution_time: seq_computation_time.as_secs_f64(),
            resolution: frame.resolution,
            iterations: frame.iterations,
        };

        if matches.get_one::<String>("visualize").unwrap() != "no_path" {
            let file_path = format!(
                "{}{}{}",
                String::from("./output_ppm/").to_string(),
                matches.get_one::<String>("name").unwrap(),
                String::from("_rust_metal.ppm").to_string()
            );
            let _ = frame.visualize(&seq_data, file_path.as_str());
        }

        let _ = save_results(infos, "./output_csv/mandelbrot_analysis.csv");
    }

}
