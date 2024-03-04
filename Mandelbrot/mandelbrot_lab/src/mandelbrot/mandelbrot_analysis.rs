use anyhow::{Ok, Result};
use polars::prelude::*;
use std::fs::File;

pub struct MandelBrotError {
    pub error_vector: Vec<i32>,
    pub divergent_pixels: i64,
    pub mean: f64,
    pub variance: i64,
    pub accuracy: i64,
}

pub struct MandelBrotSimulationInfo {
    pub methods: Vec<String>,
    pub execution_times: Vec<i64>,
    pub resolutions: Vec<i32>,
    pub iterations: Vec<i32>,
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
    let method_serie = Series::new("method", &infos.methods);
    let execution_time_serie = Series::new("execution_time", &infos.execution_times);
    let resolution_serie = Series::new("resolution", &infos.resolutions);
    let iterations_serie = Series::new("iterations", &infos.iterations);

    let mut df = DataFrame::new(vec![
        method_serie,
        execution_time_serie,
        resolution_serie,
        iterations_serie,
    ])?;

    let mut writer = CsvWriter::new(File::create(path)?);
    writer.finish(&mut df)?;

    Ok(())
}
