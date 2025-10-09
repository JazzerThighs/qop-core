mod _edit;
mod _play;

use crate::*;
use better_default::Default;
use duplicate::duplicate_item;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData, ops::RangeInclusive};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub(crate) struct Engine<Mode = Edit> {
        pub(crate) _mode: PhantomData<Mode>,
        pub name: String,
        pub description: String,


        pub(crate) dig_inputs: Vec<KeyCode>,
        pub(crate) analog_inputs: Vec<RangeInclusive<usize>>,


        pub(crate) index_delta_bool: bool,
        pub(crate) extra_delta_bool: bool,
        pub(crate) gut_max_pressed: usize,
        pub(crate) gut_min_pressed: usize,
        pub(crate) gut_radio_mode: bool,
        pub(crate) gut_hold_mode: bool,
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
        pub(crate) gut_analog_mode: bool,
        pub(crate) gut_analogs: Vec<
            pub(crate) struct AnalogMod {
                pot: usize,
                i_mem: Vec<i32>,
                x_mem: Vec<f64>,
                i_min_in: i32,
                x_min_in: f64,
                i_max_in: i32,
                x_max_in: f64,
                i_min_out: Vec<i32>,
                x_min_out: Vec<f64>,
                i_max_out: Vec<i32>,
                x_max_out: Vec<f64>,
            }
        >,
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
    }
}

impl Engine<Edit> {
    pub fn to_play(&self) -> Result<Engine<Play>, String> {
        self.check_multi_delta_lengths()?;
        self.check_digitalref_invariants()?;

        Ok(Engine {
            _mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            dig_inputs: self.dig_inputs.clone(),
            gut_max_pressed: self.gut_max_pressed.clone(),
            gut_min_pressed: self.gut_min_pressed.clone(),
            gut_radio_mode: self.gut_radio_mode.clone(),
            gut_holds: self.gut_holds.clone(),
            guts: self.guts.clone(),
            v_multi: self.v_multi.clone(),
            f_multi: self.f_multi.clone(),
            c_multi: self.c_multi.clone(),
            index_delta_bool: self.index_delta_bool.clone(),
            extra_delta_bool: self.extra_delta_bool.clone(),
        })
    }
}
