/// Returns true if there are no elements in `x` in `arr`
/// such that `x <= 0.0` or `x` is `f64::NAN` and `sum(arr) > 0.0`.
/// IF `incl_zero` is true, it tests for `x < 0.0` instead of `x <= 0.0`
pub fn is_valid_multinomial(arr: &[f64], incl_zero: bool) -> bool {
    let mut sum = 0.0;
    for i in 0..arr.len() {
        let el = *unsafe { arr.get_unchecked(i) };
        if incl_zero && el < 0.0 {
            return false;
        } else if !incl_zero && el <= 0.0 {
            return false;
        } else if el.is_nan() {
            return false;
        }
        sum += el;
    }
    sum != 0.0
}

#[cfg(test)]
pub mod test {
    use super::is_valid_multinomial;
    use crate::distribution::{Continuous, Discrete, Univariate};
    use std::f64;

    /// cdf should be the integral of the pdf
    fn check_integrate_pdf_is_cdf<D: Univariate<f64, f64> + Continuous<f64, f64>>(
        dist: &D,
        x_min: f64,
        x_max: f64,
        step: f64,
    ) {
        let mut prev_x = x_min;
        let mut prev_density = dist.pdf(x_min);
        let mut sum = 0.0;

        loop {
            let x = prev_x + step;
            let density = dist.pdf(x);

            assert!(density >= 0.0);

            let ln_density = dist.ln_pdf(x);

            assert_almost_eq!(density.ln(), ln_density, 1e-10);

            // triangle rule
            sum += (prev_density + density) * step / 2.0;

            let cdf = dist.cdf(x);
            if (sum - cdf).abs() > 1e-3 {
                println!("Integral of pdf doesn't equal cdf!");
                println!("Integration from {} by {} to {} = {}", x_min, step, x, sum);
                println!("cdf = {}", cdf);
                assert!(false);
            }

            if x >= x_max {
                break;
            } else {
                prev_x = x;
                prev_density = density;
            }
        }

        assert!(sum > 0.99);
        assert!(sum <= 1.001);
    }

    /// cdf should be the sum of the pmf
    fn check_sum_pmf_is_cdf<D: Univariate<u64, f64> + Discrete<u64, f64>>(dist: &D, x_max: u64) {
        let mut sum = 0.0;

        // go slightly beyond x_max to test for off-by-one errors
        for i in 0..x_max + 3 {
            let prob = dist.pmf(i);

            assert!(prob >= 0.0);
            assert!(prob <= 1.0);

            sum += prob;

            if i == x_max {
                assert!(sum > 0.99);
            }

            assert_almost_eq!(sum, dist.cdf(i as f64), 1e-10);
            assert_almost_eq!(sum, dist.cdf(i as f64 + 0.1), 1e-10);
            assert_almost_eq!(sum, dist.cdf(i as f64 + 0.5), 1e-10);
            assert_almost_eq!(sum, dist.cdf(i as f64 + 0.9), 1e-10);
        }

        assert!(sum > 0.99);
        assert!(sum <= 1.0 + 1e-10);
    }

    /// Does a series of checks that all continuous distributions must obey.
    /// 99% of the probability mass should be between x_min and x_max.
    pub fn check_continuous_distribution<D: Univariate<f64, f64> + Continuous<f64, f64>>(
        dist: &D,
        x_min: f64,
        x_max: f64,
    ) {
        assert_eq!(dist.pdf(f64::NEG_INFINITY), 0.0);
        assert_eq!(dist.pdf(f64::INFINITY), 0.0);
        assert_eq!(dist.ln_pdf(f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert_eq!(dist.ln_pdf(f64::INFINITY), f64::NEG_INFINITY);
        assert_eq!(dist.cdf(f64::NEG_INFINITY), 0.0);
        assert_eq!(dist.cdf(f64::INFINITY), 1.0);

        check_integrate_pdf_is_cdf(dist, x_min, x_max, (x_max - x_min) / 100000.0);
    }

    /// Does a series of checks that all positive discrete distributions must
    /// obey.
    /// 99% of the probability mass should be between 0 and x_max (inclusive).
    pub fn check_discrete_distribution<D: Univariate<u64, f64> + Discrete<u64, f64>>(
        dist: &D,
        x_max: u64,
    ) {
        assert_eq!(dist.cdf(f64::NEG_INFINITY), 0.0);
        assert_eq!(dist.cdf(-10.0), 0.0);
        assert_eq!(dist.cdf(-1.0), 0.0);
        assert_eq!(dist.cdf(-0.01), 0.0);
        assert_eq!(dist.cdf(f64::INFINITY), 1.0);

        check_sum_pmf_is_cdf(dist, x_max);
    }

    #[test]
    fn test_is_valid_multinomial() {
        use std::f64;

        let invalid = [1.0, f64::NAN, 3.0];
        assert!(!is_valid_multinomial(&invalid, true));
        let invalid2 = [-2.0, 5.0, 1.0, 6.2];
        assert!(!is_valid_multinomial(&invalid2, true));
        let invalid3 = [0.0, 0.0, 0.0];
        assert!(!is_valid_multinomial(&invalid3, true));
        let valid = [5.2, 0.0, 1e-15, 1000000.12];
        assert!(is_valid_multinomial(&valid, true));
    }

    #[test]
    fn test_is_valid_multinomial_no_zero() {
        let invalid = [5.2, 0.0, 1e-15, 1000000.12];
        assert!(!is_valid_multinomial(&invalid, false));
    }
}
