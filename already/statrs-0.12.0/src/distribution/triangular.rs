use crate::distribution::{Continuous, Univariate};
use rand::distributions::Distribution;
use rand::Rng;
use crate::statistics::*;
use std::f64;
use crate::{Result, StatsError};

/// Implements the
/// [Triangular](https://en.wikipedia.org/wiki/Triangular_distribution)
/// distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::{Triangular, Continuous};
/// use statrs::statistics::Mean;
///
/// let n = Triangular::new(0.0, 5.0, 2.5).unwrap();
/// assert_eq!(n.mean(), 7.5 / 3.0);
/// assert_eq!(n.pdf(2.5), 5.0 / 12.5);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangular {
    min: f64,
    max: f64,
    mode: f64,
}

impl Triangular {
    /// Constructs a new triangular distribution with a minimum of `min`,
    /// maximum of `max`, and a mode of `mode`.
    ///
    /// # Errors
    ///
    /// Returns an error if `min`, `max`, or `mode` are `NaN` or `±INF`.
    /// Returns an error if `max < mode`, `mode < min`, or `max == min`.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Triangular;
    ///
    /// let mut result = Triangular::new(0.0, 5.0, 2.5);
    /// assert!(result.is_ok());
    ///
    /// result = Triangular::new(2.5, 1.5, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn new(min: f64, max: f64, mode: f64) -> Result<Triangular> {
        if min.is_infinite() || max.is_infinite() || mode.is_infinite() {
            return Err(StatsError::BadParams);
        }
        if min.is_nan() || max.is_nan() || mode.is_nan() {
            return Err(StatsError::BadParams);
        }
        if max < mode || mode < min {
            return Err(StatsError::BadParams);
        }
        if max == min {
            return Err(StatsError::BadParams);
        }
        Ok(Triangular {
            min: min,
            max: max,
            mode: mode,
        })
    }
}

impl Distribution<f64> for Triangular {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> f64 {
        sample_unchecked(r, self.min, self.max, self.mode)
    }
}

impl Univariate<f64, f64> for Triangular {
    /// Calculates the cumulative distribution function for the triangular
    /// distribution
    /// at `x`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// if x == min {
    ///     0
    /// } if min < x <= mode {
    ///     (x - min)^2 / ((max - min) * (mode - min))
    /// } else if mode < x < max {
    ///     1 - (max - min)^2 / ((max - min) * (max - mode))
    /// } else {
    ///     1
    /// }
    /// ```
    fn cdf(&self, x: f64) -> f64 {
        let a = self.min;
        let b = self.max;
        let c = self.mode;
        if x <= a {
            0.0
        } else if x <= c {
            (x - a) * (x - a) / ((b - a) * (c - a))
        } else if x < b {
            1.0 - (b - x) * (b - x) / ((b - a) * (b - c))
        } else {
            1.0
        }
    }
}

impl Min<f64> for Triangular {
    /// Returns the minimum value in the domain of the
    /// triangular distribution representable by a double precision float
    ///
    /// # Remarks
    ///
    /// The return value is the same min used to construct the distribution
    fn min(&self) -> f64 {
        self.min
    }
}

impl Max<f64> for Triangular {
    /// Returns the maximum value in the domain of the
    /// triangular distribution representable by a double precision float
    ///
    /// # Remarks
    ///
    /// The return value is the same max used to construct the distribution
    fn max(&self) -> f64 {
        self.max
    }
}

impl Mean<f64> for Triangular {
    /// Returns the mean of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (min + max + mode) / 3
    /// ```
    fn mean(&self) -> f64 {
        (self.min + self.max + self.mode) / 3.0
    }
}

impl Variance<f64> for Triangular {
    /// Returns the variance of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (min^2 + max^2 + mode^2 - min * max - min * mode - max * mode) / 18
    /// ```
    fn variance(&self) -> f64 {
        let a = self.min;
        let b = self.max;
        let c = self.mode;
        (a * a + b * b + c * c - a * b - a * c - b * c) / 18.0
    }

