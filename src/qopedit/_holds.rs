use crate::*;
use duplicate::duplicate_item;

#[duplicate_item(
    holdfield       gut_hold_insert_dig            gut_hold_remove_dig;
    [sustain]       [gut_sustain_insert_dig]       [gut_sustain_remove_dig];
    [inv_sustain]   [gut_inv_sustain_insert_dig]   [gut_inv_sustain_remove_dig];
    [sostenuto]     [gut_sostenuto_insert_dig]     [gut_sostenuto_remove_dig];
    [inv_sostenuto] [gut_inv_sostenuto_insert_dig] [gut_inv_sostenuto_remove_dig];
)]
impl Engine<Edit> {
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
    onefield multifield holdfield       one_hold_insert_dig              one_hold_remove_dig              multi_hold_insert_dig              multi_hold_remove_dig;
    [v_one]  [v_multi]  [sustain]       [v_one_sustain_insert_dig]       [v_one_sustain_remove_dig]       [v_multi_sustain_insert_dig]       [v_multi_sustain_remove_dig];
    [v_one]  [v_multi]  [inv_sustain]   [v_one_inv_sustain_insert_dig]   [v_one_inv_sustain_remove_dig]   [v_multi_inv_sustain_insert_dig]   [v_multi_inv_sustain_remove_dig];
    [v_one]  [v_multi]  [sostenuto]     [v_one_sostenuto_insert_dig]     [v_one_sostenuto_remove_dig]     [v_multi_sostenuto_insert_dig]     [v_multi_sostenuto_remove_dig];
    [v_one]  [v_multi]  [inv_sostenuto] [v_one_inv_sostenuto_insert_dig] [v_one_inv_sostenuto_remove_dig] [v_multi_inv_sostenuto_insert_dig] [v_multi_inv_sostenuto_remove_dig];
    [f_one]  [f_multi]  [sustain]       [f_one_sustain_insert_dig]       [f_one_sustain_remove_dig]       [f_multi_sustain_insert_dig]       [f_multi_sustain_remove_dig];
    [f_one]  [f_multi]  [inv_sustain]   [f_one_inv_sustain_insert_dig]   [f_one_inv_sustain_remove_dig]   [f_multi_inv_sustain_insert_dig]   [f_multi_inv_sustain_remove_dig];
    [f_one]  [f_multi]  [sostenuto]     [f_one_sostenuto_insert_dig]     [f_one_sostenuto_remove_dig]     [f_multi_sostenuto_insert_dig]     [f_multi_sostenuto_remove_dig];
    [f_one]  [f_multi]  [inv_sostenuto] [f_one_inv_sostenuto_insert_dig] [f_one_inv_sostenuto_remove_dig] [f_multi_inv_sostenuto_insert_dig] [f_multi_inv_sostenuto_remove_dig];
    [c_one]  [c_multi]  [sustain]       [c_one_sustain_insert_dig]       [c_one_sustain_remove_dig]       [c_multi_sustain_insert_dig]       [c_multi_sustain_remove_dig];
    [c_one]  [c_multi]  [inv_sustain]   [c_one_inv_sustain_insert_dig]   [c_one_inv_sustain_remove_dig]   [c_multi_inv_sustain_insert_dig]   [c_multi_inv_sustain_remove_dig];
    [c_one]  [c_multi]  [sostenuto]     [c_one_sostenuto_insert_dig]     [c_one_sostenuto_remove_dig]     [c_multi_sostenuto_insert_dig]     [c_multi_sostenuto_remove_dig];
    [c_one]  [c_multi]  [inv_sostenuto] [c_one_inv_sostenuto_insert_dig] [c_one_inv_sostenuto_remove_dig] [c_multi_inv_sostenuto_insert_dig] [c_multi_inv_sostenuto_remove_dig];
)]
impl Engine<Edit> {
    pub fn one_hold_insert_dig(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && self.guts[g_idx].onefield[set_idx]
                .holds
                .holdfield
                .togs
                .contains(&key_idx_val)
        {
            self.guts[g_idx].onefield[set_idx]
                .holds
                .holdfield
                .togs
                .push(key_idx_val);
        }
    }
    pub fn one_hold_remove_dig(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
        {
            self.guts[g_idx].onefield[set_idx]
                .holds
                .holdfield
                .togs
                .retain(|&idx| idx != key_idx_val)
        }
    }
    pub fn multi_hold_insert_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && set_idx < self.multifield.len()
            && self.multifield[set_idx]
                .holds
                .holdfield
                .togs
                .contains(&key_idx_val)
        {
            self.multifield[set_idx]
                .holds
                .holdfield
                .togs
                .push(key_idx_val);
        }
    }
    pub fn multi_hold_remove_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len() && set_idx < self.multifield.len() {
            self.multifield[set_idx]
                .holds
                .holdfield
                .togs
                .retain(|&idx| idx != key_idx_val)
        }
    }
}
