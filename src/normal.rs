use rand::{CryptoRng, Rng};

pub trait NormalSample
where
    Self: Sized,
{
    type Mean: Copy + Clone;
    type Variance: Copy + Clone;

    fn normal_sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng) -> Self;

    fn normal_crypt_sample(
        mean: Self::Mean,
        variance: Self::Variance,
        rng: &mut (impl CryptoRng + Rng),
    ) -> Self {
        Self::normal_sample(mean, variance, rng)
    }

    fn normal_sample_n(
        mean: Self::Mean,
        variance: Self::Variance,
        rng: &mut impl Rng,
        n: usize,
    ) -> Vec<Self> {
        (0..n).map(|_| Self::normal_sample(mean, variance, rng)).collect()
    }

    fn normal_crypt_sample_n(
        mean: Self::Mean,
        variance: Self::Variance,
        rng: &mut (impl CryptoRng + Rng),
        n: usize,
    ) -> Vec<Self> {
        (0..n)
            .map(|_| Self::normal_crypt_sample(mean, variance, rng))
            .collect()
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

    pub fn normal_sample(&self, rng: &mut impl Rng) -> Sample {
        Sample::normal_sample(self.mean, self.variance, rng)
    }

    pub fn normal_crypt_sample(&self, rng: &mut (impl CryptoRng + Rng)) -> Sample {
        Sample::normal_crypt_sample(self.mean, self.variance, rng)
    }

    pub fn normal_sample_n(&self, rng: &mut impl Rng, n: usize) -> Vec<Sample> {
        Sample::normal_sample_n(self.mean, self.variance, rng, n)
    }

    pub fn normal_crypt_sample_n(&self, rng: &mut (impl CryptoRng + Rng), n: usize) -> Vec<Sample> {
        Sample::normal_crypt_sample_n(self.mean, self.variance, rng, n)
    }
}

macro_rules! impl_normal_distribution {
    ($type:ty) => {
        impl NormalSample for $type {
            type Mean = $type;
            type Variance = $type;

            fn normal_sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng) -> Self {
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

    fn normal_sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl Rng) -> Self {
        array_macro::array![_ => T::normal_sample(mean, variance, rng); N]
    }
}
