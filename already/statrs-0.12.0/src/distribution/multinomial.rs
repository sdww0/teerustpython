use crate::distribution::{CheckedDiscrete, Discrete};
use crate::function::factorial;
use rand::distributions::Distribution;
use rand::Rng;
use crate::statistics::*;
use crate::{Result, StatsError};
use std::vec::Vec;
use std::vec;
/// Implements the
/// [Multinomial](https://en.wikipedia.org/wiki/Multinomial_distribution)
/// distribution which is a generalization of the
/// [Binomial](https://en.wikipedia.org/wiki/Binomial_distribution)
/// distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::Multinomial;
/// use statrs::statistics::Mean;
///
/// let n = Multinomial::new(&[0.3, 0.7], 5).unwrap();
/// assert_eq!(n.mean(), [1.5, 3.5]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Multinomial {
    p: Vec<f64>,
    n: u64,
}

impl Multinomial {
    /// Constructs a new multinomial distribution with probabilities `p`
    /// and `n` number of trials.
    ///
    /// # Errors
    ///
    /// Returns an error if `p` is empty, the sum of the elements
    /// in `p` is 0, or any element in `p` is less than 0 or is `f64::NAN`
    ///
    /// # Note
    ///
    /// The elements in `p` do not need to be normalized
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Multinomial;
    ///
    /// let mut result = Multinomial::new(&[0.0, 1.0, 2.0], 3);
    /// assert!(result.is_ok());
    ///
    /// result = Multinomial::new(&[0.0, -1.0, 2.0], 3);
    /// assert!(result.is_err());
    /// ```
    pub fn new(p: &[f64], n: u64) -> Result<Multinomial> {
        if !super::internal::is_valid_multinomial(p, true) {
            Err(StatsError::BadParams)
        } else {
            Ok(Multinomial {
                p: p.to_vec(),
                n: n,
            })
        }
    }

    /// Returns the probabilities of the multinomial
    /// distribution as a slice
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Multinomial;
    ///
    /// let n = Multinomial::new(&[0.0, 1.0, 2.0], 3).unwrap();
    /// assert_eq!(n.p(), [0.0, 1.0, 2.0]);
    /// ```
    pub fn p(&self) -> &[f64] {
        &self.p
    }

    /// Returns the number of trials of the multinomial
    /// distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Multinomial;
    ///
    /// let n = Multinomial::new(&[0.0, 1.0, 2.0], 3).unwrap();
    /// assert_eq!(n.n(), 3);
    /// ```
    pub fn n(&self) -> u64 {
        self.n
    }
}

impl Distribution<Vec<f64>> for Multinomial {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> Vec<f64> {
        let p_cdf = super::categorical::prob_mass_to_cdf(self.p());
        let mut res = vec![0.0; self.p.len()];
        for _ in 0..self.n {
            let i = super::categorical::sample_unchecked(r, &p_cdf);
            let el = unsafe { res.get_unchecked_mut(i as usize) };
            *el = *el + 1.0;
        }
        res
    }
}

impl Mean<Vec<f64>> for Multinomial {
    /// Returns the mean of the multinomial distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// n * p_i for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// and `k` is the total number of probabilities
    fn mean(&self) -> Vec<f64> {
        self.p.iter().map(|x| x * self.n as f64).collect()
    }
}

impl Variance<Vec<f64>> for Multinomial {
    /// Returns the variance of the multinomial distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// n * p_i * (1 - p_1) for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// and `k` is the total number of probabilities
    fn variance(&self) -> Vec<f64> {
        self.p
            .iter()
            .map(|x| x * self.n as f64 * (1.0 - x))
            .collect()
    }

    /// Returns the standard deviation of the multinomial distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt(n * p_i * (1 - p_1)) for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// and `k` is the total number of probabilities
    fn std_dev(&self) -> Vec<f64> {
        self.variance().iter().map(|x| x.sqrt()).collect()
    }
}

impl Skewness<Vec<f64>> for Multinomial {
    /// Returns the skewness of the multinomial distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (1 - 2 * p_i) / (n * p_i * (1 - p_i)) for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// and `k` is the total number of probabilities
    fn skewness(&self) -> Vec<f64> {
        self.p
            .iter()
            .map(|x| (1.0 - 2.0 * x) / (self.n as f64 * (1.0 - x) * x).sqrt())
            .collect()
    }
}

impl<'a> Discrete<&'a [u64], f64> for Multinomial {
    /// Calculates the probability mass function for the multinomial
    /// distribution
    /// with the given `x`'s corresponding to the probabilities for this
    /// distribution
    ///
    /// # Panics
    ///
    /// If the elements in `x` do not sum to `n` or if the length of `x` is not
    /// equivalent to the length of `p`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (n! / x_1!...x_k!) * p_i^x_i for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// `x_i` is the `i`th `x` value, and `k` is the total number of
    /// probabilities
    fn pmf(&self, x: &[u64]) -> f64 {
        self.checked_pmf(x).unwrap()
    }

