use crate::{qopedit::NewTrait, *};
use duplicate::duplicate_item;

impl Qop<Edit> {
    pub fn gut_insert_g(&mut self, g_idx: usize) {
        if g_idx <= self.guts.len() {
            self.guts.insert(g_idx, Gut::new(&mut self.n));
            self.n.guts_len += 1;
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
        if self.guts.len() > 1 && g_idx < self.guts.len() {
            self.guts.remove(g_idx);
            self.n.guts_len -= 1;
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

    pub(crate) fn check_multi_delta_lengths(&self) {
        let message: &str = " does not have the same length as self.guts!";
        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_multi_delta_lengths(format!("self.v_multi[{set}]").as_str(), message, self.guts.len())
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_multi_delta_lengths(format!("self.f_multi[{set}]").as_str(), message, self.guts.len())
        }
        for set in 0..self.r_multi.len() {
            self.r_multi[set].check_multi_delta_lengths(format!("self.r_multi[{set}]").as_str(), message, self.guts.len())
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_multi_delta_lengths(format!("self.c_multi[{set}]").as_str(), message, self.guts.len())
        }
    }

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

#[duplicate_item(
    SetType     multi_insertremove_gut  insertremove_i          insertremove_x          deltafield;
    [VFRSet]    [insert_gut]            [insert(g_idx, 0i64)]   [insert(g_idx, 0.0f64)] [buttons];
    [VFRSet]    [remove_gut]            [remove(g_idx)]         [remove(g_idx)]         [buttons];
    [ComboSet]  [insert_gut]            [insert(g_idx, 0i64)]   [insert(g_idx, 0.0f64)] [combos];
    [ComboSet]  [remove_gut]            [remove(g_idx)]         [remove(g_idx)]         [combos];
)]
impl SetType<Vec<i64>, Vec<f64>> {
    pub(crate) fn multi_insertremove_gut(&mut self, g_idx: usize) {
        for del_idx in 0..self.deltafield.len() {
            self.deltafield[del_idx].i_delta.insertremove_i;
            self.deltafield[del_idx].x_delta.insertremove_x;
            self.deltafield[del_idx].i_mem.insertremove_i;
            self.deltafield[del_idx].x_mem.insertremove_x;
            for to in 0..self.deltafield[del_idx].trnsp_one.len() {
                self.deltafield[del_idx].trnsp_one[to]
                    .i_delta
                    .insertremove_i;
                self.deltafield[del_idx].trnsp_one[to]
                    .x_delta
                    .insertremove_x;
            }
        }
        self.i_mem.insertremove_i;
        self.x_mem.insertremove_x;
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.insertremove_i;
            self.trnsp_all[ta].x_delta.insertremove_x;
        }
    }
}

#[duplicate_item(
    SetType     field;
    [VFRSet]    [buttons];
    [ComboSet]  [combos];
)]
impl SetType<Vec<i64>, Vec<f64>> {
    pub(crate) fn check_multi_delta_lengths(&self, leading_str: &str, message: &str, gut_len: usize) {
        for d in 0..self.field.len() {
            assert_eq!(self.field[d].i_delta.len(), gut_len, "{leading_str}.deltafield[{d}].i_delta{message}");
            assert_eq!(self.field[d].x_delta.len(), gut_len, "{leading_str}.deltafield[{d}].x_delta{message}");
            for to in 0..self.field[d].trnsp_one.len() {
                assert_eq!(self.field[d].trnsp_one[to].i_delta.len(), gut_len, "{leading_str}.deltafield[{d}].trnsp_one[{to}].i_delta{message}");
                assert_eq!(self.field[d].trnsp_one[to].x_delta.len(), gut_len, "{leading_str}.deltafield[{d}].trnsp_one[{to}].x_delta{message}")
            }
        }
        for ta in 0..self.trnsp_all.len() {
            assert_eq!(self.trnsp_all[ta].i_delta.len(), gut_len, "{leading_str}.trnsp_all[{ta}].i_delta{message}");
            assert_eq!(self.trnsp_all[ta].x_delta.len(), gut_len, "{leading_str}.trnsp_all[{ta}].x_delta{message}");
        }
    }
}
