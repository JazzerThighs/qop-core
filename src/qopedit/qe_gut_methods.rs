use crate::*;

impl VFRSet<Vec<i64>, Vec<f64>> {
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].insert_gut(g_idx);
            self.i_mem.insert(g_idx, 0i64);
            self.x_mem.insert(g_idx, 0.0f64);

            for to in 0..self.buttons[btn].trnsp_one.len() {
                self.buttons[btn].trnsp_one[to].insert_gut(g_idx);
            }
        }
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].remove_gut(g_idx);
            self.i_mem.remove(g_idx);
            self.x_mem.remove(g_idx);

            for to in 0..self.buttons[btn].trnsp_one.len() {
                self.buttons[btn].trnsp_one[to].remove_gut(g_idx);
            }
        }
    }
}

impl VFRIndv<Vec<i64>, Vec<f64>> {
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        self.i_delta.insert(g_idx, 0i64);
        self.x_delta.insert(g_idx, 0.0f64);
        self.i_mem.insert(g_idx, 0i64);
        self.x_mem.insert(g_idx, 0.0f64);
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        self.i_delta.remove(g_idx);
        self.x_delta.remove(g_idx);
        self.i_mem.remove(g_idx);
        self.x_mem.remove(g_idx);
    }
}

impl ComboSet<Vec<i64>, Vec<f64>> {
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].insert_gut(g_idx);
            self.i_mem.insert(g_idx, 0i64);
            self.x_mem.insert(g_idx, 0.0f64);

            for to in 0..self.combos[c].trnsp_one.len() {
                self.combos[c].trnsp_one[to].insert_gut(g_idx);
            }
        }
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].remove_gut(g_idx);
            self.i_mem.remove(g_idx);
            self.x_mem.remove(g_idx);
        }
    }
}

impl Combo<Vec<i64>, Vec<f64>> {
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        self.i_delta.insert(g_idx, 0i64);
        self.x_delta.insert(g_idx, 0.0f64);
        self.i_mem.insert(g_idx, 0i64);
        self.x_mem.insert(g_idx, 0.0f64);
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        self.i_delta.remove(g_idx);
        self.x_delta.remove(g_idx);
        self.i_mem.remove(g_idx);
        self.x_mem.remove(g_idx);
    }
}

impl Qop<Edit> {
    pub fn gut_insert_g(&mut self, g_idx: usize) {
        if g_idx <= self.guts.len() {
            self.guts.insert(g_idx, Gut::default());
            for set in 0..self.v_multi.len() {
                self.v_multi[set].insert_gut(g_idx);
            }
            for set in 0..self.f_multi.len() {
                self.f_multi[set].insert_gut(g_idx);
            }
            for set in 0..self.r_multi.len() {
                self.r_multi[set].insert_gut(g_idx);
            }
            for set in 0..self.c_multi.len() {
                self.c_multi[set].insert_gut(g_idx);
            }
        }
    }
    pub fn gut_remove_g(&mut self, g_idx: usize) {
        if self.guts.len() > 1 && g_idx <= self.guts.len() {
            self.guts.remove(g_idx);
            for set in 0..self.v_multi.len() {
                self.v_multi[set].remove_gut(g_idx);
            }
            for set in 0..self.f_multi.len() {
                self.f_multi[set].remove_gut(g_idx);
            }
            for set in 0..self.r_multi.len() {
                self.r_multi[set].remove_gut(g_idx);
            }
            for set in 0..self.c_multi.len() {
                self.c_multi[set].remove_gut(g_idx);
            }
        }
    }
    pub fn gut_insert_key(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            if !self.guts[g_idx].gut_triggers.togs.contains(&key_idx_val) {
                self.guts[g_idx].gut_triggers.togs.push(key_idx_val)
            };
        }
    }
    pub fn gut_remove_key(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            self.guts[g_idx]
                .gut_triggers
                .togs
                .retain_mut(|idx: &mut usize| {
                    if *idx < key_idx_val {
                        return true;
                    } else if *idx == key_idx_val {
                        return false;
                    } else {
                        *idx -= 1;
                        return true;
                    }
                });
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
