#[cfg(test)]
mod tests {
    use distr_traits::uniform::UniformSample;
    use distr_traits_derive::UniformSample;

    #[derive(UniformSample, Debug)]
    struct Coordinate {
        x: i8,
        y: i8,
    }

    #[test]
    fn test_struct_uniform_sample() {
        let _ = Coordinate::uniform_sample(&mut rand::rngs::OsRng);
    }

    #[test]
    fn test_struct_uniform_generate() {
        for _ in 0..10000 {
            let Coordinate { x, y } = Coordinate::uniform_sample(&mut rand::rngs::OsRng);
            let _ = y;
            if x == 0x55 {
                return;
            }
        }
        panic!("Coordinate::sample() is not uniform");
    }

    #[derive(UniformSample, Debug)]
    struct CoordinateTuple(i8, i8);

    #[test]
    fn test_tuple_uniform_sample() {
        let _ = CoordinateTuple::uniform_sample(&mut rand::rngs::OsRng);
    }

    #[test]
    fn test_tuple_uniform_generate() {
        for _ in 0..10000 {
            let CoordinateTuple(x, _y) = CoordinateTuple::uniform_sample(&mut rand::rngs::OsRng);
            if x == 0x55 {
                return;
            }
        }
        panic!("CoordinateTuple::sample() is not uniform");
    }
}