    /// Returns the standard deviation of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt((min^2 + max^2 + mode^2 - min * max - min * mode - max * mode) /
    /// 18)
    /// ```
    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Entropy<f64> for Triangular {
    /// Returns the entropy of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 1 / 2 + ln((max - min) / 2)
    /// ```
    fn entropy(&self) -> f64 {
        0.5 + ((self.max - self.min) / 2.0).ln()
    }
}

impl Skewness<f64> for Triangular {
    /// Returns the skewness of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (sqrt(2) * (min + max - 2 * mode) * (2 * min - max - mode) * (min - 2 *
    /// max + mode)) /
    /// ( 5 * (min^2 + max^2 + mode^2 - min * max - min * mode - max * mode)^(3
    /// / 2))
    /// ```
    fn skewness(&self) -> f64 {
        let a = self.min;
        let b = self.max;
        let c = self.mode;
        let q = f64::consts::SQRT_2 * (a + b - 2.0 * c) * (2.0 * a - b - c) * (a - 2.0 * b + c);
        let d = 5.0 * (a * a + b * b + c * c - a * b - a * c - b * c).powf(3.0 / 2.0);
        q / d
    }
}

impl Median<f64> for Triangular {
    /// Returns the median of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// if mode >= (min + max) / 2 {
    ///     min + sqrt((max - min) * (mode - min) / 2)
    /// } else {
    ///     max - sqrt((max - min) * (max - mode) / 2)
    /// }
    /// ```
    fn median(&self) -> f64 {
        let a = self.min;
        let b = self.max;
        let c = self.mode;
        if c >= (a + b) / 2.0 {
            a + ((b - a) * (c - a) / 2.0).sqrt()
        } else {
            b - ((b - a) * (b - c) / 2.0).sqrt()
        }
    }
}

impl Mode<f64> for Triangular {
    /// Returns the mode of the triangular distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// mode
    /// ```
    fn mode(&self) -> f64 {
        self.mode
    }
}

impl Continuous<f64, f64> for Triangular {
    /// Calculates the probability density function for the triangular
    /// distribution
    /// at `x`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// if x < min {
    ///     0
    /// } else if min <= x <= mode {
    ///     2 * (x - min) / ((max - min) * (mode - min))
    /// } else if mode < x <= max {
    ///     2 * (max - x) / ((max - min) * (max - mode))
    /// } else {
    ///     0
    /// }
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        let a = self.min;
        let b = self.max;
        let c = self.mode;
        if a <= x && x <= c {
            2.0 * (x - a) / ((b - a) * (c - a))
        } else if c < x && x <= b {
            2.0 * (b - x) / ((b - a) * (b - c))
        } else {
            0.0
        }
    }

    /// Calculates the log probability density function for the triangular
    /// distribution
    /// at `x`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln( if x < min {
    ///     0
    /// } else if min <= x <= mode {
    ///     2 * (x - min) / ((max - min) * (mode - min))
    /// } else if mode < x <= max {
    ///     2 * (max - x) / ((max - min) * (max - mode))
    /// } else {
    ///     0
    /// } )
    /// ```
    fn ln_pdf(&self, x: f64) -> f64 {
        self.pdf(x).ln()
    }
}

