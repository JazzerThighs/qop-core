use crate::{Play, engine::Engine};

mod _dig_inputs;
mod _gut;
mod _holds;
mod _trnsp;
mod _vfc_sets;

impl Engine<Play> {
    pub fn play<D: Into<usize>>(&mut self, dig_down: Vec<D>, dig_up: Vec<D>, analogs: Vec<(D, D)>) {
        todo!()

        // todo: alter the dig_inputs
        // todo: alter the analog_inputs

        
    }
}

impl Engine<Play> {
    fn sustain_gate(button: bool, hold: bool) -> bool {
        todo!()
    }

    fn inv_sustain_gate(button: bool, hold: bool) -> bool {
        todo!()
    }

    fn sostenuto_gate(button: bool, hold: bool, already_pressed: bool) -> (bool, bool) {
        todo!()
    }

    fn inv_sostenuto_gate(button: bool, hold: bool) -> bool {
        todo!()
    }
}