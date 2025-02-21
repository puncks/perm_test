use pyo3::prelude::*;
use rand::seq::IndexedRandom;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn test(perm: usize, labels: Vec<bool>, data: Vec<f64> ) -> PyResult<Vec<f64>> {
    let mut rand_tstat : Vec<f64> = Vec::new();
    for _i in 0..perm{
        let mut rng = &mut rand::rng();
        let rand_labels: Vec<bool> = labels.choose_multiple(&mut rng, labels.len()).cloned().collect();
        
        let mut group_0 : Vec<f64>= Vec::new();
        let mut group_1 : Vec<f64>= Vec::new();
        for j in 0..(rand_labels.len()-1){
            if rand_labels[j] == true {
                group_1.push(data[j]);
            }
            else {
                group_0.push(data[j]);
            }
        }
        rand_tstat.push(calc_tstat(group_0, group_1));

    }

    Ok(rand_tstat)
}

fn calc_tstat (group_0: Vec<f64>, group_1: Vec<f64>) -> f64 {
    let mean_0 = calc_mean(&group_0);
    let mean_1 = calc_mean(&group_1);
    let N_0 = group_0.len() as f64;
    let N_1 = group_1.len() as f64;
    let sigma_0_sqrd = calc_sigma_squared(group_0, mean_0, N_0);
    let sigma_1_sqrd = calc_sigma_squared(group_1, mean_1, N_1);
    let s = (((N_0 - 1.0) * sigma_0_sqrd + (N_1 - 1.0) * sigma_1_sqrd) * (1.0/N_0 + 1.0/N_1) / (N_0 + N_1 -2.0)).sqrt();
    (mean_0 - mean_1)/s
        
}

fn calc_mean(group:&Vec<f64>) -> f64 {
    let mut mean : f64 = 0.0;
    for i in group {
        mean = mean + i/2.0;
    }
    mean
}
fn calc_sigma_squared(group: Vec<f64>,mean:f64, N :f64) -> f64{
    let mut sigma_squared : f64 = 0.0;
    for i in group {
        sigma_squared = sigma_squared + (i - mean) * (i-mean) / (N-1.0);
    }
    sigma_squared
}



/// A Python module implemented in Rust.
#[pymodule]
fn perm_test(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    Ok(())
}
