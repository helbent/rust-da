
pub fn mean(numbers: &[f64]) -> f64 {

    numbers.iter().sum::<f64>() / numbers.len() as f64

}

pub fn std_dev(numbers: &[f64]) -> f64 {

    let num_mean = mean(numbers);

    let variance = numbers.iter().map(|x| {
                   let diff = num_mean - *x as f64;
                   diff * diff }).sum::<f64>() / numbers.len() as f64;

    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        // should use assert_f64_near?
        assert_eq!(mean(&[1.0_f64, 2.0_f64]), 1.5_f64)
    }

    #[test]
    fn test_std_dev() {
        assert_f64_near!(std_dev(&[1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64]), 1.11803398874989484_f64)
    }
}

