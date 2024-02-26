use anyhow::Result;
use metal::{Device, DeviceRef, MTLResourceOptions};
use num_complex::Complex;
use std::fs;
use std::io::Write;

const LIB_DATA: &[u8] = include_bytes!("metal/mandelbrot.metallib");

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

    pub fn compute_metal(&self) -> Vec<i32> {
        let device: &DeviceRef = &Device::system_default().expect("No device found");
        let lib = device.new_library_with_data(LIB_DATA).unwrap();
        let function = lib.get_function("generateMandelbrotSet", None).unwrap();
        let pipeline = device
            .new_compute_pipeline_state_with_function(&function)
            .unwrap();

        let buffer_step = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.step) },
            std::mem::size_of::<f64>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let buffer_min_x = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.min_x) },
            std::mem::size_of::<i32>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let buffer_min_y = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.min_y) },
            std::mem::size_of::<i32>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let buffer_width = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.width) },
            std::mem::size_of::<i32>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let buffer_height = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.height) },
            std::mem::size_of::<i32>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let buffer_iterations = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.iterations) },
            std::mem::size_of::<i32>() as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let buffer_result = device.new_buffer(
            (self.width * self.height * 4) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let command_queue = device.new_command_queue();
        let command_buffer = command_queue.new_command_buffer();
        let compute_encoder = command_buffer.new_compute_command_encoder();

        compute_encoder.set_compute_pipeline_state(&pipeline);
        compute_encoder.set_buffers(
            0,
            &[
                Some(&buffer_result),
                Some(&buffer_step),
                Some(&buffer_min_x),
                Some(&buffer_min_y),
                Some(&buffer_width),
                Some(&buffer_height),
                Some(&buffer_iterations),
            ],
            &[0; 7],
        );

        // specify thread count and organization
        let grid_size = metal::MTLSize::new((self.width * self.height) as u64, 1, 1);
        let threadgroup_size = metal::MTLSize::new(1024, 1, 1);
        compute_encoder.dispatch_threads(grid_size, threadgroup_size);
        compute_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let ptr = buffer_result.contents() as *const i32;
        let len = buffer_result.length() as usize / std::mem::size_of::<i32>();
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        slice.to_vec()
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
