//! Test runner for integration tests.

use clap::Parser;

/// Macro for running integration tests.
macro_rules! testit {
    ($test_name:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let file = stringify!($test_name).replace("__", "/");
            let file = format!("tests/{}.toml", file);
            let test = std::fs::read_to_string(&file)
                .unwrap()
                .parse::<toml::Table>()
                .unwrap();
            let mut args = test["cmd"]
                .as_array()
                .expect("TOML test definition is missing the cmd")
                .iter()
                .map(|v| v.as_str().expect("TOML test cmd must be an arrow of string").to_string())
                .collect::<Vec<_>>();
            let expected = test["out"]
                .as_str()
                .expect("TOML test definition is missing the expected output")
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            args.insert(0, "codump".to_string());

            let args = codump::CliArgs::try_parse_from(args).expect("Failed to parse args");
            let file = args.file.clone();
            let search_path = args.search_path.clone();
            let config = args.try_into().expect("Failed to parse config");
            let output =
                codump::execute(&file, &search_path, &config).expect("Failed to execute codump");

            assert_eq!(output, expected);
        }
    };
}

testit!(main);
