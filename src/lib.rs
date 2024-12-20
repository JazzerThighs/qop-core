#![allow(dead_code)]
mod engine;
mod scale;
mod intervals;

use std::marker::PhantomData;
use winit::keyboard::KeyCode;
use better_default::Default;
use serde::{Deserialize, Serialize};
use nestify::nest;
use duplicate::duplicate_item;
use scale::NewScaleParams;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct Edit;
#[repr(C)]
pub(crate) struct Play;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop {
        pub(crate) name: String,
        pub(crate) description: String,
        pub(crate) engine:
            pub(crate) struct Engine<Mode = Edit> {
                pub(crate) _engine_mode: PhantomData<Mode>,
                pub name: String,
                pub description: String,
                pub(crate) dig_inputs:  Vec<KeyCode>,
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
                                togs: Vec<usize>,
                                pressed: bool
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
                        pub(crate) i_mem: i64,
                        pub(crate) x_mem: f64,
                        pub(crate) trnsp_gut: Vec<
                            pub(crate) struct Trnsp<T: Default, U: Default> {
                                pub(crate) triggers: Vec<usize>,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                            } ||<i64, f64>
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
                                pub(crate) i_delta: Vec<i64>,
                                pub(crate) x_delta: Vec<f64>,
                                pub(crate) trnsp_one: Vec<Trnsp<Vec<i64>, Vec<f64>>>,
                                pub(crate) i_mem: Vec<i64>,
                                pub(crate) x_mem: Vec<f64>,
                            }
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) trnsp_all: Vec<Trnsp<Vec<i64>, Vec<f64>>>,
                        pub(crate) i_mem: Vec<i64>,
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
                                togs: Vec<usize>,
                            }
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) combos: Vec<
                            pub(crate) struct Combo {
                                pub name: String,
                                pub description: String,
                                pub(crate) combo: Vec<bool>,
                                pub(crate) i_delta: Vec<i64>,
                                pub(crate) x_delta: Vec<f64>,
                                pub(crate) trnsp_one: Vec<Trnsp<Vec<i64>, Vec<f64>>>,
                                pub(crate) i_mem: Vec<i64>,
                                pub(crate) x_mem: Vec<f64>,
                            }
                        >,
                        pub(crate) holds: HoldBtns,
                        pub(crate) trnsp_all: Vec<Trnsp<Vec<i64>, Vec<f64>>>,
                        pub(crate) i_mem: Vec<i64>,
                        pub(crate) x_mem: Vec<f64>,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    }
                >,
            },
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

#[duplicate_item(
    change_string_param         new_string_param         string_param;
    [change_qop_name]           [new_qop_name]           [name];
    [change_qop_description]    [new_qop_description]    [description];
    [change_scale_name]         [new_scale_name]         [scale.name];
    [change_scale_description]  [new_scale_description]  [scale.description];
    [change_engine_name]        [new_engine_name]        [engine.name];
    [change_engine_description] [new_engine_description] [engine.description];
    
)]
impl Qop {   
    pub fn change_string_param(&mut self, new_string_param: String) {
        self.string_param = new_string_param;
    }
}