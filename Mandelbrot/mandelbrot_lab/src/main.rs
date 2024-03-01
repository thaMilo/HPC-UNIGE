use crate::mandelbrot::mandelbrot_frame::MandelBrotFrame;
use crate::mandelbrot::mandelbrot_analysis;

mod mandelbrot;

use clap::{command, Arg};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("minx").short('x').long("minx").default_value("-2"))
        .arg(Arg::new("maxx").short('X').long("maxx").default_value("1"))
        .arg(Arg::new("miny").short('y').long("miny").default_value("-1"))
        .arg(Arg::new("maxy").short('Y').long("maxy").default_value("1"))
        .arg(
            Arg::new("resolution")
                .short('r')
                .long("resolution")
                .default_value("1000"),
        )
        .arg(
            Arg::new("degree")
                .short('d')
                .long("degree")
                .default_value("2"),
        )
        .arg(
            Arg::new("iterations")
                .short('i')
                .long("iterations")
                .default_value("1000"),
        )
        .get_matches();

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

    let sequential_data : Vec<i32> = frame.compute_set();
    let _ = frame.visualize(&sequential_data, "./mandelbrot/output_ppm/sequential.ppm");
    let metal_data : Vec<i32> = frame.compute_metal();
    let _ = frame.visualize(&metal_data, "./mandelbrot/output_ppm/metal.ppm");
    let error_data  : mandelbrot_analysis::MandelBrotError = mandelbrot_analysis::compute_error(&sequential_data, &metal_data).expect("The vectors have different lengths");
    let _ = frame.visualize(&error_data.error_vector, "./mandelbrot/output_ppm/error.ppm");
    
    println!("Divergent pixels: {}", error_data.divergent_pixels);
    println!("Mean: {}", error_data.mean);
    println!("Variance: {}", error_data.variance);
}
