#![allow(dead_code)]

use colored::Colorize;
use std::fmt::Display;

#[inline]
pub fn error(msg: impl Display) {
    eprintln!("{}: {msg}", "error".bold().bright_red())
}

#[inline]
pub fn warning(msg: impl Display) {
    eprintln!("{}: {msg}", "warning".bold().bright_yellow())
}

#[inline]
pub fn info(msg: impl Display) {
    println!("{}: {msg}", "info".bold().bright_green());
}