fn sample_unchecked<R: Rng + ?Sized>(r: &mut R, min: f64, max: f64, mode: f64) -> f64 {
    let f: f64 = r.gen();
    if f < (mode - min) / (max - min) {
        min + (f * (max - min) * (mode - min)).sqrt()
    } else {
        max - ((1.0 - f) * (max - min) * (max - mode)).sqrt()
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use std::f64;
    use crate::statistics::*;
    use crate::distribution::{Univariate, Continuous, Triangular};
    use crate::distribution::internal::*;

    fn try_create(min: f64, max: f64, mode: f64) -> Triangular {
        let n = Triangular::new(min, max, mode);
        assert!(n.is_ok());
        n.unwrap()
    }

    fn create_case(min: f64, max: f64, mode: f64) {
        let n = try_create(min, max, mode);
        assert_eq!(n.min(), min);
        assert_eq!(n.max(), max);
        assert_eq!(n.mode(), mode);
    }

    fn bad_create_case(min: f64, max: f64, mode: f64) {
        let n = Triangular::new(min, max, mode);
        assert!(n.is_err());
    }

    fn get_value<T, F>(min: f64, max: f64, mode: f64, eval: F) -> T
        where T: PartialEq + Debug,
              F: Fn(Triangular) -> T
    {
        let n = try_create(min, max, mode);
        eval(n)
    }

    fn test_case<F>(min: f64, max: f64, mode: f64, expected: f64, eval: F)
        where F: Fn(Triangular) -> f64
    {
        let x = get_value(min, max, mode, eval);
        assert_eq!(expected, x);
    }

    fn test_almost<F>(min: f64, max: f64, mode: f64, expected: f64, acc: f64, eval: F)
        where F: Fn(Triangular) -> f64
    {
        let x = get_value(min, max, mode, eval);
        assert_almost_eq!(expected, x, acc);
    }

    #[test]
    fn test_create() {
        create_case(-1.0, 1.0, 0.0);
        create_case(1.0, 2.0, 1.0);
        create_case(5.0, 25.0, 25.0);
        create_case(1.0e-5, 1.0e5, 1.0e-3);
        create_case(0.0, 1.0, 0.9);
        create_case(-4.0, -0.5, -2.0);
        create_case(-13.039, 8.42, 1.17);
    }

    #[test]
    fn test_bad_create() {
        bad_create_case(0.0, 0.0, 0.0);
        bad_create_case(0.0, 1.0, -0.1);
        bad_create_case(0.0, 1.0, 1.1);
        bad_create_case(0.0, -1.0, 0.5);
        bad_create_case(2.0, 1.0, 1.5);
        bad_create_case(f64::NAN, 1.0, 0.5);
        bad_create_case(0.2, f64::NAN, 0.5);
        bad_create_case(0.5, 1.0, f64::NAN);
        bad_create_case(f64::NAN, f64::NAN, f64::NAN);
        bad_create_case(f64::NEG_INFINITY, 1.0, 0.5);
        bad_create_case(0.0, f64::INFINITY, 0.5);
    }

    #[test]
    fn test_variance() {
        test_case(0.0, 1.0, 0.5, 0.75 / 18.0, |x| x.variance());
        test_case(0.0, 1.0, 0.75, 0.8125 / 18.0, |x| x.variance());
        test_case(-5.0, 8.0, -3.5, 151.75 / 18.0, |x| x.variance());
        test_case(-5.0, 8.0, 5.0, 139.0 / 18.0, |x| x.variance());
        test_case(-5.0, -3.0, -4.0, 3.0 / 18.0, |x| x.variance());
        test_case(15.0, 134.0, 21.0, 13483.0 / 18.0, |x| x.variance());
    }

    #[test]
    fn test_std_dev() {
        test_case(0.0, 1.0, 0.5, (0.75f64 / 18.0).sqrt(), |x| x.std_dev());
        test_case(0.0, 1.0, 0.75, (0.8125f64 / 18.0).sqrt(), |x| x.std_dev());
        test_case(-5.0, 8.0, -3.5, (151.75f64 / 18.0).sqrt(), |x| x.std_dev());
        test_case(-5.0, 8.0, 5.0, (139.0f64 / 18.0).sqrt(), |x| x.std_dev());
        test_case(-5.0, -3.0, -4.0, (3.0f64 / 18.0).sqrt(), |x| x.std_dev());
        test_case(15.0, 134.0, 21.0, (13483.0f64 / 18.0).sqrt(), |x| x.std_dev());
    }

    #[test]
    fn test_entropy() {
        test_almost(0.0, 1.0, 0.5, -0.1931471805599453094172, 1e-16, |x| x.entropy());
        test_almost(0.0, 1.0, 0.75, -0.1931471805599453094172, 1e-16, |x| x.entropy());
        test_case(-5.0, 8.0, -3.5, 2.371802176901591426636, |x| x.entropy());
        test_case(-5.0, 8.0, 5.0, 2.371802176901591426636, |x| x.entropy());
        test_case(-5.0, -3.0, -4.0, 0.5, |x| x.entropy());
        test_case(15.0, 134.0, 21.0, 4.585976312551584075938, |x| x.entropy());
    }

    #[test]
    fn test_skewness() {
        test_case(0.0, 1.0, 0.5, 0.0, |x| x.skewness());
        test_case(0.0, 1.0, 0.75, -0.4224039833745502226059, |x| x.skewness());
        test_case(-5.0, 8.0, -3.5, 0.5375093589712976359809, |x| x.skewness());
        test_case(-5.0, 8.0, 5.0, -0.4445991743012595633537, |x| x.skewness());
        test_case(-5.0, -3.0, -4.0, 0.0, |x| x.skewness());
        test_case(15.0, 134.0, 21.0, 0.5605920922751860613217, |x| x.skewness());
    }

    #[test]
    fn test_mode() {
        test_case(0.0, 1.0, 0.5, 0.5, |x| x.mode());
        test_case(0.0, 1.0, 0.75, 0.75, |x| x.mode());
        test_case(-5.0, 8.0, -3.5, -3.5, |x| x.mode());
        test_case(-5.0, 8.0, 5.0, 5.0, |x| x.mode());
        test_case(-5.0, -3.0, -4.0, -4.0, |x| x.mode());
        test_case(15.0, 134.0, 21.0, 21.0, |x| x.mode());
    }

    #[test]
    fn test_median() {
        test_case(0.0, 1.0, 0.5, 0.5, |x| x.median());
        test_case(0.0, 1.0, 0.75, 0.6123724356957945245493, |x| x.median());
        test_almost(-5.0, 8.0, -3.5, -0.6458082328952913226724, 1e-15, |x| x.median());
        test_almost(-5.0, 8.0, 5.0, 3.062257748298549652367, 1e-15, |x| x.median());
        test_case(-5.0, -3.0, -4.0, -4.0, |x| x.median());
        test_almost(15.0, 134.0, 21.0, 52.00304883716712238797, 1e-14, |x| x.median());
    }

    #[test]
    fn test_pdf() {
        test_case(0.0, 1.0, 0.5, 0.0, |x| x.pdf(-1.0));
        test_case(0.0, 1.0, 0.5, 0.0, |x| x.pdf(1.1));
        test_case(0.0, 1.0, 0.5, 1.0, |x| x.pdf(0.25));
        test_case(0.0, 1.0, 0.5, 2.0, |x| x.pdf(0.5));
        test_case(0.0, 1.0, 0.5, 1.0, |x| x.pdf(0.75));
        test_case(-5.0, 8.0, -3.5, 0.0, |x| x.pdf(-5.1));
        test_case(-5.0, 8.0, -3.5, 0.0, |x| x.pdf(8.1));
        test_case(-5.0, 8.0, -3.5, 0.1025641025641025641026, |x| x.pdf(-4.0));
        test_case(-5.0, 8.0, -3.5, 0.1538461538461538461538, |x| x.pdf(-3.5));
        test_case(-5.0, 8.0, -3.5, 0.05351170568561872909699, |x| x.pdf(4.0));
        test_case(-5.0, -3.0, -4.0, 0.0, |x| x.pdf(-5.1));
        test_case(-5.0, -3.0, -4.0, 0.0, |x| x.pdf(-2.9));
        test_case(-5.0, -3.0, -4.0, 0.5, |x| x.pdf(-4.5));
        test_case(-5.0, -3.0, -4.0, 1.0, |x| x.pdf(-4.0));
        test_case(-5.0, -3.0, -4.0, 0.5, |x| x.pdf(-3.5));
    }

    #[test]
    fn test_ln_pdf() {
        test_case(0.0, 1.0, 0.5, f64::NEG_INFINITY, |x| x.ln_pdf(-1.0));
        test_case(0.0, 1.0, 0.5, f64::NEG_INFINITY, |x| x.ln_pdf(1.1));
        test_case(0.0, 1.0, 0.5, 0.0, |x| x.ln_pdf(0.25));
        test_case(0.0, 1.0, 0.5, 2f64.ln(), |x| x.ln_pdf(0.5));
        test_case(0.0, 1.0, 0.5, 0.0, |x| x.ln_pdf(0.75));
        test_case(-5.0, 8.0, -3.5, f64::NEG_INFINITY, |x| x.ln_pdf(-5.1));
        test_case(-5.0, 8.0, -3.5, f64::NEG_INFINITY, |x| x.ln_pdf(8.1));
        test_case(-5.0, 8.0, -3.5, 0.1025641025641025641026f64.ln(), |x| x.ln_pdf(-4.0));
        test_case(-5.0, 8.0, -3.5, 0.1538461538461538461538f64.ln(), |x| x.ln_pdf(-3.5));
        test_case(-5.0, 8.0, -3.5, 0.05351170568561872909699f64.ln(), |x| x.ln_pdf(4.0));
        test_case(-5.0, -3.0, -4.0, f64::NEG_INFINITY, |x| x.ln_pdf(-5.1));
        test_case(-5.0, -3.0, -4.0, f64::NEG_INFINITY, |x| x.ln_pdf(-2.9));
        test_case(-5.0, -3.0, -4.0, 0.5f64.ln(), |x| x.ln_pdf(-4.5));
        test_case(-5.0, -3.0, -4.0, 0.0, |x| x.ln_pdf(-4.0));
        test_case(-5.0, -3.0, -4.0, 0.5f64.ln(), |x| x.ln_pdf(-3.5));
    }

    #[test]
    fn test_cdf() {
        test_case(0.0, 1.0, 0.5, 0.125, |x| x.cdf(0.25));
        test_case(0.0, 1.0, 0.5, 0.5, |x| x.cdf(0.5));
        test_case(0.0, 1.0, 0.5, 0.875, |x| x.cdf(0.75));
        test_case(-5.0, 8.0, -3.5, 0.05128205128205128205128, |x| x.cdf(-4.0));
        test_case(-5.0, 8.0, -3.5, 0.1153846153846153846154, |x| x.cdf(-3.5));
        test_case(-5.0, 8.0, -3.5, 0.892976588628762541806, |x| x.cdf(4.0));
        test_case(-5.0, -3.0, -4.0, 0.125, |x| x.cdf(-4.5));
        test_case(-5.0, -3.0, -4.0, 0.5, |x| x.cdf(-4.0));
        test_case(-5.0, -3.0, -4.0, 0.875, |x| x.cdf(-3.5));
    }

    #[test]
    fn test_cdf_lower_bound() {
        test_case(0.0, 3.0, 1.5, 0.0, |x| x.cdf(-1.0));
    }

    #[test]
    fn test_cdf_upper_bound() {
        test_case(0.0, 3.0, 1.5, 1.0, |x| x.cdf(5.0));
    }

    #[test]
    fn test_continuous() {
        test::check_continuous_distribution(&try_create(-5.0, 5.0, 0.0), -5.0, 5.0);
        test::check_continuous_distribution(&try_create(-15.0, -2.0, -3.0), -15.0, -2.0);
    }
}