    /// Calculates the log probability mass function for the multinomial
    /// distribution
    /// with the given `x`'s corresponding to the probabilities for this
    /// distribution
    ///
    /// # Panics
    ///
    /// If the elements in `x` do not sum to `n` or if the length of `x` is not
    /// equivalent to the length of `p`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln((n! / x_1!...x_k!) * p_i^x_i) for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// `x_i` is the `i`th `x` value, and `k` is the total number of
    /// probabilities
    fn ln_pmf(&self, x: &[u64]) -> f64 {
        self.checked_ln_pmf(x).unwrap()
    }
}

impl<'a> CheckedDiscrete<&'a [u64], f64> for Multinomial {
    /// Calculates the probability mass function for the multinomial
    /// distribution
    /// with the given `x`'s corresponding to the probabilities for this
    /// distribution
    ///
    /// # Errors
    ///
    /// If the elements in `x` do not sum to `n` or if the length of `x` is not
    /// equivalent to the length of `p`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (n! / x_1!...x_k!) * p_i^x_i for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// `x_i` is the `i`th `x` value, and `k` is the total number of
    /// probabilities
    fn checked_pmf(&self, x: &[u64]) -> Result<f64> {
        if self.p.len() != x.len() {
            return Err(StatsError::ContainersMustBeSameLength);
        }
        if x.iter().sum::<u64>() != self.n {
            return Err(StatsError::ContainerExpectedSumVar("x", "n"));
        }
        let coeff = factorial::multinomial(self.n, x);
        let val = coeff
            * self
                .p
                .iter()
                .zip(x.iter())
                .fold(1.0, |acc, (pi, xi)| acc * pi.powf(*xi as f64));
        Ok(val)
    }

