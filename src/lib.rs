#![feature(once_cell)]

pub mod annotation;
mod diff;
pub mod foliate;
pub mod options;

pub use diff::{save, set_diff_flag};
