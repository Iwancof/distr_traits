use rand::{Rng, CryptoRng};

pub trait NormalSample 
where   
    Self: Sized,
{
    type Mean: Copy + Clone;
    type Variance: Copy + Clone;

    fn sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng) -> Self;

    fn crypt_sample(mean: Self::Mean, variance: Self::Variance, rng: &mut (impl CryptoRng + Rng)) -> Self {
        Self::sample(mean, variance, rng)
    }

    fn sample_n(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng, n: usize) -> Vec<Self> {
        (0..n).map(|_| Self::sample(mean, variance, rng)).collect()
    }

    fn crypt_sample_n(mean: Self::Mean, variance: Self::Variance, rng: &mut (impl CryptoRng + Rng), n: usize) -> Vec<Self> {
        (0..n).map(|_| Self::crypt_sample(mean, variance, rng)).collect()
    }
}

pub struct NormalHelper<Sample: NormalSample> {
    mean: Sample::Mean,
    variance: Sample::Variance,
}

impl<Sample: NormalSample> NormalHelper<Sample> {
    pub fn new(mean: Sample::Mean, variance: Sample::Variance) -> Self {
        Self { mean, variance }
    }

    pub fn sample(&self, rng: &mut impl Rng) -> Sample {
        Sample::sample(self.mean, self.variance, rng)
    }

    pub fn crypt_sample(&self, rng: &mut (impl CryptoRng + Rng)) -> Sample {
        Sample::crypt_sample(self.mean, self.variance, rng)
    }

    pub fn sample_n(&self, rng: &mut impl Rng, n: usize) -> Vec<Sample> {
        Sample::sample_n(self.mean, self.variance, rng, n)
    }

    pub fn crypt_sample_n(&self, rng: &mut (impl CryptoRng + Rng), n: usize) -> Vec<Sample> {
        Sample::crypt_sample_n(self.mean, self.variance, rng, n)
    }
}

macro_rules! impl_normal_distribution {
    ($type:ty) => {
        impl NormalSample for $type {
            type Mean = $type;
            type Variance = $type;

            fn sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng) -> Self {
                let normal = rand_distr::Normal::new(mean, variance).unwrap();
                rng.sample(normal)
            }
        }
    };
}

impl_normal_distribution!(f32);
impl_normal_distribution!(f64);

impl<const N: usize, T: NormalSample> NormalSample for [T; N] {
    type Mean = T::Mean;
    type Variance = T::Variance;

    fn sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng) -> Self {
        array_macro::array![_ => T::sample(mean, variance, rng); N]
    }
}
