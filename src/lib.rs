#![allow(dead_code)]

mod qopedit;
mod qopplay;

use std::marker::PhantomData;
use winit::keyboard::KeyCode;
use better_default::Default;
use serde::{Deserialize, Serialize};
use nestify::nest;

pub(crate) struct Edit;
pub(crate) struct Play;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop<Mode = Edit> {
        pub(crate) _qop_mode: PhantomData<Mode>,
        pub name: String,
        pub(crate) n:
            pub(crate) struct NewStuffPointers {
                #[default(1usize)]
                pub(crate) guts_len: usize,
                #[default(1usize)]
                pub(crate) c_btn_len: usize,
            },
        pub(crate) dig_inputs:  Vec<KeyCode>,
        pub(crate) gut_max_pressed: usize,
        pub(crate) gut_min_pressed: usize,
        pub(crate) gut_radio_mode: bool,
        pub(crate) gut_holds:
            pub(crate) struct HoldBtns {
                pub(crate) sustain: 
                    pub(crate) struct HoldTog {
                        pub name: String,
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
                >,
                pub(crate) v_one: Vec<
                    pub(crate) struct VFSet<T: Default, U: Default> {
                        pub name: String,
                        pub(crate) buttons: Vec<
                            pub(crate) struct VFBtn<T: Default, U: Default> {
                                pub name: String,
                                pub(crate) togs: Vec<usize>,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                                pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                pub(crate) i_mem: T,
                                pub(crate) x_mem: U,
                            } ||<T, U>
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                        pub(crate) i_mem: T,
                        pub(crate) x_mem: U,
                        pub(crate) holds: HoldBtns,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    } ||<i64, f64>
                >,
                pub(crate) f_one: Vec<VFSet<i64, f64>>,
                pub(crate) c_one: Vec<
                    pub(crate) struct ComboSet<T: Default, U: Default> {
                        pub name: String,
                        #[default(vec![ComboTog::default()])]
                        pub(crate) buttons: Vec<
                            pub(crate) struct ComboTog {
                                pub name: String,
                                togs: Vec<usize>,
                            }
                        >,
                        #[default(vec![false])]
                        pub(crate) pressed: Vec<bool>,
                        pub(crate) combos: Vec<
                            pub(crate) struct Combo<T: Default, U: Default> {
                                pub name: String,
                                pub(crate) combo: Vec<bool>,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                                pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                pub(crate) i_mem: T,
                                pub(crate) x_mem: U,
                            } ||<T, U>
                        >,
                        pub(crate) holds: HoldBtns,
                        pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                        pub(crate) i_mem: T,
                        pub(crate) x_mem: U,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    }||<i64, f64>
                >
            }
        >,
        pub(crate) v_multi: Vec<VFSet<Vec<i64>, Vec<f64>>>,
        pub(crate) f_multi: Vec<VFSet<Vec<i64>, Vec<f64>>>,
        pub(crate) c_multi: Vec<ComboSet<Vec<i64>, Vec<f64>>>,
    }
}

impl Qop<Edit> {
    pub fn to_play(&self) -> Qop<Play> {
        Qop {
            _qop_mode: PhantomData,
            name: self.name.clone(),
            n: self.n.clone(),
            dig_inputs: self.dig_inputs.clone(),
            gut_max_pressed: self.gut_max_pressed.clone(),
            gut_min_pressed: self.gut_min_pressed.clone(),
            gut_radio_mode: self.gut_radio_mode.clone(),
            gut_holds: self.gut_holds.clone(),
            guts: self.guts.clone(),
            v_multi: self.v_multi.clone(),
            f_multi: self.f_multi.clone(),
            c_multi: self.c_multi.clone(),
        }
    }
}
