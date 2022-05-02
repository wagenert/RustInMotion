pub fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if series.len() >= n {
        let result_size = series.len() - n + 1;
        let mut sma: Vec<f64> = Vec::with_capacity(result_size);
        for i in 0..result_size {
            let sum: f64 = series[i..(i + n)].iter().sum();
            sma.push(sum / n as f64);
        }
        return Some(sma);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sma_none() {
        let s: [f64; 3] = [1.0, 2.0, 3.0];
        assert_eq!(n_window_sma(4, &s), None);
    }

    #[test]
    fn test_sma_some_one_value() {
        let s: [f64; 3] = [1.0, 2.0, 3.0];
        if let Some(mut result) = n_window_sma(3, &s) {
            assert_eq!(result.len(), 1);
            assert_eq!(result.pop(), Some(2.0));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_sma_some_multiple_values() {
        let s: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
        if let Some(mut result) = n_window_sma(3, &s) {
            assert_eq!(result.len(), 2);
            assert_eq!(result.pop(), Some(3.0));
            assert_eq!(result.pop(), Some(2.0));
        } else {
            assert!(false);
        }
    }

}
