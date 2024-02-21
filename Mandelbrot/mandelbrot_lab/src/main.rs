use crate::mandelbrot_frame::MandelBrotFrame;
mod mandelbrot_frame;

fn main() {
    let frame = MandelBrotFrame::new(-2, 1, -1, 1, 1000, 2, 1000);
    let data = frame.compute_set();
    frame.visualize(data, "./mandelbrot.ppm").expect("not able to create visualization file");
}
