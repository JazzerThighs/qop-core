pub mod btns;
use btns::{ComboSet, HoldBtns, IndvSet, Pluck};
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QopEdit {
    pub(super) key_codes: Vec<KeyCode>,
    pub(super) plucks: Vec<Pluck>,
    pub(super) plk_holds: HoldBtns,
    pub(super) valve_sets: Vec<IndvSet>,
    pub(super) fret_sets: Vec<IndvSet>,
    pub(super) radio_sets: Vec<IndvSet>,
    pub(super) aero_sets: Vec<ComboSet>,
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

