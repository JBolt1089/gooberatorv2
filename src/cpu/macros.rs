macro_rules! check_args {
    ($actual_args:expr, $expected_args:expr, $pc:expr) => {
        if $actual_args != $expected_args {
            eprintln!(
                "wrong number of arguments: expected {}, found {}. pc is {}",
                $expected_args, $actual_args, $pc
            );
            std::process::exit(69);
        }
    };
}

pub(crate) use check_args;
