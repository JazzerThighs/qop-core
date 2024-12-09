#![allow(dead_code)]
use std::marker::PhantomData;
use winit::keyboard::KeyCode;
use better_default::Default;
use serde::{Deserialize, Serialize};
use nestify::nest;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop<Mode = Edit> {
        pub(crate) qop_mode: PhantomData<Mode>,
        #[serde(skip_serializing)]
        pub(crate) n:
            pub(crate) struct NewStuffPointers {
                #[default(1)]
                pub(crate) guts_len: usize,
                #[default(1)]
                pub(crate) c_btn_len: usize,
            },
        pub(crate) dig_inputs:  Vec<KeyCode>,
        #[default(vec![Gut::default()])]
        pub(crate) guts: Vec<
            pub(crate) struct Gut {
                pub(crate) gut_triggers:
                    pub(crate) struct BtnTog {
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool,
                    },
                pub(crate) index_out: usize,
                pub(crate) extra_out: f64,
                pub(crate) trnsp_gut: Vec<
                    pub(crate) struct Trnsp<T: Default, U: Default> {
                        pub(crate) triggers: Vec<usize>,
                        pub(crate) i_delta: T,
                        pub(crate) x_delta: U,
                    } ||<i64, f64>
                >,
                #[serde(skip_serializing)]
                pub(crate) i_mem: i64,
                #[serde(skip_serializing)]
                pub(crate) x_mem: f64,
                pub(crate) v_one: Vec<
                    pub(crate) struct VFSet<T: Default, U: Default> {
                        pub(crate) buttons: Vec<
                            pub(crate) struct VFBtn<T: Default, U: Default> {
                                pub(crate) togs: Vec<usize>,
                                pub(crate) pressed: bool,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                                pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                #[serde(skip_serializing)]
                                pub(crate) i_mem: T,
                                #[serde(skip_serializing)]
                                pub(crate) x_mem: U,
                            } ||<T, U>
                        >,
                        pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                        #[serde(skip_serializing)]
                        pub(crate) i_mem: T,
                        #[serde(skip_serializing)]
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
                        #[default(vec![BtnTog::default()])]
                        pub(crate) buttons: Vec<BtnTog>,
                        pub(crate) combos: Vec<
                            pub(crate) struct Combo<T: Default, U: Default> {
                                pub(crate) combo: Vec<bool>,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                                pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                #[serde(skip_serializing)]
                                pub(crate) i_mem: T,
                                #[serde(skip_serializing)]
                                pub(crate) x_mem: U,
                            } ||<T, U>
                        >,
                        pub(crate) holds: HoldBtns,
                        pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                        #[serde(skip_serializing)]
                        pub(crate) i_mem: T,
                        #[serde(skip_serializing)]
                        pub(crate) x_mem: U,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                        pub(crate) radio_mode: bool,
                    }||<i64, f64>
                >
            }
        >,
        pub(crate) gut_max_pressed: usize,
        pub(crate) gut_min_pressed: usize,
        pub(crate) gut_radio_mode: bool,
        pub(crate) gut_holds:
            pub(crate) struct HoldBtns {
                pub(crate) sustain: BtnTog,
                pub(crate) inv_sustain: BtnTog,
                pub(crate) sostenuto: BtnTog,
                pub(crate) inv_sostenuto: BtnTog,
            },
        pub(crate) v_multi: Vec<VFSet<Vec<i64>, Vec<f64>>>,
        pub(crate) f_multi: Vec<VFSet<Vec<i64>, Vec<f64>>>,
        pub(crate) c_multi: Vec<ComboSet<Vec<i64>, Vec<f64>>>,
    }
}

#[derive(Default)]
pub struct Edit;
struct Play;

impl Qop<Edit> {
    #[allow(private_interfaces)]
    pub fn to_play(&self) -> Qop<Play> {
        todo!()
        // Qop {
        //     qop_mode: std::marker::PhantomData,
        //     ..other fields filled in
        // }
    }
}

mod qopedit;
mod qopplay;
