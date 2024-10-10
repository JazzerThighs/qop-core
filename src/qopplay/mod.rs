use winit::keyboard::KeyCode;
use crate::{qopedit::btns::{ComboSet, HoldBtns, IndvSet, Pluck}, QopEdit};

#[repr(C)]
#[derive(Debug)]
pub struct QopPlay {
    key_codes: Vec<KeyCode>,
    plucks: Vec<Pluck>,
    plk_holds: HoldBtns,
    valve_sets: Vec<IndvSet>,
    fret_sets: Vec<IndvSet>,
    radio_sets: Vec<IndvSet>,
    aero_sets: Vec<ComboSet>,
}

impl From<&QopEdit> for QopPlay {
    fn from(qe: &QopEdit) -> Self {
        return Self {
            key_codes: qe.key_codes.clone(),
            plucks: qe.plucks.clone(),
            plk_holds: qe.plk_holds.clone(),
            valve_sets: qe.valve_sets.clone(),
            fret_sets: qe.fret_sets.clone(),
            radio_sets: qe.radio_sets.clone(),
            aero_sets: qe.aero_sets.clone(),
        };
    }
}