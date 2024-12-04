use crate::*;

impl Qop<Edit> {
    pub fn gut_insert_key(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            if !self.guts[g_idx].gut_triggers.togs.contains(&key_idx_val) {
                self.guts[g_idx].gut_triggers.togs.push(key_idx_val)
            };
        }
    }
    pub fn gut_remove_tog_key(&mut self, g_idx: usize, key_idx_val: usize) {
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

    pub fn sustain_insert_key(&mut self, key_idx_val: usize) {
        if !self.gut_holds.sustain.togs.contains(&key_idx_val) {
            self.gut_holds.sustain.togs.push(key_idx_val)
        }
    }
    pub fn inv_sustain_insert_key(&mut self, key_idx_val: usize) {
        if !self.gut_holds.inv_sustain.togs.contains(&key_idx_val) {
            self.gut_holds.inv_sustain.togs.push(key_idx_val)
        }
    }
    pub fn sostenuto_insert_key(&mut self, key_idx_val: usize) {
        if !self.gut_holds.sostenuto.togs.contains(&key_idx_val) {
            self.gut_holds.sostenuto.togs.push(key_idx_val)
        }
    }
    pub fn inv_sostenuto_insert_key(&mut self, key_idx_val: usize) {
        if !self.gut_holds.inv_sostenuto.togs.contains(&key_idx_val) {
            self.gut_holds.inv_sostenuto.togs.push(key_idx_val)
        }
    }
    pub fn sustain_remove_key(&mut self, key_idx_val: usize) {
        self.gut_holds.sustain.togs.retain(|&idx| idx != key_idx_val)
    }
    pub fn inv_sustain_remove_key(&mut self, key_idx_val: usize) {
        self.gut_holds.inv_sustain.togs.retain(|&idx| idx != key_idx_val)
    }
    pub fn sostenuto_remove_key(&mut self, key_idx_val: usize) {
        self.gut_holds.sostenuto.togs.retain(|&idx| idx != key_idx_val)
    }
    pub fn inv_sostenuto_remove_key(&mut self, key_idx_val: usize) {
        self.gut_holds.inv_sostenuto.togs.retain(|&idx| idx != key_idx_val)
    }
}