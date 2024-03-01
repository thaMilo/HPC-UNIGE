use anyhow::Result;

pub struct MandelBrotError {
    pub error_vector: Vec<i32>,
    pub total_error: f64,
}

pub fn compute_error(
    output_1: &Vec<i32>,
    output_2: &Vec<i32>,
) -> Result<MandelBrotError, anyhow::Error> {
    if output_1.len() != output_2.len() {
        return Err(anyhow::anyhow!("Output vectors are not the same length"));
    }
    let mut diff_vector: Vec<i32> = Vec::with_capacity(output_1.len());
    for i in 0..output_1.len() {
        diff_vector.push( (output_1[i] - output_2[i]).abs() );
    }
    let mut total_error: f64 = (diff_vector.iter().sum::<i32>()) as f64;
    total_error = total_error / output_1.len() as f64;
    Ok(MandelBrotError {
        error_vector: diff_vector,
        total_error: total_error as f64,
    })
}
