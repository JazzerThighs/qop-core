pub mod btns;
use nestify::nest;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]*
    pub struct QopEdit {
        pub(crate) key_codes: Vec<KeyCode>,
        pub(crate) plucks: Vec< pub(crate) struct Pluck {
            pub(crate) pluck: pub(crate) struct BtnTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
            },
            pub(crate) idx_out: usize,
            pub(crate) xtra_out: f64,
            pub(crate) trnsp_pluck: Vec<pub(crate) struct TrnspPluck {
                pub(crate) triggers: Vec<usize>,
                pub(crate) idx_delta: i64,
                pub(crate) xtra_delta: f64,
            }>,
            pub(crate) tp_i_mem: i64,
            pub(crate) tp_x_mem: f64,
        }>,
        pub(crate) plk_holds: pub(crate) struct HoldBtns {
            pub(crate) sustain: BtnTog,
            pub(crate) inv_sustain: BtnTog,
            pub(crate) sostenuto: BtnTog,
            pub(crate) inv_sostenuto: BtnTog,
        },
        pub(crate) valve_sets: Vec<pub(crate) struct IndvSet {
            pub(crate) buttons: Vec<pub(crate) struct DeltaTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
                pub(crate) idx_deltas: Vec<i64>,
                pub(crate) xtra_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<pub(crate) struct TrnspSet {
                    triggers: Vec<usize>,
                    idx_delta: Vec<i64>,
                    xtra_delta: Vec<f64>,
                }>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
            }>,
            pub(crate) max_pressed: u8,
            pub(crate) min_pressed: u8,
            pub(crate) holds: HoldBtns,
            pub(crate) trnsp_all: Vec<TrnspSet>,
            pub(crate) tp_i_mem: Vec<i64>,
            pub(crate) tp_x_mem: Vec<f64>,
        }>,
        pub(crate) fret_sets: Vec<IndvSet>,
        pub(crate) radio_sets: Vec<IndvSet>,
        pub(crate) aero_sets: Vec<pub(crate) struct ComboSet {
            pub(crate) buttons: Vec<BtnTog>,
            pub(crate) combos: Vec<pub(crate) struct Combo {
                pub(crate) combo: Vec<bool>,
                pub(crate) idx_deltas: Vec<i64>,
                pub(crate) xtra_deltas: Vec<f64>,
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

impl QopEdit {
    pub fn new() -> Self {
        return Self {
            key_codes: vec![],
            plucks: vec![Pluck::default()],
            plk_holds: HoldBtns::default(),
            valve_sets: vec![],
            fret_sets: vec![],
            radio_sets: vec![],
            aero_sets: vec![],
        };
    }
}

mod qe_kcs_methods;
mod qe_plk_methods;
mod qe_set_methods;
mod qe_trnsp_methods;

