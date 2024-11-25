use winit::keyboard::KeyCode;
use crate::qopedit::{QopEdit, ComboSet, HoldBtns, IndvSet, Gut};

#[repr(C)]
#[derive(Debug)]
pub struct QopPlay {
    key_codes: Vec<KeyCode>,
    plucks: Vec<Gut>,
    plk_holds: HoldBtns,
    valve_sets: Vec<IndvSet>,
    fret_sets: Vec<IndvSet>,
    radio_sets: Vec<IndvSet>,
    combo_sets: Vec<ComboSet>,
}

impl From<&QopEdit> for QopPlay {
    fn from(qe: &QopEdit) -> Self {
        return Self {
            key_codes: qe.key_codes.clone(),
            plucks: qe.guts.clone(),
            plk_holds: qe.gut_holds.clone(),
            valve_sets: qe.valve_sets.clone(),
            fret_sets: qe.fret_sets.clone(),
            radio_sets: qe.radio_sets.clone(),
            combo_sets: qe.combo_sets.clone(),
        };
    }
}