use crate::*;
use duplicate::duplicate_item;

#[duplicate_item(
    holdfield       gut_hold_dig                    operation                          ;
    [sustain]       [gut_sustain_insert_dig]        [push(key_idx_val)]                ;
    [inv_sustain]   [gut_inv_sustain_insert_dig]    [push(key_idx_val)]                ;
    [sostenuto]     [gut_sostenuto_insert_dig]      [push(key_idx_val)]                ;
    [inv_sostenuto] [gut_inv_sostenuto_insert_dig]  [push(key_idx_val)]                ;
    [sustain]       [gut_sustain_remove_dig]        [retain(|&idx| idx != key_idx_val)];
    [inv_sustain]   [gut_inv_sustain_remove_dig]    [retain(|&idx| idx != key_idx_val)];
    [sostenuto]     [gut_sostenuto_remove_dig]      [retain(|&idx| idx != key_idx_val)];
    [inv_sostenuto] [gut_inv_sostenuto_remove_dig]  [retain(|&idx| idx != key_idx_val)];
)]
impl Engine<Edit> {
    pub fn gut_hold_dig(&mut self, key_idx_val: usize) {
        self.gut_holds.holdfield.togs.operation;
        self.gut_holds.holdfield.togs.dedup()
    }
}

#[duplicate_item(
    onefield multifield holdfield       one_hold_dig                     multi_hold_dig                     operation                          ;
    [v_one]  [v_multi]  [sustain]       [v_one_sustain_insert_dig]       [v_multi_sustain_insert_dig]       [push(key_idx_val)]                ;
    [v_one]  [v_multi]  [inv_sustain]   [v_one_inv_sustain_insert_dig]   [v_multi_inv_sustain_insert_dig]   [push(key_idx_val)]                ;
    [v_one]  [v_multi]  [sostenuto]     [v_one_sostenuto_insert_dig]     [v_multi_sostenuto_insert_dig]     [push(key_idx_val)]                ;
    [v_one]  [v_multi]  [inv_sostenuto] [v_one_inv_sostenuto_insert_dig] [v_multi_inv_sostenuto_insert_dig] [push(key_idx_val)]                ;
    [f_one]  [f_multi]  [sustain]       [f_one_sustain_insert_dig]       [f_multi_sustain_insert_dig]       [push(key_idx_val)]                ;
    [f_one]  [f_multi]  [inv_sustain]   [f_one_inv_sustain_insert_dig]   [f_multi_inv_sustain_insert_dig]   [push(key_idx_val)]                ;
    [f_one]  [f_multi]  [sostenuto]     [f_one_sostenuto_insert_dig]     [f_multi_sostenuto_insert_dig]     [push(key_idx_val)]                ;
    [f_one]  [f_multi]  [inv_sostenuto] [f_one_inv_sostenuto_insert_dig] [f_multi_inv_sostenuto_insert_dig] [push(key_idx_val)]                ;
    [c_one]  [c_multi]  [sustain]       [c_one_sustain_insert_dig]       [c_multi_sustain_insert_dig]       [push(key_idx_val)]                ;
    [c_one]  [c_multi]  [inv_sustain]   [c_one_inv_sustain_insert_dig]   [c_multi_inv_sustain_insert_dig]   [push(key_idx_val)]                ;
    [c_one]  [c_multi]  [sostenuto]     [c_one_sostenuto_insert_dig]     [c_multi_sostenuto_insert_dig]     [push(key_idx_val)]                ;
    [c_one]  [c_multi]  [inv_sostenuto] [c_one_inv_sostenuto_insert_dig] [c_multi_inv_sostenuto_insert_dig] [push(key_idx_val)]                ;
    [v_one]  [v_multi]  [sustain]       [v_one_sustain_remove_dig]       [v_multi_sustain_remove_dig]       [retain(|&idx| idx != key_idx_val)];
    [v_one]  [v_multi]  [inv_sustain]   [v_one_inv_sustain_remove_dig]   [v_multi_inv_sustain_remove_dig]   [retain(|&idx| idx != key_idx_val)];
    [v_one]  [v_multi]  [sostenuto]     [v_one_sostenuto_remove_dig]     [v_multi_sostenuto_remove_dig]     [retain(|&idx| idx != key_idx_val)];
    [v_one]  [v_multi]  [inv_sostenuto] [v_one_inv_sostenuto_remove_dig] [v_multi_inv_sostenuto_remove_dig] [retain(|&idx| idx != key_idx_val)];
    [f_one]  [f_multi]  [sustain]       [f_one_sustain_remove_dig]       [f_multi_sustain_remove_dig]       [retain(|&idx| idx != key_idx_val)];
    [f_one]  [f_multi]  [inv_sustain]   [f_one_inv_sustain_remove_dig]   [f_multi_inv_sustain_remove_dig]   [retain(|&idx| idx != key_idx_val)];
    [f_one]  [f_multi]  [sostenuto]     [f_one_sostenuto_remove_dig]     [f_multi_sostenuto_remove_dig]     [retain(|&idx| idx != key_idx_val)];
    [f_one]  [f_multi]  [inv_sostenuto] [f_one_inv_sostenuto_remove_dig] [f_multi_inv_sostenuto_remove_dig] [retain(|&idx| idx != key_idx_val)];
    [c_one]  [c_multi]  [sustain]       [c_one_sustain_remove_dig]       [c_multi_sustain_remove_dig]       [retain(|&idx| idx != key_idx_val)];
    [c_one]  [c_multi]  [inv_sustain]   [c_one_inv_sustain_remove_dig]   [c_multi_inv_sustain_remove_dig]   [retain(|&idx| idx != key_idx_val)];
    [c_one]  [c_multi]  [sostenuto]     [c_one_sostenuto_remove_dig]     [c_multi_sostenuto_remove_dig]     [retain(|&idx| idx != key_idx_val)];
    [c_one]  [c_multi]  [inv_sostenuto] [c_one_inv_sostenuto_remove_dig] [c_multi_inv_sostenuto_remove_dig] [retain(|&idx| idx != key_idx_val)];
)]
impl Engine<Edit> {
    pub fn one_hold_dig(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && set_idx < self.guts[g_idx].onefield.len() {
            self.guts[g_idx].onefield[set_idx]
                .holds
                .holdfield
                .togs
                .operation;
            self.guts[g_idx].onefield[set_idx]
                .holds
                .holdfield
                .togs
                .dedup();
        }
    }
    pub fn multi_hold_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].holds.holdfield.togs.operation;
            self.multifield[set_idx].holds.holdfield.togs.dedup();
        }
    }
}
