use crate::*;
use duplicate::duplicate_item;

#[duplicate_item(
    holdfield       gut_hold_insert_dig             gut_hold_remove_dig;
    [sustain]       [gut_sustain_insert_dig]        [gut_sustain_remove_dig];
    [inv_sustain]   [gut_inv_sustain_insert_dig]    [gut_inv_sustain_remove_dig];
    [sostenuto]     [gut_sostenuto_insert_dig]      [gut_sostenuto_remove_dig];
    [inv_sostenuto] [gut_inv_sostenuto_insert_dig]  [gut_inv_sostenuto_remove_dig];
)]
impl Qop<Edit> {
    pub fn gut_hold_insert_dig(&mut self, key_idx_val: usize) {
        if !self.gut_holds.holdfield.togs.contains(&key_idx_val) {
            self.gut_holds.holdfield.togs.push(key_idx_val)
        }
    }
    pub fn gut_hold_remove_dig(&mut self, key_idx_val: usize) {
        self.gut_holds
            .holdfield
            .togs
            .retain(|&idx| idx != key_idx_val)
    }
}

#[duplicate_item(
    onefield    multifield  one_hold_insert_dig                 one_hold_remove_dig                 multi_hold_insert_dig               multi_hold_remove_dig               hold_insert_dig               hold_remove_dig;
    [v_one]     [v_multi]   [v_one_sustain_insert_dig]          [v_one_sustain_remove_dig]          [v_multi_sustain_insert_dig]        [v_multi_sustain_remove_dig]        [sustain_insert_dig]          [sustain_remove_dig];
    [v_one]     [v_multi]   [v_one_inv_sustain_insert_dig]      [v_one_inv_sustain_remove_dig]      [v_multi_inv_sustain_insert_dig]    [v_multi_inv_sustain_remove_dig]    [inv_sustain_insert_dig]      [inv_sustain_remove_dig];
    [v_one]     [v_multi]   [v_one_sostenuto_insert_dig]        [v_one_sostenuto_remove_dig]        [v_multi_sostenuto_insert_dig]      [v_multi_sostenuto_remove_dig]      [sostenuto_insert_dig]        [sostenuto_remove_dig];
    [v_one]     [v_multi]   [v_one_inv_sostenuto_insert_dig]    [v_one_inv_sostenuto_remove_dig]    [v_multi_inv_sostenuto_insert_dig]  [v_multi_inv_sostenuto_remove_dig]  [inv_sostenuto_insert_dig]    [inv_sostenuto_remove_dig];
    [f_one]     [f_multi]   [f_one_sustain_insert_dig]          [f_one_sustain_remove_dig]          [f_multi_sustain_insert_dig]        [f_multi_sustain_remove_dig]        [sustain_insert_dig]          [sustain_remove_dig];
    [f_one]     [f_multi]   [f_one_inv_sustain_insert_dig]      [f_one_inv_sustain_remove_dig]      [f_multi_inv_sustain_insert_dig]    [f_multi_inv_sustain_remove_dig]    [inv_sustain_insert_dig]      [inv_sustain_remove_dig];
    [f_one]     [f_multi]   [f_one_sostenuto_insert_dig]        [f_one_sostenuto_remove_dig]        [f_multi_sostenuto_insert_dig]      [f_multi_sostenuto_remove_dig]      [sostenuto_insert_dig]        [sostenuto_remove_dig];
    [f_one]     [f_multi]   [f_one_inv_sostenuto_insert_dig]    [f_one_inv_sostenuto_remove_dig]    [f_multi_inv_sostenuto_insert_dig]  [f_multi_inv_sostenuto_remove_dig]  [inv_sostenuto_insert_dig]    [inv_sostenuto_remove_dig];
    [r_one]     [r_multi]   [r_one_sustain_insert_dig]          [r_one_sustain_remove_dig]          [r_multi_sustain_insert_dig]        [r_multi_sustain_remove_dig]        [sustain_insert_dig]          [sustain_remove_dig];
    [r_one]     [r_multi]   [r_one_inv_sustain_insert_dig]      [r_one_inv_sustain_remove_dig]      [r_multi_inv_sustain_insert_dig]    [r_multi_inv_sustain_remove_dig]    [inv_sustain_insert_dig]      [inv_sustain_remove_dig];
    [r_one]     [r_multi]   [r_one_sostenuto_insert_dig]        [r_one_sostenuto_remove_dig]        [r_multi_sostenuto_insert_dig]      [r_multi_sostenuto_remove_dig]      [sostenuto_insert_dig]        [sostenuto_remove_dig];
    [r_one]     [r_multi]   [r_one_inv_sostenuto_insert_dig]    [r_one_inv_sostenuto_remove_dig]    [r_multi_inv_sostenuto_insert_dig]  [r_multi_inv_sostenuto_remove_dig]  [inv_sostenuto_insert_dig]    [inv_sostenuto_remove_dig];
    [c_one]     [c_multi]   [c_one_sustain_insert_dig]          [c_one_sustain_remove_dig]          [c_multi_sustain_insert_dig]        [c_multi_sustain_remove_dig]        [sustain_insert_dig]          [sustain_remove_dig];
    [c_one]     [c_multi]   [c_one_inv_sustain_insert_dig]      [c_one_inv_sustain_remove_dig]      [c_multi_inv_sustain_insert_dig]    [c_multi_inv_sustain_remove_dig]    [inv_sustain_insert_dig]      [inv_sustain_remove_dig];
    [c_one]     [c_multi]   [c_one_sostenuto_insert_dig]        [c_one_sostenuto_remove_dig]        [c_multi_sostenuto_insert_dig]      [c_multi_sostenuto_remove_dig]      [sostenuto_insert_dig]        [sostenuto_remove_dig];
    [c_one]     [c_multi]   [c_one_inv_sostenuto_insert_dig]    [c_one_inv_sostenuto_remove_dig]    [c_multi_inv_sostenuto_insert_dig]  [c_multi_inv_sostenuto_remove_dig]  [inv_sostenuto_insert_dig]    [inv_sostenuto_remove_dig];
)]
impl Qop<Edit> {
    pub fn one_hold_insert_dig(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
        {
            self.guts[g_idx].onefield[set_idx].hold_insert_dig(key_idx_val)
        }
    }
    pub fn one_hold_remove_dig(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
        {
            self.guts[g_idx].onefield[set_idx].hold_remove_dig(key_idx_val)
        }
    }
    pub fn multi_hold_insert_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len() && set_idx < self.multifield.len() {
            self.multifield[set_idx].hold_insert_dig(key_idx_val)
        }
    }
    pub fn multi_hold_remove_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len() && set_idx < self.multifield.len() {
            self.multifield[set_idx].hold_remove_dig(key_idx_val)
        }
    }
}

#[duplicate_item(
    SetType     multi_hold_insert_dig       multi_hold_remove_dig       holdfield;
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
    pub(crate) fn multi_hold_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.holdfield.togs.contains(&key_idx_val) {
            self.holds.holdfield.togs.push(key_idx_val)
        }
    }
    pub(crate) fn multi_hold_remove_dig(&mut self, key_idx_val: usize) {
        self.holds.holdfield.togs.retain(|&idx| idx != key_idx_val)
    }
}
