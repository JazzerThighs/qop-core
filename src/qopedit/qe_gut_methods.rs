use crate::*;

impl VFRSet<Vec<i64>, Vec<f64>> {
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].i_delta.insert(g_idx, 0i64);
            self.buttons[btn].x_delta.insert(g_idx, 0.0f64);
            self.buttons[btn].i_mem.insert(g_idx, 0i64);
            self.buttons[btn].x_mem.insert(g_idx, 0.0f64);
            for to in 0..self.buttons[btn].trnsp_one.len() {
                self.buttons[btn].trnsp_one[to].i_delta.insert(g_idx, 0i64);
                self.buttons[btn].trnsp_one[to].x_delta.insert(g_idx, 0.0f64);
            }
        }
        self.i_mem.insert(g_idx, 0i64);
        self.x_mem.insert(g_idx, 0.0f64);
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.insert(g_idx, 0i64);
            self.trnsp_all[ta].x_delta.insert(g_idx, 0.0f64);
        }
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].i_delta.remove(g_idx);
            self.buttons[btn].x_delta.remove(g_idx);
            self.buttons[btn].i_mem.remove(g_idx);
            self.buttons[btn].x_mem.remove(g_idx);
            for to in 0..self.buttons[btn].trnsp_one.len() {
                self.buttons[btn].trnsp_one[to].i_delta.remove(g_idx);
                self.buttons[btn].trnsp_one[to].x_delta.remove(g_idx);
            }
        }
        self.i_mem.remove(g_idx);
        self.x_mem.remove(g_idx);
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.remove(g_idx);
            self.trnsp_all[ta].x_delta.remove(g_idx);
        }
    }
}

impl ComboSet<Vec<i64>, Vec<f64>> {
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].i_delta.insert(g_idx, 0i64);
            self.combos[c].x_delta.insert(g_idx, 0.0f64);
            self.combos[c].i_mem.insert(g_idx, 0i64);
            self.combos[c].x_mem.insert(g_idx, 0.0f64);
            for to in 0..self.combos[c].trnsp_one.len() {
                self.combos[c].trnsp_one[to].i_delta.insert(g_idx, 0i64);
                self.combos[c].trnsp_one[to].x_delta.insert(g_idx, 0.0f64);
            }
        }
        self.i_mem.insert(g_idx, 0i64);
        self.x_mem.insert(g_idx, 0.0f64);
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.insert(g_idx, 0i64);
            self.trnsp_all[ta].x_delta.insert(g_idx, 0.0f64);
        }
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].i_delta.remove(g_idx);
            self.combos[c].x_delta.remove(g_idx);
            self.combos[c].i_mem.remove(g_idx);
            self.combos[c].x_mem.remove(g_idx);
            for to in 0..self.combos[c].trnsp_one.len() {
                self.combos[c].trnsp_one[to].i_delta.remove(g_idx);
                self.combos[c].trnsp_one[to].x_delta.remove(g_idx);
            }
        }
        self.i_mem.remove(g_idx);
        self.x_mem.remove(g_idx);
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.remove(g_idx);
            self.trnsp_all[ta].x_delta.remove(g_idx);
        }
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
}

mod gut_field_methods;
mod set_one_methods;