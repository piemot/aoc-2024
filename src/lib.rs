// Based on https://www.reddit.com/r/adventofcode/comments/e5sa2d/comment/f9nea6y
/// Creates a `main()` function for each day, which automatically loads the appropriate input from `dayN.txt`.
///
/// The first argument of each function must be `&str`, unless the function is prefixed with `byte`.
/// If it is, `&[u8]` will be provided instead.
///
/// ## Note
/// Both functions must either be marked as `byte`, or not marked as `byte`.
#[macro_export]
macro_rules! day {
    ($($part:ident),+) => {
        const INPUT: &str = include_str!(concat!(module_path!(), ".txt"));
        fn main() {
            $(println!("{}: {}", stringify!($part), $part(INPUT));)+
        }
    };
    ($(byte $part:ident),+) => {
        const INPUT: &str = include_bytes!(concat!(module_path!(), ".txt"));
        fn main() {
            $(println!("{}: {}", stringify!($part), $part(INPUT));)+
        }
    };
}

/// Generates a testing function (with the `#[test]` attribute already added).
/// The format is `test_day!(fn_name -> tested_function(input), expected_output)`.
/// * `input` may be either a literal or an identifier.
/// * `expected_output` may be either a literal or an identifier.
#[macro_export]
macro_rules! test_day {
    ($name:ident -> $part:ident($input:literal), $output:literal) => {
        #[test]
        fn $name() {
            assert_eq!($part($input), $output);
        }
    };
    ($name:ident -> $part:ident($input:ident), $output:literal) => {
        #[test]
        fn $name() {
            assert_eq!($part($input), $output);
        }
    };
    ($name:ident -> $part:ident($input:literal), $output:ident) => {
        #[test]
        fn $name() {
            assert_eq!($part($input), $output);
        }
    };
    ($name:ident -> $part:ident($input:ident), $output:ident) => {
        #[test]
        fn $name() {
            assert_eq!($part($input), $output);
        }
    };
}
