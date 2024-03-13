use anyhow::Result;
use num_complex::Complex;
use std::fs;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;

const LIB_DATA: &[u8] = include_bytes!("metal/mandelbrot.metallib");

pub struct MandelBrotFrame {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub resolution: i32,
    pub width: i32,
    pub height: i32,
    pub step: f32,
    pub degree: i32,
    pub iterations: i32,
}

impl MandelBrotFrame {
    pub fn new(
        min_x: i32,      // -2
        max_x: i32,      //  1
        min_y: i32,      // -1
        max_y: i32,      //  1
        resolution: i32, // 1000
        degree: i32,     // 2
        iterations: i32, // 1000
    ) -> Self {
        let width = (max_x - min_x) * resolution;
        let height = (max_y - min_y) * resolution;
        let step = (max_x - min_x) as f32 / width as f32;
        MandelBrotFrame {
            min_x,
            max_x,
            min_y,
            max_y,
            resolution,
            width,
            height,
            step,
            degree,
            iterations,
        }
    }

    pub fn compute_set(&self) -> (Vec<i32>, Duration, Duration){
        let sequential_allocation_started = Instant::now();
        let mut image = vec![0; (self.width * self.height) as usize];
        let sequential_computation_started = Instant::now();
        for pos in 0..self.width * self.height {
            image[pos as usize] = 0;
            let row = pos / self.width;
            let col = pos % self.width;
            let constant_complex_number = Complex::new(
                self.min_x as f32 + col as f32 * self.step,
                self.min_y as f32 + row as f32 * self.step,
            );

            let mut z = Complex::new(0.0, 0.0);
            for i in 0..self.iterations {
                z = z.powi(self.degree) + constant_complex_number;
                if z.norm() >= 2.0 {
                    image[pos as usize] = i;
                    break;
                }
            }
        }
        let sequential_computation_total_time = sequential_computation_started.elapsed();
        let sequential_allocation_total_time = sequential_allocation_started.elapsed();
        (image, sequential_allocation_total_time, sequential_computation_total_time)
    }

    pub fn visualize(&self, data: &Vec<i32>, filepath: &str, error: bool) -> Result<(), anyhow::Error> {
        let mut f = fs::File::create(filepath)?;
        f.write_all(b"P3\n")?;
        f.write_all(format!("{} {}\n", self.width, self.height).as_bytes())?;
        f.write_all(b"255\n")?;

        for row in 0..self.height {
            for col in 0..self.width {
                let val = data[(row * self.width + col) as usize];
                if error {
                    let ir = (val as f32 * (255.0 / self.iterations as f32)) as i32;
                    f.write_all(format!("{} {} {}", ir, ir, ir).as_bytes())?;
                } else {
                    if val == 0{
                        f.write_all(b"0 0 0")?;
                    } else {
                        let ir = (1000.0/(val as f64)) % 255.0;
                        let ig = (1000.0/((val as f64)) + 85.0) % 255.0;
                        let ib = (1000.0/((val as f64)) + 170.0) % 255.0;
                        f.write_all(format!("{} {} {}", ir as i32, ig as i32, ib as i32).as_bytes())?;
                    }
                }

                if col < self.width - 1 {
                    f.write_all(b"\n")?;
                }
            }
            f.write_all(b"\n")?;
        }
        Ok(())
    }
}
