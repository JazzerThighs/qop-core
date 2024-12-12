use crate::{qopedit::{NewTrait, NewEnginePartParams}, *};
use duplicate::duplicate_item;

#[duplicate_item(
    onefield deltafield one_insert_trnsp_all_t     one_remove_trnsp_all_t     one_insert_trnsp_all_dig     one_remove_trnsp_all_dig     one_trnsp_all_change_deltas     one_insert_trnsp_one_t     one_remove_trnsp_one_t     one_insert_trnsp_one_dig     one_remove_trnsp_one_dig     one_trnsp_one_change_deltas;
    [v_one]  [buttons]  [v_one_insert_trnsp_all_t] [v_one_remove_trnsp_all_t] [v_one_insert_trnsp_all_dig] [v_one_remove_trnsp_all_dig] [v_one_trnsp_all_change_deltas] [v_one_insert_trnsp_one_t] [v_one_remove_trnsp_one_t] [v_one_insert_trnsp_one_dig] [v_one_remove_trnsp_one_dig] [v_one_trnsp_one_change_deltas];
    [f_one]  [buttons]  [f_one_insert_trnsp_all_t] [f_one_remove_trnsp_all_t] [f_one_insert_trnsp_all_dig] [f_one_remove_trnsp_all_dig] [f_one_trnsp_all_change_deltas] [f_one_insert_trnsp_one_t] [f_one_remove_trnsp_one_t] [f_one_insert_trnsp_one_dig] [f_one_remove_trnsp_one_dig] [f_one_trnsp_one_change_deltas];
    [c_one]  [combos]   [c_one_insert_trnsp_all_t] [c_one_remove_trnsp_all_t] [c_one_insert_trnsp_all_dig] [c_one_remove_trnsp_all_dig] [c_one_trnsp_all_change_deltas] [c_one_insert_trnsp_one_t] [c_one_remove_trnsp_one_t] [c_one_insert_trnsp_one_dig] [c_one_remove_trnsp_one_dig] [c_one_trnsp_one_change_deltas];
)]
impl Engine<Edit> {
    pub fn one_insert_trnsp_all_t(&mut self, g_idx: usize, set_idx: usize, trnsp_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && trnsp_idx <= self.guts[g_idx].onefield[set_idx].trnsp_all.len()
        {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.guts[g_idx].onefield[set_idx].trnsp_all_insert_t(trnsp_idx, &mut n);
        }
    }
    pub fn one_remove_trnsp_all_t(&mut self, g_idx: usize, set_idx: usize, trnsp_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && trnsp_idx < self.guts[g_idx].onefield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_all_remove_t(trnsp_idx);
        }
    }
    pub fn one_insert_trnsp_all_dig(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && trnsp_idx < self.guts[g_idx].onefield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_all_insert_dig(trnsp_idx, key_idx_val);
        }
    }
    pub fn one_remove_trnsp_all_dig(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && trnsp_idx < self.guts[g_idx].onefield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_all_remove_dig(trnsp_idx, key_idx_val);
        }
    }

    pub fn one_insert_trnsp_one_t(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && del_idx < self.guts[g_idx].onefield[set_idx].deltafield.len()
            && trnsp_idx
                <= self.guts[g_idx].onefield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.guts[g_idx].onefield[set_idx].trnsp_one_insert_t(del_idx, trnsp_idx, &mut n);
        }
    }
    pub fn one_remove_trnsp_one_t(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && del_idx < self.guts[g_idx].onefield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].onefield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_one_remove_t(del_idx, trnsp_idx);
        }
    }
    pub fn one_insert_trnsp_one_dig(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && del_idx < self.guts[g_idx].onefield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].onefield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_one_insert_dig(
                del_idx,
                trnsp_idx,
                key_idx_val,
            );
        }
    }
    pub fn one_remove_trnsp_one_dig(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && del_idx < self.guts[g_idx].onefield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].onefield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_one_remove_dig(
                del_idx,
                trnsp_idx,
                key_idx_val,
            );
        }
    }
}

