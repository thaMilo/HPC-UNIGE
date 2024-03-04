use crate::mandelbrot::clap_arguments;
use crate::mandelbrot::mandelbrot_analysis;
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

    /* let (sequential_data, sequential_allocation_time, sequential_computation_time) = frame.compute_set(); */
    // let _ = frame.visualize(&sequential_data, "./mandelbrot/output_ppm/sequential.ppm");

    let (metal_data, metal_allocation_time, metal_computation_time) = frame.compute_metal();
    
    println!("Metal allocation time: {:?}", metal_allocation_time);
    println!("Metal computation time: {:?}", metal_computation_time);
    
    let _ = frame.visualize(&metal_data, "./mandelbrot/output_ppm/metal.ppm");
    
    /*     let error_data  : mandelbrot_analysis::MandelBrotError = mandelbrot_analysis::compute_error(&sequential_data, &metal_data).expect("The vectors have different lengths"); */
    // let _ = frame.visualize(&error_data.error_vector, "./mandelbrot/output_ppm/error.ppm");
    // println!("Divergent pixels: {}", error_data.divergent_pixels);
    // println!("Mean: {}", error_data.mean);
    // println!("Variance: {}", error_data.variance);
}
