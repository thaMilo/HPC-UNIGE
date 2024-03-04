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
    
    let infos = MandelBrotSimulationInfo {
        methods: vec!["sequential".to_string(), "metal".to_string(), "metal".to_string()],
        execution_times: vec![1000, 100, 1000],
        resolutions: vec![frame.resolution, frame.resolution, frame.resolution],
        iterations: vec![frame.iterations, frame.iterations, frame.iterations],
    };
    
    /* computations */ 

    let _ = save_results( infos, "./src/mandelbrot/output_csv/mandelbrot_analysis.csv");  
}
