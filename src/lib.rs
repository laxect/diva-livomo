#![feature(once_cell)]

pub mod annotation;
mod diff;
pub mod foliate;

pub use diff::{save, set_diff_flag};
