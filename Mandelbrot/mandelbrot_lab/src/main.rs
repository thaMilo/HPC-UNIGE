use crate::mandelbrot_frame::MandelBrotFrame;
mod mandelbrot_frame;
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

    let data = frame.compute_metal();
    frame
        .visualize(data, "./mandelbrot_set.ppm")
        .expect("not able to create visualization file");
}
