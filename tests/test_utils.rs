use libnoise::prelude::*;
use proptest::prelude::*;
use tempdir::TempDir;

macro_rules! strategy_float_numeric {
    () => {
        prop::num::f64::NORMAL
            | prop::num::f64::NEGATIVE
            | prop::num::f64::POSITIVE
            | prop::num::f64::ZERO
    };
}

proptest! {
    // =================================================================
    // test NoiseBuffer
    // =================================================================
    #[test]
    fn test_noises_buffer_1d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        NoiseBuffer::<1>::new([1000], &generator);
    }

    #[test]
    fn test_noises_buffer_2d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        NoiseBuffer::<2>::new([100, 100], &generator);
    }

    #[test]
    fn test_noises_buffer_3d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        NoiseBuffer::<3>::new([30, 30, 30], &generator);
    }

    #[test]
    fn test_noises_buffer_4d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        NoiseBuffer::<4>::new([10, 10, 10, 10], &generator);
    }

    // =================================================================
    // test Visualizer
    // =================================================================
    #[test]
    fn test_visualizer_1d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        let tmp_dir = TempDir::new("libnoise").unwrap();
        let path = &tmp_dir.path().join("output.png").into_os_string().into_string().unwrap();
        Visualizer::<1>::new([1000], &generator).write_to_file(path).unwrap();
    }

    #[test]
    fn test_visualizer_2d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        let tmp_dir = TempDir::new("libnoise").unwrap();
        let path = &tmp_dir.path().join("output.png").into_os_string().into_string().unwrap();
        Visualizer::<2>::new([100, 100], &generator).write_to_file(path).unwrap();
    }

    #[test]
    fn test_visualizer_3d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        let tmp_dir = TempDir::new("libnoise").unwrap();
        let path = &tmp_dir.path().join("output.png").into_os_string().into_string().unwrap();
        Visualizer::<3>::new([30, 30, 30], &generator).write_to_file(path).unwrap();
    }

    #[test]
    fn test_visualizer_4d(val in strategy_float_numeric!()) {
        let generator = Source::constant(val);
        let tmp_dir = TempDir::new("libnoise").unwrap();
        let path = &tmp_dir.path().join("output.png").into_os_string().into_string().unwrap();
        Visualizer::<4>::new([10, 10, 10, 10], &generator).write_to_file(path).unwrap();
    }
}
