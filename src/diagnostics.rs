use std::io::stderr;
use std::io::Write;
use std::fmt;

use file_position::FilePosition;

pub fn print_error(message: fmt::Arguments) {
    writeln!(stderr(), "Error: {}", message).unwrap();
}

pub fn print_warning(message: fmt::Arguments) {
    writeln!(stderr(), "Warning: {}\n", message).unwrap();
}

pub fn print_fpos(fpos: &FilePosition) {
    write!(stderr(), "{}:{}:{}: ",
           fpos.filename, fpos.line, fpos.column).unwrap();
}

pub fn print_error_pos(message: fmt::Arguments, fpos: &FilePosition) {
    print_fpos(fpos);
    print_error(message);
}

pub fn print_warning_pos(message: fmt::Arguments, fpos: &FilePosition) {
    print_fpos(fpos);
    print_warning(message);
}
