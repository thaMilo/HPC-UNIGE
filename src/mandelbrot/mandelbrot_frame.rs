use anyhow::Result;
use metal::{Device, DeviceRef, MTLResourceOptions};
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

    pub fn compute_metal(&self, thread_count: &mut u64) -> (Vec<i32>, Duration, Duration) {
        let capture_scope = metal::CaptureManager::shared()
            .new_capture_scope_with_device(&metal::Device::system_default().unwrap());

        let capture_descriptor = metal::CaptureDescriptor::new();
        capture_descriptor.set_capture_scope(&capture_scope);
        capture_descriptor.set_output_url(std::path::Path::new("./gputrace/framecapture.gputrace"));
        capture_descriptor.set_destination(metal::MTLCaptureDestination::GpuTraceDocument);
        metal::CaptureManager::shared().start_capture(&capture_descriptor);

        let metal_allocation_started: Instant = Instant::now();
        let device: &DeviceRef = &Device::system_default().expect("No device found");
        let lib: metal::Library = device.new_library_with_data(LIB_DATA).unwrap();
        let function: metal::Function = lib.get_function("generateMandelbrotSet", None).unwrap();
        let pipeline: metal::ComputePipelineState = device
            .new_compute_pipeline_state_with_function(&function)
            .unwrap();

        let buffer_step = device.new_buffer_with_data(
            unsafe { std::mem::transmute(&self.step) },
            std::mem::size_of::<f32>() as u64,
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
            (self.width * self.height * std::mem::size_of::<i32>() as i32) as u64,
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
        let grid_size = metal::MTLSize::new(self.width as u64, self.height as u64, 1);
        let max_thread_count = pipeline.max_total_threads_per_threadgroup();

        if *thread_count > max_thread_count {
            *thread_count = max_thread_count;
        }

        let threadgroup_size = metal::MTLSize::new(*thread_count, 1, 1);
        compute_encoder.dispatch_threads(grid_size, threadgroup_size);
        compute_encoder.end_encoding();

        capture_scope.begin_scope();
        let metal_computation_started = Instant::now();

        // commit the command buffer and wait for it to complete
        command_buffer.commit();
        command_buffer.wait_until_completed();
        let metal_computation_total_time = metal_computation_started.elapsed();
        capture_scope.end_scope();

        let ptr = buffer_result.contents() as *const i32;
        let len = buffer_result.length() as usize / std::mem::size_of::<i32>();
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        let metal_allocation_total_time = metal_allocation_started.elapsed();

        (
            slice.to_vec(),
            metal_allocation_total_time,
            metal_computation_total_time,
        )
    }

    pub fn compute_set(&self) -> (Vec<i32>, Duration, Duration) {
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
        (
            image,
            sequential_allocation_total_time,
            sequential_computation_total_time,
        )
    }

    pub fn visualize(
        &self,
        data: &Vec<i32>,
        filepath: &str,
        error: bool,
    ) -> Result<(), anyhow::Error> {
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
                    if val == 0 {
                        f.write_all(b"0 0 0")?;
                    } else {
                        let ir = (1000.0 / (val as f64)) % 255.0;
                        let ig = (1000.0 / (val as f64) + 85.0) % 255.0;
                        let ib = (1000.0 / (val as f64) + 170.0) % 255.0;
                        f.write_all(
                            format!("{} {} {}", ir as i32, ig as i32, ib as i32).as_bytes(),
                        )?;
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
