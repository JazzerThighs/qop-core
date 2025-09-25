mod _dig_inputs;
mod _gut;
mod _holds;
mod _trnsp;
mod _vfc_sets;

use crate::*;

impl<I: Int, F: Flo> Engine<I, F, Edit> where f32: From<F> {
    pub fn to_play(&self) -> Engine<I, F, Play> {
        Engine {
            _engine_mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            dig_inputs: self.dig_inputs.clone(),
            gut_max_pressed: self.gut_max_pressed.clone(),
            gut_min_pressed: self.gut_min_pressed.clone(),
            gut_radio_mode: self.gut_radio_mode.clone(),
            gut_holds: self.gut_holds.clone(),
            guts: self.guts.clone(),
            v_multi: self.v_multi.clone(),
            f_multi: self.f_multi.clone(),
            c_multi: self.c_multi.clone(),
            index_delta_bool: self.index_delta_bool.clone(),
            extra_delta_bool: self.extra_delta_bool.clone(),
        }
    }
}