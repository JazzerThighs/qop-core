use crate::{*, qopedit::NewTrait};
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
}

#[duplicate_item(
    SetType     set_multi_insert_gut    set_multi_remove_gut    field;
    [VFRSet]    [insert_gut]            [remove_gut]            [buttons];
    [ComboSet]  [insert_gut]            [remove_gut]            [combos];
)]
impl SetType<Vec<i64>, Vec<f64>> {
    pub(crate) fn set_multi_insert_gut(&mut self, g_idx: usize) {
        for btn in 0..self.field.len() {
            self.field[btn].i_delta.insert(g_idx, 0i64);
            self.field[btn].x_delta.insert(g_idx, 0.0f64);
            self.field[btn].i_mem.insert(g_idx, 0i64);
            self.field[btn].x_mem.insert(g_idx, 0.0f64);
            for to in 0..self.field[btn].trnsp_one.len() {
                self.field[btn].trnsp_one[to].i_delta.insert(g_idx, 0i64);
                self.field[btn].trnsp_one[to].x_delta.insert(g_idx, 0.0f64);
            }
        }
        self.i_mem.insert(g_idx, 0i64);
        self.x_mem.insert(g_idx, 0.0f64);
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.insert(g_idx, 0i64);
            self.trnsp_all[ta].x_delta.insert(g_idx, 0.0f64);
        }
    }
    pub(crate) fn set_multi_remove_gut(&mut self, g_idx: usize) {
        for btn in 0..self.field.len() {
            self.field[btn].i_delta.remove(g_idx);
            self.field[btn].x_delta.remove(g_idx);
            self.field[btn].i_mem.remove(g_idx);
            self.field[btn].x_mem.remove(g_idx);
            for to in 0..self.field[btn].trnsp_one.len() {
                self.field[btn].trnsp_one[to].i_delta.remove(g_idx);
                self.field[btn].trnsp_one[to].x_delta.remove(g_idx);
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

mod _fields;
mod _set_one;
mod _trnsp;
mod _holds;