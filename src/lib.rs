#![allow(dead_code)]
use nestify::nest;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop<Mode = Edit> {
        pub(crate) qop_mode: std::marker::PhantomData<Mode>,
        pub(crate) key_codes: Vec<KeyCode>,
        pub(crate) guts: Vec<
            pub(crate) struct Gut {
                pub(crate) gut_triggers: 
                    pub(crate) struct BtnTog {
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool,
                    },
                pub(crate) index_out: usize,
                pub(crate) extra_out: f64,
                pub(crate) i_mem: i64,
                pub(crate) x_mem: f64,
                pub(crate) trnsp_gut: Vec<
                    pub(crate) struct Trnsp<T, U> {
                        pub(crate) triggers: Vec<usize>,
                        pub(crate) i_delta: T,
                        pub(crate) x_delta: U,
                    } ||<i64, f64>
                >,
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
                    pub(crate) max_pressed: u8,
                    pub(crate) min_pressed: u8,
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

struct NewStuffPointers {

}

pub trait NewTrait: Default {
    fn new(n: &NewStuffPointers) -> Self;
}

impl NewTrait for Qop<Edit> {
    fn new(n: &NewStuffPointers) -> Qop<Edit> {
        Qop {
            guts: vec![Gut::new()],

            ..Default::default()
        }
    }
}

impl Qop<Edit> {
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