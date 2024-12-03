#![allow(dead_code)]
use nestify::nest;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;
use std::marker::PhantomData;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop<Mode = Edit> {
        pub(crate) qop_mode: PhantomData<Mode>,
        pub(crate) n: 
            pub(crate) struct NewStuffPointers {
                len_guts: usize,
                dig_vec: Vec<KeyCode>,
                v_one_indv_len: Vec<usize>,
                f_one_indv_len: Vec<usize>,
                r_one_indv_len: Vec<usize>,
                c_one_btns_len: Vec<usize>,
                c_one_comb_len: Vec<usize>,
                v_multi_len: usize,
                f_multi_len: usize,
                r_multi_len: usize,
                c_multi_len: usize,
                v_multi_indv_len: Vec<usize>,
                f_multi_indv_len: Vec<usize>,
                r_multi_indv_len: Vec<usize>,
                c_multi_btns_len: Vec<usize>,
                c_multi_comb_len: Vec<usize>,
            },
        pub(crate) dig_inputs:  Vec<KeyCode>,
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
                    pub(crate) struct Trnsp<T, U> {
                        pub(crate) triggers: Vec<usize>,
                        pub(crate) i_delta: T,
                        pub(crate) x_delta: U,
                    } ||<i64, f64>
                >,
                pub(crate) x_mem: f64,
                pub(crate) i_mem: i64,
                pub(crate) v_one: Vec<
                    pub(crate) struct VFRSet<T, U> {
                        pub(crate) buttons: Vec<
                            pub(crate) struct VFRIndv<T, U> {
                                pub(crate) togs: Vec<usize>,
                                pub(crate) pressed: bool,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                                pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                pub(crate) i_mem: T,
                                pub(crate) x_mem: U,
                            } ||<T, U>
                        >,
                        pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                        pub(crate) i_mem: T,
                        pub(crate) x_mem: U,
                        pub(crate) holds: HoldBtns,
                        pub(crate) max_pressed: usize,
                        pub(crate) min_pressed: usize,
                    } ||<i64, f64>
                >,
                pub(crate) f_one: Vec<VFRSet<i64, f64>>,
                pub(crate) r_one: Vec<VFRSet<i64, f64>>,
                pub(crate) c_one: Vec<
                    pub(crate) struct ComboSet<T,U> {
                        pub(crate) buttons: Vec<BtnTog>,
                        pub(crate) combos: Vec<
                            pub(crate) struct Combo<T, U> {
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
                        pub(crate) i_mem: i64,
                        pub(crate) x_mem: f64,
                    }||<i64, f64>>
            }
        >,
        pub(crate) gut_holds: 
            pub(crate) struct HoldBtns {
                pub(crate) sustain: BtnTog,
                pub(crate) inv_sustain: BtnTog,
                pub(crate) sostenuto: BtnTog,
                pub(crate) inv_sostenuto: BtnTog,
            },
        pub(crate) v_multi: Vec<VFRSet<Vec<i64>, Vec<f64>>>,
        pub(crate) f_multi: Vec<VFRSet<Vec<i64>, Vec<f64>>>,
        pub(crate) r_multi: Vec<VFRSet<Vec<i64>, Vec<f64>>>,
        pub(crate) c_multi: Vec<ComboSet<Vec<i64>, Vec<f64>>>,
    }
}

#[derive(Default)]

struct Edit;
struct Play;

impl NewStuffPointers {
    fn refresh_fields(&mut self, _q: &Qop) {
        todo!()
    }
}

impl Qop<Edit> {
    pub fn new() -> Qop<Edit> {
        let n: NewStuffPointers = NewStuffPointers {
            len_guts: 1usize,
            v_one_indv_len: vec![0usize],
            f_one_indv_len: vec![0usize],
            r_one_indv_len: vec![0usize],
            c_one_btns_len: vec![0usize],
            c_one_comb_len: vec![0usize],
            ..Default::default()
        };
        Qop {
            n,
            guts: vec![Gut::default()],
            ..Default::default()
        }
    }

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