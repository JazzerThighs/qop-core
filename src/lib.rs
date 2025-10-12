#![allow(dead_code)]
mod engine;
mod temperament;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct Edit;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct Play;

pub trait QopPart<T>: Debug + Clone + Default + Serialize + for<'a> Deserialize<'a> {
    fn handle_engine_output(&self, qop: (usize, f64)) -> T;

    fn output_to_engine(&self) -> (usize, f64);
}
