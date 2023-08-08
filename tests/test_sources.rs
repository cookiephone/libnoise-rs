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
    // test source: checkerboard
    // =================================================================
    #[test]
    fn test_checkerboard_1d(point in strategy_array_float_numeric!()) {
        let n = Source::<1>::checkerboard().sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_checkerboard_2d(point in strategy_array_float_numeric!()) {
        let n = Source::<2>::checkerboard().sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_checkerboard_3d(point in strategy_array_float_numeric!()) {
        let n = Source::<3>::checkerboard().sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_checkerboard_4d(point in strategy_array_float_numeric!()) {
        let n = Source::<4>::checkerboard().sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    // =================================================================
    // test source: constant
    // =================================================================
    #[test]
    fn test_constant_1d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::constant(value).sample(point);
        prop_assert_eq!(value, n, "value {} was not emitted, instead: {}", value, n);
    }

    #[test]
    fn test_constant_2d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::constant(value).sample(point);
        prop_assert_eq!(value, n, "value {} was not emitted, instead: {}", value, n);
    }

    #[test]
    fn test_constant_3d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::constant(value).sample(point);
        prop_assert_eq!(value, n, "value {} was not emitted, instead: {}", value, n);
    }

    #[test]
    fn test_constant_4d(value in strategy_float_numeric!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::constant(value).sample(point);
        prop_assert_eq!(value, n, "value {} was not emitted, instead: {}", value, n);
    }

    // =================================================================
    // test source: custom
    // =================================================================
    #[test]
    fn test_custom_1d(point in strategy_array_float_numeric!()) {
        let closure = |p: [f64; 1]| p.iter().sum();
        let n = Source::<1>::custom(closure).sample(point);
        let expected = closure(point);
        prop_assert_eq!(expected, n, "expected {}, instead: {}", expected, n);
    }

    #[test]
    fn test_custom_2d(point in strategy_array_float_numeric!()) {
        let closure = |p: [f64; 2]| p.iter().sum();
        let n = Source::<2>::custom(closure).sample(point);
        let expected = closure(point);
        prop_assert_eq!(expected, n, "expected {}, instead: {}", expected, n);
    }

    #[test]
    fn test_custom_3d(point in strategy_array_float_numeric!()) {
        let closure = |p: [f64; 3]| p.iter().sum();
        let n = Source::<3>::custom(closure).sample(point);
        let expected = closure(point);
        prop_assert_eq!(expected, n, "expected {}, instead: {}", expected, n);
    }

    #[test]
    fn test_custom_4d(point in strategy_array_float_numeric!()) {
        let closure = |p: [f64; 4]| p.iter().sum();
        let n = Source::<4>::custom(closure).sample(point);
        let expected = closure(point);
        prop_assert_eq!(expected, n, "expected {}, instead: {}", expected, n);
    }

    // =================================================================
    // test source: improved_perlin
    // =================================================================
    #[test]
    fn test_improved_perlin_1d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::improved_perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_improved_perlin_2d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::improved_perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }


    #[test]
    fn test_improved_perlin_3d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::improved_perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_improved_perlin_4d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::improved_perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    // =================================================================
    // test source: perlin
    // =================================================================
    #[test]
    fn test_perlin_1d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_perlin_2d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_perlin_3d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_perlin_4d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::perlin(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    // =================================================================
    // test source: simplex
    // =================================================================
    #[test]
    fn test_simplex_1d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_simplex_2d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n) || n.is_nan(), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_simplex_3d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n) || n.is_nan(), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_simplex_4d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n) || n.is_nan(), "value not in [-1, 1] range, instead: {}", n);
    }

    // =================================================================
    // test source: value
    // =================================================================
    #[test]
    fn test_value_1d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::value(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_value_2d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::value(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_value_3d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::value(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_value_4d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::worley(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    // =================================================================
    // test source: worley
    // =================================================================
    #[test]
    fn test_worley_1d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<1>::worley(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_worley_2d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<2>::worley(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_worley_3d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<3>::worley(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_worley_4d(seed in prop::num::u64::ANY, point in strategy_array_float_numeric!()) {
        let n = Source::<4>::worley(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }
}
