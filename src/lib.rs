#![allow(dead_code)]
mod engine;
mod temperament;

use better_default::Default;
use duplicate::duplicate_item;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};
use winit::keyboard::KeyCode;

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