    /// Calculates the log probability mass function for the multinomial
    /// distribution
    /// with the given `x`'s corresponding to the probabilities for this
    /// distribution
    ///
    /// # Errors
    ///
    /// If the elements in `x` do not sum to `n` or if the length of `x` is not
    /// equivalent to the length of `p`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln((n! / x_1!...x_k!) * p_i^x_i) for i in 1...k
    /// ```
    ///
    /// where `n` is the number of trials, `p_i` is the `i`th probability,
    /// `x_i` is the `i`th `x` value, and `k` is the total number of
    /// probabilities
    fn checked_ln_pmf(&self, x: &[u64]) -> Result<f64> {
        if self.p.len() != x.len() {
            return Err(StatsError::ContainersMustBeSameLength);
        }
        if x.iter().sum::<u64>() != self.n {
            return Err(StatsError::ContainerExpectedSumVar("x", "n"));
        }
        let coeff = factorial::multinomial(self.n, x).ln();
        let val = coeff
            + self
                .p
                .iter()
                .zip(x.iter())
                .map(|(pi, xi)| *xi as f64 * pi.ln())
                .fold(0.0, |acc, x| acc + x);
        Ok(val)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use crate::statistics::*;
    use crate::distribution::{CheckedDiscrete, Discrete, Multinomial};

    fn try_create(p: &[f64], n: u64) -> Multinomial {
        let dist = Multinomial::new(p, n);
        assert!(dist.is_ok());
        dist.unwrap()
    }

    fn create_case(p: &[f64], n: u64) {
        let dist = try_create(p, n);
        assert_eq!(dist.p(), p);
        assert_eq!(dist.n(), n);
    }

    fn bad_create_case(p: &[f64], n: u64) {
        let dist = Multinomial::new(p, n);
        assert!(dist.is_err());
    }

    fn test_case<F>(p: &[f64], n: u64, expected: &[f64], eval: F)
        where F: Fn(Multinomial) -> Vec<f64>
    {
        let dist = try_create(p, n);
        let x = eval(dist);
        assert_eq!(*expected, *x);
    }

    fn test_almost<F>(p: &[f64], n: u64, expected: &[f64], acc: f64, eval: F)
        where F: Fn(Multinomial) -> Vec<f64>
    {
        let dist = try_create(p, n);
        let x = eval(dist);
        assert_eq!(expected.len(), x.len());
        for i in 0..expected.len() {
            assert_almost_eq!(expected[i], x[i], acc);
        }
    }

    fn test_almost_sr<F>(p: &[f64], n: u64, expected: f64, acc:f64, eval: F)
        where F: Fn(Multinomial) -> f64
    {
        let dist = try_create(p, n);
        let x = eval(dist);
        assert_almost_eq!(expected, x, acc);
    }

    #[test]
    fn test_create() {
        create_case(&[1.0, 1.0, 1.0], 4);
        create_case(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 4);
    }

    #[test]
    fn test_bad_create() {
        bad_create_case(&[-1.0, 1.0], 4);
        bad_create_case(&[0.0, 0.0], 4);
    }

    #[test]
    fn test_mean() {
        test_case(&[0.3, 0.7], 5, &[1.5, 3.5], |x| x.mean());
        test_case(&[0.1, 0.3, 0.6], 10, &[1.0, 3.0, 6.0], |x| x.mean());
        test_case(&[0.15, 0.35, 0.3, 0.2], 20, &[3.0, 7.0, 6.0, 4.0], |x| x.mean());
    }

    #[test]
    fn test_variance() {
        test_almost(&[0.3, 0.7], 5, &[1.05, 1.05], 1e-15, |x| x.variance());
        test_almost(&[0.1, 0.3, 0.6], 10, &[0.9, 2.1, 2.4], 1e-15, |x| x.variance());
        test_almost(&[0.15, 0.35, 0.3, 0.2], 20, &[2.55, 4.55, 4.2, 3.2], 1e-15, |x| x.variance());
    }

    #[test]
    fn test_std_dev() {
        test_almost(&[0.3, 0.7], 5, &[1.05f64.sqrt(), 1.05f64.sqrt()], 1e-15, |x| x.std_dev());
        test_almost(&[0.1, 0.3, 0.6], 10, &[0.9f64.sqrt(), 2.1f64.sqrt(), 2.4f64.sqrt()], 1e-15, |x| x.std_dev());
        test_almost(&[0.15, 0.35, 0.3, 0.2], 20, &[2.55f64.sqrt(), 4.55f64.sqrt(), 4.2f64.sqrt(), 3.2f64.sqrt()], 1e-15, |x| x.std_dev());
    }

    #[test]
    fn test_skewness() {
        test_almost(&[0.3, 0.7], 5, &[0.390360029179413, -0.390360029179413], 1e-15, |x| x.skewness());
        test_almost(&[0.1, 0.3, 0.6], 10, &[0.843274042711568, 0.276026223736942, -0.12909944487358], 1e-15, |x| x.skewness());
        test_almost(&[0.15, 0.35, 0.3, 0.2], 20, &[0.438357003759605, 0.140642169281549, 0.195180014589707, 0.335410196624968], 1e-15, |x| x.skewness());
    }

    #[test]
    fn test_pmf() {
        test_almost_sr(&[0.3, 0.7], 10, 0.121060821, 1e-15, |x| x.pmf(&[1, 9]));
        test_almost_sr(&[0.1, 0.3, 0.6], 10, 0.105815808, 1e-15, |x| x.pmf(&[1, 3, 6]));
        test_almost_sr(&[0.15, 0.35, 0.3, 0.2], 10, 0.000145152, 1e-15, |x| x.pmf(&[1, 1, 1, 7]));
    }

    #[test]
    #[should_panic]
    fn test_pmf_x_wrong_length() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        n.pmf(&[1]);
    }

    #[test]
    #[should_panic]
    fn test_pmf_x_wrong_sum() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        n.pmf(&[1, 3]);
    }

    #[test]
    fn test_checked_pmf_x_wrong_length() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        assert!(n.checked_pmf(&[1]).is_err());
    }

    #[test]
    fn test_checked_pmf_x_wrong_sum() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        assert!(n.checked_pmf(&[1, 3]).is_err());
    }

    #[test]
    fn test_ln_pmf() {
        let large_p = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let n = Multinomial::new(large_p, 45).unwrap();
        let x = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_almost_eq!(n.pmf(x).ln(), n.ln_pmf(x), 1e-13);
        let n2 = Multinomial::new(large_p, 18).unwrap();
        let x2 = &[1, 1, 1, 2, 2, 2, 3, 3, 3];
        assert_almost_eq!(n2.pmf(x2).ln(), n2.ln_pmf(x2), 1e-13);
        let n3 = Multinomial::new(large_p, 51).unwrap();
        let x3 = &[5, 6, 7, 8, 7, 6, 5, 4, 3];
        assert_almost_eq!(n3.pmf(x3).ln(), n3.ln_pmf(x3), 1e-13);
    }

    #[test]
    #[should_panic]
    fn test_ln_pmf_x_wrong_length() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        n.ln_pmf(&[1]);
    }

    #[test]
    #[should_panic]
    fn test_ln_pmf_x_wrong_sum() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        n.ln_pmf(&[1, 3]);
    }

    #[test]
    fn test_checked_ln_pmf_x_wrong_length() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        assert!(n.checked_ln_pmf(&[1]).is_err());
    }

    #[test]
    fn test_checked_ln_pmf_x_wrong_sum() {
        let n = Multinomial::new(&[0.3, 0.7], 10).unwrap();
        assert!(n.checked_ln_pmf(&[1, 3]).is_err());
    }
}
