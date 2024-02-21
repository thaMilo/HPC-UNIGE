use anyhow::Result;
use num_complex::Complex;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct MandelBrotFrame {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    resolution: i32,
    width: i32,
    height: i32,
    step: f64,
    degree: i32,
    iterations: i32,
}

impl MandelBrotFrame {
    pub fn new(
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
        resolution: i32,
        degree: i32,
        iterations: i32,
    ) -> Self {
        let width = (max_x - min_x) * resolution;
        let height = (max_y - min_y) * resolution;
        let step = (max_x - min_x) as f64 / width as f64;
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

    pub fn compute_set(&self) -> Vec<i32> {
        let mut image = vec![0; (self.width * self.height) as usize];
        for pos in 0..self.width * self.height {
            image[pos as usize] = 0;
            let row = pos / self.width;
            let col = pos % self.width;
            let constant_complex_number = Complex::new(
                self.min_x as f64 + col as f64 * self.step,
                self.min_y as f64 + row as f64 * self.step,
            );

            let mut z = Complex::new(0.0, 0.0);
            for i in 0..self.iterations {
                z = z.powi(self.degree) + constant_complex_number;
                if z.norm() > 2.0 {
                    image[pos as usize] = i;
                    break;
                }
            }
        }

        image
    }

    pub fn visualize(&self, data: Vec<i32>, filepath: &str) -> Result<(), anyhow::Error> {
        let mut f = fs::File::create(filepath)?;
        f.write_all(b"P3\n")?;
        f.write_all(format!("{} {}\n", self.width, self.height).as_bytes())?;
        f.write_all(b"255\n")?;

        for row in 0..self.height {
            for col in 0..self.width {
                let val = data[(row * self.width + col) as usize];
                let refined_val = val.min(255);
                let ig = (refined_val as f64 * (1000.0 / 255.0)) as i32;
                let ib = (refined_val as f64 * (1000.0 / 255.0)) as i32;
                f.write_all(format!("0 {} {}", ig, ib).as_bytes())?;

                if col < self.width - 1 {
                    f.write_all(b"\n")?;
                }
            }
            f.write_all(b"\n")?;
        }
        Ok(())
    }
}
