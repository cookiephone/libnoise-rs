use libnoise::prelude::*;
use proptest::prelude::*;

macro_rules! strategy_float_numeric {
    () => {
        prop::num::f64::NORMAL
            | prop::num::f64::NEGATIVE
            | prop::num::f64::POSITIVE
            | prop::num::f64::ZERO
    };
}

macro_rules! strategy_array_float_numeric {
    () => {
        prop::array::uniform(strategy_float_numeric!())
    };
}

proptest! {
    // =================================================================
    // test adapter: abs
    // =================================================================
    #[test]
    fn test_abs_1d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).abs().sample(point);
        let expected = value.abs();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_abs_2d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).abs().sample(point);
        let expected = value.abs();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_abs_3d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).abs().sample(point);
        let expected = value.abs();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_abs_4d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).abs().sample(point);
        let expected = value.abs();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: add
    // =================================================================
    #[test]
    fn test_add_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).add(value_b).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_add_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).add(value_b).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_add_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).add(value_b).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_add_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).add(value_b).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: billow
    // =================================================================
    #[test]
    fn test_billow_1d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).billow(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * (value.abs() * 2.0 - 1.0);
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_billow_2d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).billow(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * (value.abs() * 2.0 - 1.0);
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_billow_3d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).billow(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * (value.abs() * 2.0 - 1.0);
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_billow_4d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).billow(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * (value.abs() * 2.0 - 1.0);
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: blend
    // =================================================================
    #[test]
    fn test_blend_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in -1_f64..1.0, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).blend(Source::<1>::constant(value_b), Source::<1>::constant(value_control)).sample(point);
        let expected = value_a + (value_control * 0.5 + 0.5) * (value_b - value_a);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_blend_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in -1_f64..1.0, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).blend(Source::<2>::constant(value_b), Source::<2>::constant(value_control)).sample(point);
        let expected = value_a + (value_control * 0.5 + 0.5) * (value_b - value_a);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_blend_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in -1_f64..1.0, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).blend(Source::<3>::constant(value_b), Source::<3>::constant(value_control)).sample(point);
        let expected = value_a + (value_control * 0.5 + 0.5) * (value_b - value_a);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_blend_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in -1_f64..1.0, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).blend(Source::<4>::constant(value_b), Source::<4>::constant(value_control)).sample(point);
        let expected = value_a + (value_control * 0.5 + 0.5) * (value_b - value_a);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: clamp
    // =================================================================
    #[test]
    fn test_clamp_1d(value in strategy_float_numeric!(), min in strategy_float_numeric!(), max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        if min > max {
            return Ok(());
        }
        let n = Source::<1>::constant(value).clamp(min, max).sample(point);
        let expected = value.clamp(min, max);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_clamp_2d(value in strategy_float_numeric!(), min in strategy_float_numeric!(), max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        if min > max {
            return Ok(());
        }
        let n = Source::<2>::constant(value).clamp(min, max).sample(point);
        let expected = value.clamp(min, max);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_clamp_3d(value in strategy_float_numeric!(), min in strategy_float_numeric!(), max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        if min > max {
            return Ok(());
        }
        let n = Source::<3>::constant(value).clamp(min, max).sample(point);
        let expected = value.clamp(min, max);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_clamp_4d(value in strategy_float_numeric!(), min in strategy_float_numeric!(), max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        if min > max {
            return Ok(());
        }
        let n = Source::<4>::constant(value).clamp(min, max).sample(point);
        let expected = value.clamp(min, max);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: displace
    // =================================================================
    #[test]
    fn test_displace_x_1d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).displace_x(Source::<1>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_x_2d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).displace_x(Source::<2>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_y_2d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).displace_y(Source::<2>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_x_3d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).displace_x(Source::<3>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_y_3d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).displace_y(Source::<3>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_z_3d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).displace_z(Source::<3>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_x_4d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).displace_x(Source::<4>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_y_4d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).displace_y(Source::<4>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_z_4d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).displace_z(Source::<4>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_displace_w_4d(value in strategy_float_numeric!(), displacement in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).displace_w(Source::<4>::constant(displacement)).sample(point);
        let expected = value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: exp
    // =================================================================
    #[test]
    fn test_exp_1d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).exp().sample(point);
        let expected = value.exp();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_exp_2d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).exp().sample(point);
        let expected = value.exp();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_exp_3d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).exp().sample(point);
        let expected = value.exp();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_exp_4d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).exp().sample(point);
        let expected = value.exp();
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: fbm
    // =================================================================
    #[test]
    fn test_fbm_1d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).fbm(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * value;
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_fbm_2d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).fbm(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * value;
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_fbm_3d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).fbm(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * value;
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_fbm_4d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), persistence in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).fbm(octaves, frequency, lacunarity, persistence).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            expected += amp * value;
            amp *= persistence;
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| acc + persistence.powi(octave as i32));
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: lambda
    // =================================================================
    #[test]
    fn test_lambda_1d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let closure = |x| x * x - 1.0;
        let n = Source::<1>::constant(value).lambda(closure).sample(point);
        let expected = closure(value);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_lambda_2d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let closure = |x| x * x - 1.0;
        let n = Source::<2>::constant(value).lambda(closure).sample(point);
        let expected = closure(value);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_lambda_3d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let closure = |x| x * x - 1.0;
        let n = Source::<3>::constant(value).lambda(closure).sample(point);
        let expected = closure(value);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_lambda_4d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let closure = |x| x * x - 1.0;
        let n = Source::<4>::constant(value).lambda(closure).sample(point);
        let expected = closure(value);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: max
    // =================================================================
    #[test]
    fn test_max_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).max(Source::<1>::constant(value_b)).sample(point);
        let expected = value_a.max(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_max_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).max(Source::<2>::constant(value_b)).sample(point);
        let expected = value_a.max(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_max_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).max(Source::<3>::constant(value_b)).sample(point);
        let expected = value_a.max(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_max_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).max(Source::<4>::constant(value_b)).sample(point);
        let expected = value_a.max(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: min
    // =================================================================
    #[test]
    fn test_min_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).min(Source::<1>::constant(value_b)).sample(point);
        let expected = value_a.min(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_min_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).min(Source::<2>::constant(value_b)).sample(point);
        let expected = value_a.min(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_min_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).min(Source::<3>::constant(value_b)).sample(point);
        let expected = value_a.min(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_min_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).min(Source::<4>::constant(value_b)).sample(point);
        let expected = value_a.min(value_b);
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: mul
    // =================================================================
    #[test]
    fn test_mul_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).mul(value_b).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_mul_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).mul(value_b).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_mul_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).mul(value_b).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_mul_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).mul(value_b).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: neg
    // =================================================================
    #[test]
    fn test_neg_1d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).neg().sample(point);
        let expected = -value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_neg_2d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).neg().sample(point);
        let expected = -value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_neg_3d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).neg().sample(point);
        let expected = -value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_neg_4d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).neg().sample(point);
        let expected = -value;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: pow
    // =================================================================
    #[test]
    fn test_powi_1d(value in strategy_float_numeric!(), exponent in -1000_i32..1000, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).powi(exponent).sample(point);
        let expected = value.powi(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powi_2d(value in strategy_float_numeric!(), exponent in -1000_i32..1000, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).powi(exponent).sample(point);
        let expected = value.powi(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powi_3d(value in strategy_float_numeric!(), exponent in -1000_i32..1000, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).powi(exponent).sample(point);
        let expected = value.powi(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powi_4d(value in strategy_float_numeric!(), exponent in -1000_i32..1000, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).powi(exponent).sample(point);
        let expected = value.powi(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powf_1d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).powf(exponent).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powf_2d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).powf(exponent).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powf_3d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).powf(exponent).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_powf_4d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).powf(exponent).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: power
    // =================================================================
    #[test]
    fn test_power_1d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).power(Source::<1>::constant(exponent)).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_power_2d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).power(Source::<2>::constant(exponent)).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_power_3d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).power(Source::<3>::constant(exponent)).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_power_4d(value in strategy_float_numeric!(), exponent in -1000_f64..1000.0, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).power(Source::<4>::constant(exponent)).sample(point);
        let expected = value.powf(exponent);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: product
    // =================================================================
    #[test]
    fn test_product_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).product(Source::<1>::constant(value_b)).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_product_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).product(Source::<2>::constant(value_b)).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_product_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).product(Source::<3>::constant(value_b)).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_product_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).product(Source::<4>::constant(value_b)).sample(point);
        let expected = value_a * value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: ridgedmulti
    // =================================================================
    #[test]
    fn test_ridgedmulti_1d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), attenuation in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).ridgedmulti(octaves, frequency, lacunarity, attenuation).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            let mut layer = 1.0 - value.abs();
            layer *= layer;
            layer *= amp;
            expected += layer;
            amp = (layer / attenuation).clamp(0.0, 1.0);
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| {
            acc + (1.0 / attenuation).powi(octave as i32)
        });
        expected = expected * 2.0 - 1.0;
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_ridgedmulti_2d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), attenuation in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).ridgedmulti(octaves, frequency, lacunarity, attenuation).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            let mut layer = 1.0 - value.abs();
            layer *= layer;
            layer *= amp;
            expected += layer;
            amp = (layer / attenuation).clamp(0.0, 1.0);
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| {
            acc + (1.0 / attenuation).powi(octave as i32)
        });
        expected = expected * 2.0 - 1.0;
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_ridgedmulti_3d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), attenuation in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).ridgedmulti(octaves, frequency, lacunarity, attenuation).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            let mut layer = 1.0 - value.abs();
            layer *= layer;
            layer *= amp;
            expected += layer;
            amp = (layer / attenuation).clamp(0.0, 1.0);
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| {
            acc + (1.0 / attenuation).powi(octave as i32)
        });
        expected = expected * 2.0 - 1.0;
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_ridgedmulti_4d(value in strategy_float_numeric!(), octaves in 0_u32..10, frequency in strategy_float_numeric!(), lacunarity in strategy_float_numeric!(), attenuation in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).ridgedmulti(octaves, frequency, lacunarity, attenuation).sample(point);
        let mut expected = 0.0;
        let mut amp = 1.0;
        for _ in 0..octaves {
            let mut layer = 1.0 - value.abs();
            layer *= layer;
            layer *= amp;
            expected += layer;
            amp = (layer / attenuation).clamp(0.0, 1.0);
        }
        expected *= 1.0 / (0..octaves).fold(0.0, |acc, octave| {
            acc + (1.0 / attenuation).powi(octave as i32)
        });
        expected = expected * 2.0 - 1.0;
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: rotate
    // =================================================================
    #[test]
    fn test_rotate_2d(seed in prop::num::u64::ANY, rotation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        Source::<2>::simplex(seed).rotate(rotation).sample(point);
    }

    #[test]
    fn test_rotate_3d(seed in prop::num::u64::ANY, rotation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        Source::<3>::simplex(seed).rotate(rotation).sample(point);
    }

    #[test]
    fn test_rotate_4d(seed in prop::num::u64::ANY, rotation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        Source::<4>::simplex(seed).rotate(rotation).sample(point);
    }

    // =================================================================
    // test adapter: scale
    // =================================================================
    #[test]
    fn test_scale_1d(seed in prop::num::u64::ANY, scale in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::simplex(seed).scale(scale).sample(point);
        let expected = Source::<1>::simplex(seed).sample([point[0] * scale[0]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_scale_2d(seed in prop::num::u64::ANY, scale in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::simplex(seed).scale(scale).sample(point);
        let expected = Source::<2>::simplex(seed).sample([point[0] * scale[0], point[1] * scale[1]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_scale_3d(seed in prop::num::u64::ANY, scale in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::simplex(seed).scale(scale).sample(point);
        let expected = Source::<3>::simplex(seed).sample([point[0] * scale[0], point[1] * scale[1], point[2] * scale[2]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_scale_4d(seed in prop::num::u64::ANY, scale in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::simplex(seed).scale(scale).sample(point);
        let expected = Source::<4>::simplex(seed).sample([point[0] * scale[0], point[1] * scale[1], point[2] * scale[2], point[3] * scale[3]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: select
    // =================================================================
    #[test]
    fn test_select_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in strategy_float_numeric!(), selection_min in strategy_float_numeric!(), selection_max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).select(Source::<1>::constant(value_b), Source::<1>::constant(value_control), selection_min, selection_max).sample(point);
        let expected = match value_control {
            t if t >= selection_min && t <= selection_max => {
                value_a
            }
            _ => value_b
        };
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_select_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in strategy_float_numeric!(), selection_min in strategy_float_numeric!(), selection_max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).select(Source::<2>::constant(value_b), Source::<2>::constant(value_control), selection_min, selection_max).sample(point);
        let expected = match value_control {
            t if t >= selection_min && t <= selection_max => {
                value_a
            }
            _ => value_b
        };
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_select_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in strategy_float_numeric!(), selection_min in strategy_float_numeric!(), selection_max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).select(Source::<3>::constant(value_b), Source::<3>::constant(value_control), selection_min, selection_max).sample(point);
        let expected = match value_control {
            t if t >= selection_min && t <= selection_max => {
                value_a
            }
            _ => value_b
        };
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_select_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), value_control in strategy_float_numeric!(), selection_min in strategy_float_numeric!(), selection_max in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).select(Source::<4>::constant(value_b), Source::<4>::constant(value_control), selection_min, selection_max).sample(point);
        let expected = match value_control {
            t if t >= selection_min && t <= selection_max => {
                value_a
            }
            _ => value_b
        };
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: sum
    // =================================================================
    #[test]
    fn test_sum_1d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value_a).sum(Source::<1>::constant(value_b)).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_sum_2d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value_a).sum(Source::<2>::constant(value_b)).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_sum_3d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value_a).sum(Source::<3>::constant(value_b)).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_sum_4d(value_a in strategy_float_numeric!(), value_b in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value_a).sum(Source::<4>::constant(value_b)).sample(point);
        let expected = value_a + value_b;
        prop_assert_eq!(n, expected, "expected value {}, instead: {}", n, expected);
    }

    // =================================================================
    // test adapter: translate
    // =================================================================
    #[test]
    fn test_translate_1d(seed in prop::num::u64::ANY, translation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::simplex(seed).translate(translation).sample(point);
        let expected = Source::<1>::simplex(seed).sample([point[0] + translation[0]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_translate_2d(seed in prop::num::u64::ANY, translation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::simplex(seed).translate(translation).sample(point);
        let expected = Source::<2>::simplex(seed).sample([point[0] + translation[0], point[1] + translation[1]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_translate_3d(seed in prop::num::u64::ANY, translation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::simplex(seed).translate(translation).sample(point);
        let expected = Source::<3>::simplex(seed).sample([point[0] + translation[0], point[1] + translation[1], point[2] + translation[2]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }

    #[test]
    fn test_translate_4d(seed in prop::num::u64::ANY, translation in strategy_array_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::simplex(seed).translate(translation).sample(point);
        let expected = Source::<4>::simplex(seed).sample([point[0] + translation[0], point[1] + translation[1], point[2] + translation[2], point[3] + translation[3]]);
        prop_assert!(n == expected || (n.is_nan() && expected.is_nan()), "expected value {}, instead: {}", n, expected);
    }
}
