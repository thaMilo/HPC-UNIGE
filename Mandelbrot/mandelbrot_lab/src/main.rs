use crate::mandelbrot_frame::MandelBrotFrame;
mod mandelbrot_frame;

fn main() {
    let frame = MandelBrotFrame::new(-2, 1, -1, 1, 1000, 2, 1000);
    let data = frame.compute_metal();
    frame.visualize(data, "./mandelbrot_metal.ppm").expect("not able to create visualization file");
}
