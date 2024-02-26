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

macro_rules! strategy_byte_array_seed {
    () => {
        prop::array::uniform32(prop::num::u8::ANY)
    };
}

proptest! {
    // =================================================================
    // test [u8; 32] seeding
    // =================================================================
    #[test]
    fn test_byte_array_seeding_via_simplex_1d(seed in strategy_byte_array_seed!(), point in strategy_array_float_numeric!()) {
        let n = Source::<1>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_byte_array_seeding_via_simplex_2d(seed in strategy_byte_array_seed!(), point in strategy_array_float_numeric!()) {
        let n = Source::<2>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n) || n.is_nan(), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_byte_array_seeding_via_simplex_3d(seed in strategy_byte_array_seed!(), point in strategy_array_float_numeric!()) {
        let n = Source::<3>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n) || n.is_nan(), "value not in [-1, 1] range, instead: {}", n);
    }

    #[test]
    fn test_byte_array_seeding_via_simplex_4d(seed in strategy_byte_array_seed!(), point in strategy_array_float_numeric!()) {
        let n = Source::<4>::simplex(seed).sample(point);
        prop_assert!((-1.0..=1.0).contains(&n) || n.is_nan(), "value not in [-1, 1] range, instead: {}", n);
    }
}
