use crate::*;
use duplicate::duplicate_item;

impl Engine<Edit> {
    pub fn gut_insert_g(&mut self, g_idx: usize) {
        if g_idx <= self.guts.len() {
            self.guts.insert(g_idx, Gut::default());
            let mut n: NewStuffPointers = NewStuffPointers::default();
            n.guts_len = self.guts.len();
            for set in 0..self.v_multi.len() {
                self.v_multi[set].insert_gut(g_idx);
            }
            for set in 0..self.f_multi.len() {
                self.f_multi[set].insert_gut(g_idx);
            }
            for set in 0..self.c_multi.len() {
                self.c_multi[set].insert_gut(g_idx);
            }
        }
    }
    pub fn gut_remove_g(&mut self, g_idx: usize) {
        if self.guts.len() > 1 && g_idx < self.guts.len() {
            self.guts.remove(g_idx);
            for set in 0..self.v_multi.len() {
                self.v_multi[set].remove_gut(g_idx);
            }
            for set in 0..self.f_multi.len() {
                self.f_multi[set].remove_gut(g_idx);
            }
            for set in 0..self.c_multi.len() {
                self.c_multi[set].remove_gut(g_idx);
            }
        }
    }
    pub fn gut_insert_dig(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            if !self.guts[g_idx].togs.contains(&key_idx_val) {
                self.guts[g_idx].togs.push(key_idx_val)
            };
        }
    }
    pub fn gut_remove_dig(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            self.guts[g_idx]
                .togs.retain(|&idx| idx != key_idx_val);
        }
    }
    pub fn gut_toggle_radio_mode(&mut self) {
        self.gut_radio_mode = !self.gut_radio_mode;
    }
    pub(crate) fn check_multi_delta_lengths(&self) {
        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_multi_delta_lengths(self.guts.len())
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_multi_delta_lengths(self.guts.len())
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_multi_delta_lengths(self.guts.len())
        }
    }
}

#[duplicate_item(
    gut_change_delta_out         d_out       d_del_val   del_type gut_change_minmax        minmaxval minmax_field      iscomparedto;
    [gut_change_index_delta_out] [index_out] [i_del_val] [usize]  [gut_change_min_pressed] [min_val] [gut_min_pressed] [le(&self.gut_max_pressed)];
    [gut_change_extra_delta_out] [extra_out] [x_del_val] [f64]    [gut_change_max_pressed] [max_val] [gut_max_pressed] [ge(&self.gut_max_pressed)];
)]
impl Engine<Edit> {
    pub fn gut_change_delta_out(&mut self, g_idx: usize, d_del_val: del_type) {
        if g_idx < self.guts.len() {
            self.guts[g_idx].d_out = d_del_val;
        }
    }
    pub fn gut_change_minmax(&mut self, minmaxval: usize) {
        if minmaxval.iscomparedto {
            self.minmax_field = minmaxval;
        }
    }
}

#[duplicate_item(
    SetType    multi_insertremove_gut insertremove_i        insertremove_x          deltafield;
    [VFSet]    [insert_gut]           [insert(g_idx, 0i64)] [insert(g_idx, 0.0f64)] [buttons];
    [VFSet]    [remove_gut]           [remove(g_idx)]       [remove(g_idx)]         [buttons];
    [ComboSet] [insert_gut]           [insert(g_idx, 0i64)] [insert(g_idx, 0.0f64)] [combos];
    [ComboSet] [remove_gut]           [remove(g_idx)]       [remove(g_idx)]         [combos];
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

macro_rules! assert_eq_expr {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!(
                "Assertion failed: `{}` == `{}`\n(left: `{:?}`, right: `{:?}`)",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            );
        }
    };
}

#[duplicate_item(
    SetType    field;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType<Vec<i64>, Vec<f64>> {
    pub(crate) fn check_multi_delta_lengths(
        &self,
        guts_len: usize,
    ) {
        for d in 0..self.field.len() {
            assert_eq_expr!(
                self.field[d].i_delta.len(),
                guts_len
            );
            assert_eq_expr!(
                self.field[d].x_delta.len(),
                guts_len
            );
            assert_eq_expr!(
                self.field[d].i_mem.len(),
                guts_len
            );
            assert_eq_expr!(
                self.field[d].x_mem.len(),
                guts_len
            );
            for to in 0..self.field[d].trnsp_one.len() {
                assert_eq_expr!(
                    self.field[d].trnsp_one[to].i_delta.len(),
                    guts_len
                );
                assert_eq_expr!(
                    self.field[d].trnsp_one[to].x_delta.len(),
                    guts_len
                );
            }
        }
        assert_eq_expr!(
            self.i_mem.len(),
            guts_len
        );
        assert_eq_expr!(
            self.x_mem.len(),
            guts_len
        );
        for ta in 0..self.trnsp_all.len() {
            assert_eq_expr!(
                self.trnsp_all[ta].i_delta.len(),
                guts_len
            );
            assert_eq_expr!(
                self.trnsp_all[ta].x_delta.len(),
                guts_len
            );
        }
    }
}
