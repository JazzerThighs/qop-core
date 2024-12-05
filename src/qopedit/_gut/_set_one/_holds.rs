use crate::*;
use duplicate::duplicate_item;

impl Qop<Edit> {
    #[duplicate_item(
        one_hold_insert_dig_method        field   setfunc;
        [v_one_sustain_insert_dig]        [v_one] [sustain_insert_dig];
        [v_one_inv_sustain_insert_dig]    [v_one] [inv_sustain_insert_dig];
        [v_one_sostenuto_insert_dig]      [v_one] [sostenuto_insert_dig];
        [v_one_inv_sostenuto_insert_dig]  [v_one] [inv_sostenuto_insert_dig];

        [f_one_sustain_insert_dig]        [f_one] [sustain_insert_dig];
        [f_one_inv_sustain_insert_dig]    [f_one] [inv_sustain_insert_dig];
        [f_one_sostenuto_insert_dig]      [f_one] [sostenuto_insert_dig];
        [f_one_inv_sostenuto_insert_dig]  [f_one] [inv_sostenuto_insert_dig];

        [r_one_sustain_insert_dig]        [r_one] [sustain_insert_dig];
        [r_one_inv_sustain_insert_dig]    [r_one] [inv_sustain_insert_dig];
        [r_one_sostenuto_insert_dig]      [r_one] [sostenuto_insert_dig];
        [r_one_inv_sostenuto_insert_dig]  [r_one] [inv_sostenuto_insert_dig];

        [c_one_sustain_insert_dig]        [c_one] [sustain_insert_dig];
        [c_one_inv_sustain_insert_dig]    [c_one] [inv_sustain_insert_dig];
        [c_one_sostenuto_insert_dig]      [c_one] [sostenuto_insert_dig];
        [c_one_inv_sostenuto_insert_dig]  [c_one] [inv_sostenuto_insert_dig];
      )]
    pub fn one_hold_insert_dig_method(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].field.len()
        {
            self.guts[g_idx].field[set_idx].setfunc(key_idx_val)
        }
    }
    
    #[duplicate_item(
        one_hold_remove_dig_method        field   setfunc;
        [v_one_sustain_remove_dig]        [v_one] [sustain_remove_dig];
        [v_one_inv_sustain_remove_dig]    [v_one] [inv_sustain_remove_dig];
        [v_one_sostenuto_remove_dig]      [v_one] [sostenuto_remove_dig];
        [v_one_inv_sostenuto_remove_dig]  [v_one] [inv_sostenuto_remove_dig];

        [f_one_sustain_remove_dig]        [f_one] [sustain_remove_dig];
        [f_one_inv_sustain_remove_dig]    [f_one] [inv_sustain_remove_dig];
        [f_one_sostenuto_remove_dig]      [f_one] [sostenuto_remove_dig];
        [f_one_inv_sostenuto_remove_dig]  [f_one] [inv_sostenuto_remove_dig];

        [r_one_sustain_remove_dig]        [r_one] [sustain_remove_dig];
        [r_one_inv_sustain_remove_dig]    [r_one] [inv_sustain_remove_dig];
        [r_one_sostenuto_remove_dig]      [r_one] [sostenuto_remove_dig];
        [r_one_inv_sostenuto_remove_dig]  [r_one] [inv_sostenuto_remove_dig];

        [c_one_sustain_remove_dig]        [c_one] [sustain_remove_dig];
        [c_one_inv_sustain_remove_dig]    [c_one] [inv_sustain_remove_dig];
        [c_one_sostenuto_remove_dig]      [c_one] [sostenuto_remove_dig];
        [c_one_inv_sostenuto_remove_dig]  [c_one] [inv_sostenuto_remove_dig];
      )]
    pub fn one_hold_remove_dig_method(&mut self, g_idx: usize, set_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len()
            && g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].field.len()
        {
            self.guts[g_idx].field[set_idx].setfunc(key_idx_val)
        }
    }
}
