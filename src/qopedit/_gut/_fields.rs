use crate::*;

impl Qop<Edit> {
    pub fn gut_insert_dig(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            if !self.guts[g_idx].gut_triggers.togs.contains(&key_idx_val) {
                self.guts[g_idx].gut_triggers.togs.push(key_idx_val)
            };
        }
    }
    pub fn gut_remove_dig(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            self.guts[g_idx]
                .gut_triggers
                .togs
                .retain(|&idx| idx != key_idx_val);
        }
    }

    pub fn gut_change_index_out(&mut self, g_idx: usize, i_del_val: usize) {
        if g_idx < self.guts.len() {
            self.guts[g_idx].index_out = i_del_val;
        }
    }
    pub fn gut_change_extra_out(&mut self, g_idx: usize, x_del_val: f64) {
        if g_idx < self.guts.len() {
            self.guts[g_idx].extra_out = x_del_val;
        }
    }
}
