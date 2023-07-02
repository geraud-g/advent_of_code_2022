use std::fs;

#[cfg(windows)]
pub const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";

pub fn get_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Unable to read file")
}

#[macro_export]
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
