use crate::prelude::*;

pub fn max(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        let max = series
            .iter()
            .fold(-INFINITY, |acc, x| if *x > acc { *x } else { acc });
        return Some(max);
    }
    None
}

pub fn min(series: &[f64]) -> Option<f64> {
    if !series.is_empty() {
        let min = series
            .iter()
            .fold(INFINITY, |acc, x| if *x < acc { *x } else { acc });
        return Some(min);
    }
    None
}

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

pub fn price_difference(series: &[f64]) -> Option<(f64, f64)> {
    if series.len() > 1 {
        let start = series[0];
        if start > 0.0 {
            let end = series.last().unwrap();
            let percentage = end * 100.0 / start;
            let diff = end - start;
            return Some((percentage, diff));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_none() {
        let s: [f64; 0] = [];
        assert_eq!(max(&s), None);
    }

    #[test]
    fn test_max_some() {
        let s: [f64; 3] = [0.0, 0.1, 10.0];
        assert_eq!(max(&s), Some(10.0));
    }

    #[test]
    fn test_min_none() {
        let s: [f64; 0] = [];
        assert_eq!(min(&s), None);
    }

    #[test]
    fn test_min_some() {
        let s: [f64; 3] = [0.0, 0.1, -10.0];
        assert_eq!(min(&s), Some(-10.0));
    }

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

    #[test]
    fn test_sma_none_too_large_window() {
        let s: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(n_window_sma(5, &s), None);
    }

    #[test]
    fn test_price_difference_none() {
        let s: [f64; 0] = [];
        assert_eq!(price_difference(&s), None);
        let s: [f64; 1] = [124.3];
        assert_eq!(price_difference(&s), None);
    }

    #[test]
    fn test_price_difference_some() {
        let s: [f64; 3] = [1.0, 3.0, 2.0];
        if let Some((perc, diff)) = price_difference(&s) {
            assert_eq!(perc, 200.0);
            assert_eq!(diff, 1.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_price_difference_zero() {
        let s: [f64; 3] = [0.0, 3.0, 2.0];
        assert_eq!(price_difference(&s), None);
    }
}
