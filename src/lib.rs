#![allow(dead_code)]
use nestify::nest;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

#[derive(Default)]

struct Edit;
struct Play;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop<Mode = Edit> {
        pub(crate) qop_mode: std::marker::PhantomData<Mode>,
        pub(crate) key_codes: Vec<KeyCode>,
        pub(crate) guts: Vec<pub(crate) struct Gut {
            pub(crate) gut_triggers: pub(crate) struct BtnTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
            },
            pub(crate) index_out: usize,
            pub(crate) extra_out: f64,
            pub(crate) i_mem: i64,
            pub(crate) x_mem: f64,
            pub(crate) trnsp_gut: Vec<pub(crate) struct OneTrnsp {
                pub(crate) triggers: Vec<usize>,
                pub(crate) i_delta: i64,
                pub(crate) x_delta: f64,
            }>,
            pub(crate) v_one: Vec<pub(crate) struct OneGutSet {
                pub(crate) buttons: Vec<pub(crate) struct OneGutIndv {
                    pub(crate) togs: Vec<usize>,
                    pub(crate) pressed: bool,
                    pub(crate) i_deltas: i64,
                    pub(crate) x_deltas: f64,
                    pub(crate) trnsp_one: Vec<OneTrnsp>,
                    pub(crate) i_mem: i64,
                    pub(crate) x_mem: f64,
                }>,
                pub(crate) trnsp_all: Vec<OneTrnsp>,
                pub(crate) i_mem: i64,
                pub(crate) x_mem: f64,
                pub(crate) holds: HoldBtns,
                pub(crate) max_pressed: u8,
                pub(crate) min_pressed: u8,
            }>,
            pub(crate) f_one: Vec<OneGutSet>,
            pub(crate) r_one: Vec<OneGutSet>,
            pub(crate) c_one: Vec<pub(crate) struct OneComboSet {
                pub(crate) buttons: Vec<BtnTog>,
                pub(crate) combos: Vec<pub(crate) struct OneGutCombo {
                    pub(crate) combo: Vec<bool>,
                    pub(crate) i_delta: i64,
                    pub(crate) x_delta: f64,
                    pub(crate) trnsp_one: Vec<MultiTrnsp>,
                    pub(crate) i_mem: Vec<i64>,
                    pub(crate) x_mem: Vec<f64>,
                }>,
            pub(crate) holds: HoldBtns,
            pub(crate) trnsp_all: Vec<MultiTrnsp>,
            pub(crate) i_mem: Vec<i64>,
            pub(crate) x_mem: Vec<f64>,
            }>
        }>,
        pub(crate) gut_holds: pub(crate) struct HoldBtns {
            pub(crate) sustain: BtnTog,
            pub(crate) inv_sustain: BtnTog,
            pub(crate) sostenuto: BtnTog,
            pub(crate) inv_sostenuto: BtnTog,
        },
        pub(crate) v_multi: Vec<pub(crate) struct MultiSet {
            pub(crate) buttons: Vec<pub(crate) struct MultiTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
                pub(crate) i_deltas: Vec<i64>,
                pub(crate) x_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<pub(crate) struct MultiTrnsp {
                    triggers: Vec<usize>,
                    i_deltas: Vec<i64>,
                    x_deltas: Vec<f64>,
                }>,
                pub(crate) i_mem: Vec<i64>,
                pub(crate) x_mem: Vec<f64>,
            }>,
            pub(crate) trnsp_all: Vec<MultiTrnsp>,
            pub(crate) i_mem: Vec<i64>,
            pub(crate) x_mem: Vec<f64>,
            pub(crate) holds: HoldBtns,
            pub(crate) max_pressed: u8,
            pub(crate) min_pressed: u8,
        }>,
        pub(crate) f_multi: Vec<MultiSet>,
        pub(crate) r_multi: Vec<MultiSet>,
        pub(crate) c_multi: Vec<pub(crate) struct MultiComboSet {
            pub(crate) buttons: Vec<BtnTog>,
            pub(crate) combos: Vec<pub(crate) struct Combo {
                pub(crate) combo: Vec<bool>,
                pub(crate) i_deltas: Vec<i64>,
                pub(crate) x_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<MultiTrnsp>,
                pub(crate) i_mem: Vec<i64>,
                pub(crate) x_mem: Vec<f64>,
            }>,
            pub(crate) holds: HoldBtns,
            pub(crate) trnsp_all: Vec<MultiTrnsp>,
            pub(crate) i_mem: Vec<i64>,
            pub(crate) x_mem: Vec<f64>,
        }>,
    }
}

impl Qop<Edit> {
    pub fn new() -> Qop<Edit> {
        Qop {
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