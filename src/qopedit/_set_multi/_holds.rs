use duplicate::duplicate_item;

use crate::*;

impl Qop<Edit> {
    #[duplicate_item(
        multi_hold_insert_dig_method        field     setfunc;
        [v_multi_sustain_insert_dig]        [v_multi] [sustain_insert_dig];
        [v_multi_inv_sustain_insert_dig]    [v_multi] [inv_sustain_insert_dig];
        [v_multi_sostenuto_insert_dig]      [v_multi] [sostenuto_insert_dig];
        [v_multi_inv_sostenuto_insert_dig]  [v_multi] [inv_sostenuto_insert_dig];

        [f_multi_sustain_insert_dig]        [f_multi] [sustain_insert_dig];
        [f_multi_inv_sustain_insert_dig]    [f_multi] [inv_sustain_insert_dig];
        [f_multi_sostenuto_insert_dig]      [f_multi] [sostenuto_insert_dig];
        [f_multi_inv_sostenuto_insert_dig]  [f_multi] [inv_sostenuto_insert_dig];

        [r_multi_sustain_insert_dig]        [r_multi] [sustain_insert_dig];
        [r_multi_inv_sustain_insert_dig]    [r_multi] [inv_sustain_insert_dig];
        [r_multi_sostenuto_insert_dig]      [r_multi] [sostenuto_insert_dig];
        [r_multi_inv_sostenuto_insert_dig]  [r_multi] [inv_sostenuto_insert_dig];

        [c_multi_sustain_insert_dig]        [c_multi] [sustain_insert_dig];
        [c_multi_inv_sustain_insert_dig]    [c_multi] [inv_sustain_insert_dig];
        [c_multi_sostenuto_insert_dig]      [c_multi] [sostenuto_insert_dig];
        [c_multi_inv_sostenuto_insert_dig]  [c_multi] [inv_sostenuto_insert_dig];
    )]
    pub fn multi_hold_insert_dig_method(&mut self, set_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() && set_idx < self.field.len() {
            self.field[set_idx].setfunc(key_idx_val)
        }
    }

    #[duplicate_item(
        multi_hold_remove_dig_method        field     setfunc;
        [v_multi_sustain_remove_dig]        [v_multi] [sustain_remove_dig];
        [v_multi_inv_sustain_remove_dig]    [v_multi] [inv_sustain_remove_dig];
        [v_multi_sostenuto_remove_dig]      [v_multi] [sostenuto_remove_dig];
        [v_multi_inv_sostenuto_remove_dig]  [v_multi] [inv_sostenuto_remove_dig];

        [f_multi_sustain_remove_dig]        [f_multi] [sustain_remove_dig];
        [f_multi_inv_sustain_remove_dig]    [f_multi] [inv_sustain_remove_dig];
        [f_multi_sostenuto_remove_dig]      [f_multi] [sostenuto_remove_dig];
        [f_multi_inv_sostenuto_remove_dig]  [f_multi] [inv_sostenuto_remove_dig];

        [r_multi_sustain_remove_dig]        [r_multi] [sustain_remove_dig];
        [r_multi_inv_sustain_remove_dig]    [r_multi] [inv_sustain_remove_dig];
        [r_multi_sostenuto_remove_dig]      [r_multi] [sostenuto_remove_dig];
        [r_multi_inv_sostenuto_remove_dig]  [r_multi] [inv_sostenuto_remove_dig];

        [c_multi_sustain_remove_dig]        [c_multi] [sustain_remove_dig];
        [c_multi_inv_sustain_remove_dig]    [c_multi] [inv_sustain_remove_dig];
        [c_multi_sostenuto_remove_dig]      [c_multi] [sostenuto_remove_dig];
        [c_multi_inv_sostenuto_remove_dig]  [c_multi] [inv_sostenuto_remove_dig];
    )]
    pub fn multi_hold_remove_dig_method(&mut self, set_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() && set_idx < self.field.len() {
            self.field[set_idx].setfunc(key_idx_val)
        }
    }
}

#[duplicate_item(
    SetType     set_hold_insert_dig_method  set_hold_remove_dig_method  field;
    [VFRSet]    [sustain_insert_dig]        [sustain_remove_dig]        [sustain];
    [VFRSet]    [inv_sustain_insert_dig]    [inv_sustain_remove_dig]    [inv_sustain];
    [VFRSet]    [sostenuto_insert_dig]      [sostenuto_remove_dig]      [sostenuto];
    [VFRSet]    [inv_sostenuto_insert_dig]  [inv_sostenuto_remove_dig]  [inv_sostenuto];
    
    [ComboSet]  [sustain_insert_dig]        [sustain_remove_dig]        [sustain];
    [ComboSet]  [inv_sustain_insert_dig]    [inv_sustain_remove_dig]    [inv_sustain];
    [ComboSet]  [sostenuto_insert_dig]      [sostenuto_remove_dig]      [sostenuto];
    [ComboSet]  [inv_sostenuto_insert_dig]  [inv_sostenuto_remove_dig]  [inv_sostenuto];
)]
impl<T, U> SetType<T, U> {
    pub(crate) fn set_hold_insert_dig_method(&mut self, key_idx_val: usize) {
        if !self.holds.field.togs.contains(&key_idx_val) {
            self.holds.field.togs.push(key_idx_val)
        }
    }
    pub(crate) fn set_hold_remove_dig_method(&mut self, key_idx_val: usize) {
        self.holds.field.togs.retain(|&idx| idx != key_idx_val)
    }
}
