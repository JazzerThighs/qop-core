#![allow(dead_code)]
mod engine;
mod scale;

use std::marker::PhantomData;
use winit::keyboard::KeyCode;
use better_default::Default;
use serde::Serialize;
use nestify::nest;
use duplicate::duplicate_item;
use num_traits::{float::Float, sign::Signed, int::PrimInt};

use scale::NewScaleParams;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct Edit;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct Play;

pub trait Int: PrimInt + Signed + Clone + Default {}
impl<I: PrimInt + Signed + Clone + Default> Int for I {}
pub trait Flo: Float + Clone + Default {}
impl<F: Float + Clone + Default> Flo for F {}

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize)]*
    #[derive(Default)]*
    pub struct Qop<I: Int, F: Flo> {
        pub(crate) name: String,
        pub(crate) description: String,
        pub(crate) engine:
            pub(crate) struct Engine<I: Int, F: Flo, Mode = Edit> {
                pub(crate) _engine_mode: PhantomData<Mode>,
                pub name: String,
                pub description: String,
                pub(crate) dig_inputs: Vec<KeyCode>,
                pub(crate) index_delta_bool: bool,
                pub(crate) extra_delta_bool: bool,
                pub(crate) gut_max_pressed: usize,
                pub(crate) gut_min_pressed: usize,
                pub(crate) gut_radio_mode: bool,
                pub(crate) gut_holds:
                    pub(crate) struct HoldBtns {
                        pub(crate) sustain: 
                            pub(crate) struct HoldTog {
                                pub name: String,
                                pub description: String,
                                pub(crate) togs: Vec<usize>,
                                pub(crate) pressed: bool
                            },
                        pub(crate) inv_sustain: HoldTog,
                        pub(crate) sostenuto: HoldTog,
                        pub(crate) inv_sostenuto: HoldTog,
                    },
                #[default(vec![Gut::default()])]
                pub(crate) guts: Vec<
                    pub(crate) struct Gut<I: Int, F: Flo> {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool,
                        pub(crate) index_out: usize,
                        pub(crate) extra_out: F,
                        pub(crate) i_mem: I,
                        pub(crate) x_mem: F,
                        pub(crate) trnsp_gut: Vec<
                            pub(crate) struct Trnsp<I: Int, F: Flo> {
                                pub(crate) triggers: Vec<usize>,
                                pub(crate) i_delta: I,
                                pub(crate) x_delta: F,
                            } ||<I, F>
                        >
                    } ||<I, F>
                >,
                pub(crate) v_multi: Vec<
                    pub(crate) struct VFSet<I: Int, F: Flo> {
                        pub name: String,
                        pub description: String,
                        pub(crate) buttons: Vec<
                            pub(crate) struct VFBtn<I: Int, F: Flo> {
                                pub name: String,
                                pub description: String,
                                pub(crate) togs: Vec<usize>,
                                pub(crate) i_delta: Vec<I>,
                                pub(crate) x_delta: Vec<F>,
                                pub(crate) trnsp_one: Vec<
                                    pub(crate) struct MulTrnsp<I: Int, F: Flo> {
                                        pub(crate) triggers: Vec<usize>,
                                        pub(crate) i_delta: Vec<I>,
                                        pub(crate) x_delta: Vec<F>,
                                    } ||<I, F>
                                >,
                                pub(crate) i_mem: Vec<I>,
                                pub(crate) x_mem: Vec<F>,
                            } ||<I, F>
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) trnsp_all: Vec<MulTrnsp<I, F>>,
                        pub(crate) i_mem: Vec<I>,
                        pub(crate) x_mem: Vec<F>,
                        pub(crate) holds: HoldBtns,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    } ||<I, F>
                >,
                pub(crate) f_multi: Vec<VFSet<I, F>>,
                pub(crate) c_multi: Vec<
                    pub(crate) struct ComboSet<I: Int, F: Flo> {
                        pub name: String,
                        pub description: String,
                        #[default(vec![ComboTog::default()])]
                        pub(crate) buttons: Vec<
                            pub(crate) struct ComboTog {
                                pub name: String,
                                pub description: String,
                                pub(crate) togs: Vec<usize>,
                            }
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) combos: Vec<
                            pub(crate) struct Combo<I: Int, F: Flo> {
                                pub name: String,
                                pub description: String,
                                pub(crate) combo: Vec<bool>,
                                pub(crate) i_delta: Vec<I>,
                                pub(crate) x_delta: Vec<F>,
                                pub(crate) trnsp_one: Vec<MulTrnsp<I, F>>,
                                pub(crate) i_mem: Vec<I>,
                                pub(crate) x_mem: Vec<F>,
                            } ||<I, F>
                        >,
                        pub(crate) holds: HoldBtns,
                        pub(crate) trnsp_all: Vec<MulTrnsp<I, F>>,
                        pub(crate) i_mem: Vec<I>,
                        pub(crate) x_mem: Vec<F>,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    } ||<I, F>
                >,
            } ||<I, F>,
        #[default(Scale::new(NewScaleParams::default()))]
        pub(crate) scale:
            pub(crate) struct Scale<Mode = Edit> {
                pub(crate) _scale_mode: PhantomData<Mode>,
                pub name: String,
                pub description: String,
                pub(crate) reference_note: usize,
                pub(crate) tuning_hz: f64,
                pub(crate) note_class_set: Vec<String>,
                pub(crate) notes: Vec<
                    pub(crate) struct Note {
                        pub name: String,
                        pub description: String,
                        pub(crate) note_num: usize,
                        pub color: String,
                        pub(crate) frequency: f64,
                    }
                >,
            },
    }
}

impl<I: Int, F: Flo> Qop<I, F> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[duplicate_item(
    change_string_param         new_string_param         string_param;
    [change_qop_name]           [new_qop_name]           [name];
    [change_qop_description]    [new_qop_description]    [description];
    [change_scale_name]         [new_scale_name]         [scale.name];
    [change_scale_description]  [new_scale_description]  [scale.description];
    [change_engine_name]        [new_engine_name]        [engine.name];
    [change_engine_description] [new_engine_description] [engine.description];
)]
impl<I: Int, F: Flo> Qop<I, F> {   
    pub fn change_string_param(&mut self, new_string_param: String) {
        self.string_param = new_string_param;
    }
}