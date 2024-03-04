use rand::{Rng, CryptoRng};

pub trait UniformSample 
where
    Self: Sized,
{
    fn uniform_sample(rng: &mut impl Rng) -> Self;

    fn uniform_crypt_sample(rng: &mut (impl Rng + CryptoRng)) -> Self {
        Self::uniform_sample(rng)
    }

    fn uniform_sample_n(rng: &mut impl Rng, n: usize) -> Vec<Self> {
        (0..n).map(|_| Self::uniform_sample(rng)).collect()
    }

    fn uniform_crypt_sample_n(rng: &mut (impl Rng + CryptoRng), n: usize) -> Vec<Self> {
        (0..n).map(|_| Self::uniform_crypt_sample(rng)).collect()
    }
}

macro_rules! impl_uniform_sample {
    ($($t:ty),*) => {
        $(
            impl UniformSample for $t {
                fn uniform_sample(rng: &mut impl Rng) -> Self {
                    rng.gen()
                }
            }
        )*
    };
}

impl_uniform_sample!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl<const N: usize, T: UniformSample> UniformSample for [T; N] {
    fn uniform_sample(rng: &mut impl Rng) -> Self {
        array_macro::array![_ => T::uniform_sample(rng); N]
    }
}
