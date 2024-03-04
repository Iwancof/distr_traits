#[cfg(test)]
mod tests {
    use distr_traits::normal::NormalSample;
    use distr_traits_derive::NormalSample;

    #[derive(NormalSample, Debug)]
    struct Coordinate {
        x: f64,
        y: f64,
    }

    #[test]
    fn test_struct_normal_sample() {
        let _ = Coordinate::normal_sample(0., 1., &mut rand::rngs::OsRng);
    }

    #[test]
    fn test_struct_mean() {
        let coordinates = (0..1000).map(|_| Coordinate::normal_sample(0., 1., &mut rand::rngs::OsRng));

        let mut x_sum = 0.;
        let mut y_sum = 0.;
        for c in coordinates {
            x_sum += c.x;
            y_sum += c.y;
        }

        let x_mean = x_sum / 1000.;
        let y_mean = y_sum / 1000.;

        assert!(x_mean.abs() < 1.);
        assert!(y_mean.abs() < 1.);
    }

    #[test]
    fn test_generate_big() {
        for _ in 0..10000 {
            let c = Coordinate::normal_sample(0., 1., &mut rand::rngs::OsRng);
            if 1. < c.x.abs() || 1. < c.y.abs() {
                return;
            }
        }
        panic!("All coordinates are within the range");
    }

    #[derive(NormalSample, Debug)]
    struct CoordinateTuple(f64, f64);

    #[test]
    fn test_tuple_normal_sample() {
        let _ = CoordinateTuple::normal_sample(0., 1., &mut rand::rngs::OsRng);
    }

    #[test]
    fn test_tuple_mean() {
        let coordinates = (0..1000).map(|_| CoordinateTuple::normal_sample(0., 1., &mut rand::rngs::OsRng));

        let mut x_sum = 0.;
        let mut y_sum = 0.;
        for c in coordinates {
            x_sum += c.0;
            y_sum += c.1;
        }

        let x_mean = x_sum / 1000.;
        let y_mean = y_sum / 1000.;

        assert!(x_mean.abs() < 1.);
        assert!(y_mean.abs() < 1.);
    }

    #[test]
    fn test_tuple_generate_big() {
        for _ in 0..10000 {
            let c = CoordinateTuple::normal_sample(0., 1., &mut rand::rngs::OsRng);
            if 1. < c.0.abs() || 1. < c.1.abs() {
                return;
            }
        }
        panic!("All coordinates are within the range");
    }
}
