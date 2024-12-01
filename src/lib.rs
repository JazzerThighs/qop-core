#![allow(dead_code)]
use nestify::nest;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop {
        pub(crate) qop_mode: pub(crate) enum QopMode {
            #[default]
            Edit,
            Play
        },
        pub(crate) key_codes: Vec<KeyCode>,
        pub(crate) guts: Vec<pub(crate) struct Gut {
            pub(crate) gut_triggers: pub(crate) struct BtnTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
            },
            pub(crate) index_out: usize,
            pub(crate) extra_out: f64,
            pub(crate) trnsp_gut: Vec<pub(crate) struct TrnspGut {
                pub(crate) triggers: Vec<usize>,
                pub(crate) i_delta: i64,
                pub(crate) x_delta: f64,
            }>,
            pub(crate) tp_i_mem: i64,
            pub(crate) tp_x_mem: f64,
        }>,
        pub(crate) gut_holds: pub(crate) struct HoldBtns {
            pub(crate) sustain: BtnTog,
            pub(crate) inv_sustain: BtnTog,
            pub(crate) sostenuto: BtnTog,
            pub(crate) inv_sostenuto: BtnTog,
        },
        pub(crate) valve_sets: Vec<pub(crate) struct IndvSet {
            pub(crate) buttons: Vec<pub(crate) struct DeltaTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
                pub(crate) i_deltas: Vec<i64>,
                pub(crate) x_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<pub(crate) struct TrnspSet {
                    triggers: Vec<usize>,
                    i_deltas: Vec<i64>,
                    x_deltas: Vec<f64>,
                }>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
            }>,
            pub(crate) trnsp_all: Vec<TrnspSet>,
            pub(crate) tp_i_mem: Vec<i64>,
            pub(crate) tp_x_mem: Vec<f64>,
            pub(crate) holds: HoldBtns,
            pub(crate) max_pressed: u8,
            pub(crate) min_pressed: u8,
        }>,
        pub(crate) fret_sets: Vec<IndvSet>,
        pub(crate) radio_sets: Vec<IndvSet>,
        pub(crate) combo_sets: Vec<pub(crate) struct ComboSet {
            pub(crate) buttons: Vec<BtnTog>,
            pub(crate) combos: Vec<pub(crate) struct Combo {
                pub(crate) combo: Vec<bool>,
                pub(crate) i_deltas: Vec<i64>,
                pub(crate) x_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<TrnspSet>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
            }>,
            pub(crate) holds: HoldBtns,
            pub(crate) trnsp_all: Vec<TrnspSet>,
            pub(crate) tp_i_mem: Vec<i64>,
            pub(crate) tp_x_mem: Vec<f64>,
        }>,
    }
}

impl Qop {
    pub fn new() -> Self {
        Self {
            guts: vec![Gut::default()],
            ..Default::default()
        }
    }
}

pub enum SetType {
    ValveSet,
    FretSet,
    RadioSet,
    ComboSet,
}

pub enum HoldType {
    Sustain,
    InvSustain,
    Sostenuto,
    InvSostenuto,
}

mod qopedit;
mod qopplay;