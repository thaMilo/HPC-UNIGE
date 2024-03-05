use anyhow::{Ok, Result};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub struct MandelBrotError {
    pub error_vector: Vec<i32>,
    pub divergent_pixels: i64,
    pub mean: f64,
    pub variance: i64,
    pub accuracy: i64,
}

pub struct MandelBrotSimulationInfo {
    pub simulation_name: String,
    pub method: String,
    pub execution_time: f64,
    pub resolution: i32,
    pub iterations: i32,
}

pub fn compute_error(
    output_1: &Vec<i32>,
    output_2: &Vec<i32>,
) -> Result<MandelBrotError, anyhow::Error> {
    if output_1.len() != output_2.len() {
        return Err(anyhow::anyhow!("Output vectors are not the same length"));
    }
    let mut diff_vector: Vec<i32> = Vec::with_capacity(output_1.len());
    let mut total_error: i64 = 0;

    for i in 0..output_1.len() {
        diff_vector.push((output_1[i] - output_2[i]).abs());
        if output_1[i] != output_2[i] {
            total_error += 1;
        }
    }

    // generally how far off the two vectors are from each other
    let mean = diff_vector.iter().sum::<i32>() as f64 / diff_vector.len() as f64;

    // how much the errors are away from their mean
    let variance = diff_vector.iter().map(|x| x.pow(2)).sum::<i32>() as i64
        / diff_vector.len() as i64
        - mean.powi(2) as i64;

    let accuracy = (diff_vector.iter().sum::<i32>()) as i64;

    Ok(MandelBrotError {
        error_vector: diff_vector,
        divergent_pixels: total_error as i64,
        mean,
        variance,
        accuracy,
    })
}

pub fn save_results(infos: MandelBrotSimulationInfo, path: &str) -> Result<(), anyhow::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file in append mode");
    
    if file.metadata().unwrap().len() == 0 {
        file.write_all(b"simulation_name,method,execution_time,resolution,iterations\n")
            .expect("Failed to write to file");
    }

    file.write_all(format!(
        "{},{},{},{},{}\n",
        infos.simulation_name, infos.method, infos.execution_time, infos.resolution, infos.iterations
    ).as_bytes())
    .expect("Failed to write to file");

    file.flush().expect("Failed to flush buffer");
    Ok(())
}