#[duplicate_item(
    multifield deltafield multi_insert_trnsp_all_t     multi_remove_trnsp_all_t     multi_insert_trnsp_all_dig     multi_remove_trnsp_all_dig     multi_insert_trnsp_one_t     multi_remove_trnsp_one_t     multi_insert_trnsp_one_dig     multi_remove_trnsp_one_dig;
    [v_multi]  [buttons]  [v_multi_insert_trnsp_all_t] [v_multi_remove_trnsp_all_t] [v_multi_insert_trnsp_all_dig] [v_multi_remove_trnsp_all_dig] [v_multi_insert_trnsp_one_t] [v_multi_remove_trnsp_one_t] [v_multi_insert_trnsp_one_dig] [v_multi_remove_trnsp_one_dig];
    [f_multi]  [buttons]  [f_multi_insert_trnsp_all_t] [f_multi_remove_trnsp_all_t] [f_multi_insert_trnsp_all_dig] [f_multi_remove_trnsp_all_dig] [f_multi_insert_trnsp_one_t] [f_multi_remove_trnsp_one_t] [f_multi_insert_trnsp_one_dig] [f_multi_remove_trnsp_one_dig];
    [c_multi]  [combos]   [c_multi_insert_trnsp_all_t] [c_multi_remove_trnsp_all_t] [c_multi_insert_trnsp_all_dig] [c_multi_remove_trnsp_all_dig] [c_multi_insert_trnsp_one_t] [c_multi_remove_trnsp_one_t] [c_multi_insert_trnsp_one_dig] [c_multi_remove_trnsp_one_dig];
)]
impl Engine<Edit> {
    pub fn multi_insert_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].trnsp_all_insert_t(trnsp_idx, &mut n);
        }
    }
    pub fn multi_remove_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_remove_t(trnsp_idx);
        }
    }
    pub fn multi_insert_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_insert_dig(trnsp_idx, key_idx_val);
        }
    }
    pub fn multi_remove_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_remove_dig(trnsp_idx, key_idx_val);
        }
    }

    pub fn multi_insert_trnsp_one_t(&mut self, set_idx: usize, del_idx: usize, trnsp_idx: usize,) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx <= self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].trnsp_one_insert_t(del_idx, trnsp_idx, &mut n);
        }
    }
    pub fn multi_remove_trnsp_one_t(&mut self, set_idx: usize, del_idx: usize, trnsp_idx: usize,) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].trnsp_one_remove_t(del_idx, trnsp_idx);
        }
    }
    pub fn multi_insert_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.multifield[set_idx].trnsp_one_insert_dig(del_idx, trnsp_idx, key_idx_val);
        }
    }
    pub fn multi_remove_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.multifield[set_idx].trnsp_one_remove_dig(del_idx, trnsp_idx, key_idx_val);
        }
    }
}

#[duplicate_item(
    onefield multifield deltafield d_del_val   del_type      d_del_vec   del_type_vec       one_trnsp_all_change_deltas       one_trnsp_one_change_deltas       multi_trnsp_all_change_deltas       multi_trnsp_one_change_deltas       trnsp_all_change_deltas     trnsp_one_change_deltas;
    [v_one]  [v_multi]  [buttons]  [i_del_val] [Option<i64>] [i_del_vec] [Vec<Option<i64>>] [v_one_trnsp_all_change_i_deltas] [v_one_trnsp_one_change_i_deltas] [v_multi_trnsp_all_change_i_deltas] [v_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [f_one]  [f_multi]  [buttons]  [i_del_val] [Option<i64>] [i_del_vec] [Vec<Option<i64>>] [f_one_trnsp_all_change_i_deltas] [f_one_trnsp_one_change_i_deltas] [f_multi_trnsp_all_change_i_deltas] [f_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [c_one]  [c_multi]  [combos]   [i_del_val] [Option<i64>] [i_del_vec] [Vec<Option<i64>>] [c_one_trnsp_all_change_i_deltas] [c_one_trnsp_one_change_i_deltas] [c_multi_trnsp_all_change_i_deltas] [c_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [v_one]  [v_multi]  [buttons]  [x_del_val] [Option<f64>] [x_del_vec] [Vec<Option<f64>>] [v_one_trnsp_all_change_x_deltas] [v_one_trnsp_one_change_x_deltas] [v_multi_trnsp_all_change_x_deltas] [v_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
    [f_one]  [f_multi]  [buttons]  [x_del_val] [Option<f64>] [x_del_vec] [Vec<Option<f64>>] [f_one_trnsp_all_change_x_deltas] [f_one_trnsp_one_change_x_deltas] [f_multi_trnsp_all_change_x_deltas] [f_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
    [c_one]  [c_multi]  [combos]   [x_del_val] [Option<f64>] [x_del_vec] [Vec<Option<f64>>] [c_one_trnsp_all_change_x_deltas] [c_one_trnsp_one_change_x_deltas] [c_multi_trnsp_all_change_x_deltas] [c_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
)]
impl Engine<Edit> {
    pub fn one_trnsp_all_change_deltas(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        d_del_val: del_type,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && trnsp_idx < self.guts[g_idx].onefield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].onefield[set_idx].trnsp_all_change_deltas(trnsp_idx, d_del_val);
        }
    }
    pub fn one_trnsp_one_change_deltas(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_val: del_type,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && del_idx < self.guts[g_idx].onefield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].onefield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].onefield[set_idx]
                .trnsp_one_change_deltas(del_idx, trnsp_idx, d_del_val);
        }
    }

    pub fn multi_trnsp_all_change_deltas(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type_vec,
    ) {
        if set_idx < self.multifield.len()
            && d_del_vec.len() == self.guts.len()
            && trnsp_idx < self.multifield[set_idx].trnsp_all.len()
        {
            self.multifield[set_idx].trnsp_all_change_deltas(trnsp_idx, d_del_vec);
        }
    }
    pub fn multi_trnsp_one_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type_vec,
    ) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
            && d_del_vec.len() == self.guts.len()
        {
            self.multifield[set_idx].trnsp_one_change_deltas(del_idx, trnsp_idx, d_del_vec);
        }
    }
}

