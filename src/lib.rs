#![feature(const_for)]

pub mod multiwheel;
mod timer;

pub use timer::{Agent, SliceSmallVectorTimer, SliceVectorTimer, VectorVectorTimer};
