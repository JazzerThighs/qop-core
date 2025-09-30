#![allow(dead_code)]
mod engine;
mod scale;

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

pub trait Gutted: Debug + Clone + Default + Serialize + for<'a> Deserialize<'a> {
    fn handle_engine_output(&self, qop: EngineOutput);
}

pub struct EngineOutput {
    index_out: Vec<usize>,
    extra_out: Vec<f64>,
}

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub struct Qop {
        pub(crate) name: String,
        pub(crate) description: String,
        pub(crate) engine:
            pub(crate) struct Engine<Mode = Edit> {
                pub(crate) _mode: PhantomData<Mode>,
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
                    pub(crate) struct Gut {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool,
                        pub(crate) index_out: usize,
                        pub(crate) extra_out: f64,
                        pub(crate) i_mem: i32,
                        pub(crate) x_mem: f64,
                        pub(crate) trnsp_gut: Vec<
                            pub(crate) struct Trnsp {
                                pub(crate) triggers: Vec<usize>,
                                pub(crate) i_delta: i32,
                                pub(crate) x_delta: f64,
                            }
                        >
                    }
                >,
                pub(crate) v_multi: Vec<
                    pub(crate) struct VFSet {
                        pub name: String,
                        pub description: String,
                        pub(crate) buttons: Vec<
                            pub(crate) struct VFBtn {
                                pub name: String,
                                pub description: String,
                                pub(crate) togs: Vec<usize>,
                                pub(crate) i_delta: Vec<i32>,
                                pub(crate) x_delta: Vec<f64>,
                                pub(crate) trnsp_one: Vec<
                                    pub(crate) struct MulTrnsp {
                                        pub(crate) triggers: Vec<usize>,
                                        pub(crate) i_delta: Vec<i32>,
                                        pub(crate) x_delta: Vec<f64>,
                                    }
                                >,
                                pub(crate) i_mem: Vec<i32>,
                                pub(crate) x_mem: Vec<f64>,
                            }
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) trnsp_all: Vec<MulTrnsp>,
                        pub(crate) i_mem: Vec<i32>,
                        pub(crate) x_mem: Vec<f64>,
                        pub(crate) holds: HoldBtns,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    }
                >,
                pub(crate) f_multi: Vec<VFSet>,
                pub(crate) c_multi: Vec<
                    pub(crate) struct ComboSet {
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
                            pub(crate) struct Combo {
                                pub name: String,
                                pub description: String,
                                pub(crate) combo: Vec<bool>,
                                pub(crate) i_delta: Vec<i32>,
                                pub(crate) x_delta: Vec<f64>,
                                pub(crate) trnsp_one: Vec<MulTrnsp>,
                                pub(crate) i_mem: Vec<i32>,
                                pub(crate) x_mem: Vec<f64>,
                            }
                        >,
                        pub(crate) holds: HoldBtns,
                        pub(crate) trnsp_all: Vec<MulTrnsp>,
                        pub(crate) i_mem: Vec<i32>,
                        pub(crate) x_mem: Vec<f64>,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    }
                >,
            },
    }
}

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub(crate) struct Temperament<Mode = Edit> {
        pub(crate) _mode: PhantomData<Mode>,
        pub name: String,
        pub description: String,
        scale_type: 
            pub enum TemperamentType {
                #[default(0: 12.0f64, 1: 2.0f64)]
                EqualTemperament(f64, f64),
                PrimeLimit(usize),
                Arbitrary,
            },
        #[default(69usize)]
        pub(crate) reference_note: usize,
        #[default(440.0f64)]
        pub(crate) tuning_hz: f64,
        #[default(4i64)]
        octave_label: i64,
        #[default(2.0f64)]
        pub(crate) octave_scalar_factor: f64,
        #[default(["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"].iter().map(|i: &&str| i.to_string()).collect::<Vec<String>>())]
        pub(crate) note_class_set: Vec<String>,
        #[default(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0])]
        pub(crate) intervals: Vec<f64>,
    }
}

impl Qop  {
    pub fn new() -> Self {
        Self::default()
    }
}

#[duplicate_item(
    change_string_param         new_string_param         string_param;
    [change_qop_name]           [new_qop_name]           [name];
    [change_qop_description]    [new_qop_description]    [description];
    [change_engine_name]        [new_engine_name]        [engine.name];
    [change_engine_description] [new_engine_description] [engine.description];
)]
impl Qop {
    pub fn change_string_param(&mut self, new_string_param: String) {
        self.string_param = new_string_param;
    }
}