#[duplicate_item(
    SetType    deltafield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl<T: Default, U: Default> SetType<T, U>
where
    Trnsp<T, U>: NewTrait,
{
    pub(crate) fn trnsp_all_insert_t(&mut self, trnsp_idx: usize, n: &mut NewEnginePartParams) {
        self.trnsp_all.insert(trnsp_idx, Trnsp::new(n))
    }
    pub(crate) fn trnsp_all_remove_t(&mut self, trnsp_idx: usize) {
        self.trnsp_all.remove(trnsp_idx);
    }
    pub(crate) fn trnsp_all_insert_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if !self.trnsp_all[trnsp_idx].triggers.contains(&key_idx_val) {
            self.trnsp_all[trnsp_idx].triggers.push(key_idx_val);
        }
    }
    pub(crate) fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        self.trnsp_all[trnsp_idx]
            .triggers
            .retain(|&idx| idx != key_idx_val);
    }

    pub(crate) fn trnsp_one_insert_t(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        n: &mut NewEnginePartParams,
    ) {
        self.deltafield[del_idx]
            .trnsp_one
            .insert(trnsp_idx, Trnsp::new(n))
    }
    pub(crate) fn trnsp_one_remove_t(&mut self, del_idx: usize, trnsp_idx: usize) {
        self.deltafield[del_idx].trnsp_one.remove(trnsp_idx);
    }
    pub(crate) fn trnsp_one_insert_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if !self.deltafield[del_idx].trnsp_one[trnsp_idx]
            .triggers
            .contains(&key_idx_val)
        {
            self.deltafield[del_idx].trnsp_one[trnsp_idx]
                .triggers
                .push(key_idx_val)
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        self.deltafield[del_idx].trnsp_one[trnsp_idx]
            .triggers
            .retain(|&idx| idx != key_idx_val);
    }
}

#[duplicate_item(
    SetType    deltafield trnsp_all_change_deltas     trnsp_one_change_deltas     d_field   d_del_val   del_type;
    [VFSet]    [buttons]  [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Option<i64>];
    [VFSet]    [buttons]  [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Option<f64>];
    [ComboSet] [combos]   [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Option<i64>];
    [ComboSet] [combos]   [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Option<f64>];
)]
impl SetType<i64, f64> {
    pub(crate) fn trnsp_all_change_deltas(&mut self, trnsp_idx: usize, d_del_val: del_type) {
        if let Some(d_val) = d_del_val {
            self.trnsp_all[trnsp_idx].d_field = d_val;
        }
    }

    pub(crate) fn trnsp_one_change_deltas(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_val: del_type,
    ) {
        if let Some(i_val) = d_del_val {
            self.deltafield[del_idx].trnsp_one[trnsp_idx].d_field = i_val;
        }
    }
}

#[duplicate_item(
    SetType    deltafield trnsp_all_change_deltas     trnsp_one_change_deltas     d_field   d_del_val   del_type;
    [VFSet]    [buttons]  [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Vec<Option<i64>>];
    [VFSet]    [buttons]  [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Vec<Option<f64>>];
    [ComboSet] [combos]   [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Vec<Option<i64>>];
    [ComboSet] [combos]   [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Vec<Option<f64>>];
)]
impl SetType<Vec<i64>, Vec<f64>> {
    pub(crate) fn trnsp_all_change_deltas(&mut self, trnsp_idx: usize, d_del_vec: del_type) {
        for d in 0..d_del_vec.len() {
            if let Some(i_val) = d_del_vec[d] {
                self.trnsp_all[trnsp_idx].d_field[d] = i_val;
            }
        }
    }

    pub(crate) fn trnsp_one_change_deltas(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type,
    ) {
        for d in 0..d_del_vec.len() {
            if let Some(d_val) = d_del_vec[d] {
                self.deltafield[del_idx].trnsp_one[trnsp_idx].d_field[d] = d_val;
            }
        }
    }
}
