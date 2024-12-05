use duplicate::duplicate_item;

use crate::*;

impl Qop<Edit> {
    #[duplicate_item(
        multi_hold_insert_dig_method              field     setfunc;
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
        multi_hold_remove_dig_method              field     setfunc;
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
    SetType;
    [ VFRSet ];
    [ ComboSet ];
)]
impl<T, U> SetType<T, U> {
    pub(crate) fn sustain_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.sustain.togs.contains(&key_idx_val) {
            self.holds.sustain.togs.push(key_idx_val)
        }
    }
    pub(crate) fn inv_sustain_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
            self.holds.inv_sustain.togs.push(key_idx_val)
        }
    }
    pub(crate) fn sostenuto_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.sostenuto.togs.contains(&key_idx_val) {
            self.holds.sostenuto.togs.push(key_idx_val)
        }
    }
    pub(crate) fn inv_sostenuto_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
            self.holds.inv_sostenuto.togs.push(key_idx_val)
        }
    }
    pub(crate) fn sustain_remove_dig(&mut self, key_idx_val: usize) {
        self.holds.sustain.togs.retain(|&idx| idx != key_idx_val)
    }
    pub(crate) fn inv_sustain_remove_dig(&mut self, key_idx_val: usize) {
        self.holds
            .inv_sustain
            .togs
            .retain(|&idx| idx != key_idx_val)
    }
    pub(crate) fn sostenuto_remove_dig(&mut self, key_idx_val: usize) {
        self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val)
    }
    pub(crate) fn inv_sostenuto_remove_dig(&mut self, key_idx_val: usize) {
        self.holds
            .inv_sostenuto
            .togs
            .retain(|&idx| idx != key_idx_val)
    }
}
