use crate::{qopedit::NewTrait, *};
use duplicate::duplicate_item;

#[duplicate_item(
    setfield    deltafield one_insert_trnsp_all_t        one_remove_trnsp_all_t        one_remove_trnsp_all_dig      one_trnsp_all_change_deltas       one_insert_trnsp_one_t        one_remove_trnsp_one_t        one_remove_trnsp_one_dig      one_trnsp_one_change_deltas;
    [v_one]   [buttons]  [v_one_insert_trnsp_all_t]    [v_one_remove_trnsp_all_t]    [v_one_remove_trnsp_all_dig]  [v_one_trnsp_all_change_deltas]   [v_one_insert_trnsp_one_t]    [v_one_remove_trnsp_one_t]    [v_one_remove_trnsp_one_dig]  [v_one_trnsp_one_change_deltas];
    [f_one]   [buttons]  [f_one_insert_trnsp_all_t]    [f_one_remove_trnsp_all_t]    [f_one_remove_trnsp_all_dig]  [f_one_trnsp_all_change_deltas]   [f_one_insert_trnsp_one_t]    [f_one_remove_trnsp_one_t]    [f_one_remove_trnsp_one_dig]  [f_one_trnsp_one_change_deltas];
    [r_one]   [buttons]  [r_one_insert_trnsp_all_t]    [r_one_remove_trnsp_all_t]    [r_one_remove_trnsp_all_dig]  [r_one_trnsp_all_change_deltas]   [r_one_insert_trnsp_one_t]    [r_one_remove_trnsp_one_t]    [r_one_remove_trnsp_one_dig]  [r_one_trnsp_one_change_deltas];
    [c_one]   [combos]   [c_one_insert_trnsp_all_t]    [c_one_remove_trnsp_all_t]    [c_one_remove_trnsp_all_dig]  [c_one_trnsp_all_change_deltas]   [c_one_insert_trnsp_one_t]    [c_one_remove_trnsp_one_t]    [c_one_remove_trnsp_one_dig]  [c_one_trnsp_one_change_deltas];
)]
impl Qop<Edit> {
    pub fn one_insert_trnsp_all_t(&mut self, g_idx: usize, set_idx: usize, trnsp_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && trnsp_idx < self.guts[g_idx].setfield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_all_insert_t(trnsp_idx, &mut self.n);
        }
    }
    pub fn one_remove_trnsp_all_t(&mut self, g_idx: usize, set_idx: usize, trnsp_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && trnsp_idx < self.guts[g_idx].setfield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_all_remove_t(trnsp_idx);
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
            && set_idx < self.guts[g_idx].setfield.len()
            && trnsp_idx < self.guts[g_idx].setfield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_all_remove_dig(trnsp_idx, key_idx_val);
        }
    }
    pub fn one_trnsp_all_change_deltas(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        i_del_val: Option<i64>,
        x_del_val: Option<f64>,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && trnsp_idx < self.guts[g_idx].setfield[set_idx].trnsp_all.len()
        {
            self.guts[g_idx].setfield[set_idx]
                .trnsp_all_change_deltas(trnsp_idx, i_del_val, x_del_val);
        }
    }

    pub fn one_insert_trnsp_one_t(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        del_idx: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && del_idx < self.guts[g_idx].setfield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].setfield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_one_insert_t(trnsp_idx, del_idx, &mut self.n);
        }
    }
    pub fn one_remove_trnsp_one_t(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        del_idx: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && del_idx < self.guts[g_idx].setfield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].setfield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_one_remove_t(trnsp_idx, del_idx);
        }
    }
    pub fn one_remove_trnsp_one_dig(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && del_idx < self.guts[g_idx].setfield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].setfield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_one_remove_dig(
                trnsp_idx,
                del_idx,
                key_idx_val,
            );
        }
    }
    pub fn one_trnsp_one_change_deltas(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        trnsp_idx: usize,
        del_idx: usize,
        i_del_val: Option<i64>,
        x_del_val: Option<f64>,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].setfield.len()
            && del_idx < self.guts[g_idx].setfield[set_idx].deltafield.len()
            && trnsp_idx
                < self.guts[g_idx].setfield[set_idx].deltafield[del_idx]
                    .trnsp_one
                    .len()
        {
            self.guts[g_idx].setfield[set_idx].trnsp_one_change_deltas(
                trnsp_idx,
                del_idx,
                i_del_val,
                x_del_val,
            );
        }
    }
}

#[duplicate_item(
    setfield    deltafield multi_insert_trnsp_all_t        multi_remove_trnsp_all_t        multi_remove_trnsp_all_dig      multi_trnsp_all_change_deltas       multi_insert_trnsp_one_t        multi_remove_trnsp_one_t        multi_remove_trnsp_one_dig      multi_trnsp_one_change_deltas;
    [v_multi]   [buttons]  [v_multi_insert_trnsp_all_t]    [v_multi_remove_trnsp_all_t]    [v_multi_remove_trnsp_all_dig]  [v_multi_trnsp_all_change_deltas]   [v_multi_insert_trnsp_one_t]    [v_multi_remove_trnsp_one_t]    [v_multi_remove_trnsp_one_dig]  [v_multi_trnsp_one_change_deltas];
    [f_multi]   [buttons]  [f_multi_insert_trnsp_all_t]    [f_multi_remove_trnsp_all_t]    [f_multi_remove_trnsp_all_dig]  [f_multi_trnsp_all_change_deltas]   [f_multi_insert_trnsp_one_t]    [f_multi_remove_trnsp_one_t]    [f_multi_remove_trnsp_one_dig]  [f_multi_trnsp_one_change_deltas];
    [r_multi]   [buttons]  [r_multi_insert_trnsp_all_t]    [r_multi_remove_trnsp_all_t]    [r_multi_remove_trnsp_all_dig]  [r_multi_trnsp_all_change_deltas]   [r_multi_insert_trnsp_one_t]    [r_multi_remove_trnsp_one_t]    [r_multi_remove_trnsp_one_dig]  [r_multi_trnsp_one_change_deltas];
    [c_multi]   [combos]   [c_multi_insert_trnsp_all_t]    [c_multi_remove_trnsp_all_t]    [c_multi_remove_trnsp_all_dig]  [c_multi_trnsp_all_change_deltas]   [c_multi_insert_trnsp_one_t]    [c_multi_remove_trnsp_one_t]    [c_multi_remove_trnsp_one_dig]  [c_multi_trnsp_one_change_deltas];
)]
impl Qop<Edit> {
    pub fn multi_insert_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) {
        if set_idx < self.setfield.len() {
            self.setfield[set_idx].trnsp_all_insert_t(trnsp_idx, &mut self.n);
        }
    }
    pub fn multi_remove_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) {
        if set_idx < self.setfield.len() {
            self.setfield[set_idx].trnsp_all_remove_t(trnsp_idx);
        }
    }

    pub fn multi_remove_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.setfield.len() {
            self.setfield[set_idx].trnsp_all_remove_dig(trnsp_idx, key_idx_val);
        }
    }
    pub fn multi_trnsp_all_change_deltas(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if set_idx < self.setfield.len()
            && i_del_vec.len() == self.guts.len()
            && x_del_vec.len() == self.guts.len()
            && trnsp_idx < self.setfield[set_idx].trnsp_all.len()
        {
            self.setfield[set_idx].trnsp_all_change_deltas(trnsp_idx, i_del_vec, x_del_vec);
        }
    }

    pub fn multi_insert_trnsp_one_t(&mut self, set_idx: usize, trnsp_idx: usize, del_idx: usize) {
        if set_idx < self.setfield.len() {
            self.setfield[set_idx].trnsp_one_insert_t(trnsp_idx, del_idx, &mut self.n);
        }
    }
    pub fn multi_remove_trnsp_one_t(&mut self, set_idx: usize, trnsp_idx: usize, del_idx: usize) {
        if set_idx < self.setfield.len() {
            self.setfield[set_idx].trnsp_one_remove_t(trnsp_idx, del_idx);
        }
    }
    pub fn multi_remove_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.setfield.len() {
            self.setfield[set_idx].trnsp_one_remove_dig(trnsp_idx, del_idx, key_idx_val);
        }
    }
    pub fn multi_trnsp_one_change_deltas(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if set_idx < self.setfield.len()
            && i_del_vec.len() == self.guts.len()
            && x_del_vec.len() == self.guts.len()
            && trnsp_idx < self.setfield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.setfield[set_idx]
                .trnsp_one_change_deltas(trnsp_idx, del_idx, i_del_vec, x_del_vec);
        }
    }
}

#[duplicate_item(
    SetType     deltafield;
    [VFRSet]    [buttons];
    [ComboSet]  [combos];
)]
impl<T, U> SetType<T, U>
where
    Trnsp<T, U>: NewTrait,
{
    pub(crate) fn trnsp_all_insert_t(&mut self, trnsp_idx: usize, n: &mut NewStuffPointers) {
        if trnsp_idx <= self.trnsp_all.len() {
            self.trnsp_all.insert(trnsp_idx, Trnsp::new(n))
        }
    }
    pub(crate) fn trnsp_all_remove_t(&mut self, trnsp_idx: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all.remove(trnsp_idx);
        }
    }
    pub(crate) fn trnsp_all_insert_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            if !self.trnsp_all[trnsp_idx].triggers.contains(&key_idx_val) {
                self.trnsp_all[trnsp_idx].triggers.push(key_idx_val);
            }
        }
    }
    pub(crate) fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all[trnsp_idx]
                .triggers
                .retain(|&idx| idx != key_idx_val);
        }
    }

    pub(crate) fn trnsp_one_insert_t(
        &mut self,
        trnsp_idx: usize,
        del_idx: usize,
        n: &mut NewStuffPointers,
    ) {
        if trnsp_idx <= self.deltafield[del_idx].trnsp_one.len() {
            self.deltafield[del_idx]
                .trnsp_one
                .insert(trnsp_idx, Trnsp::new(n))
        }
    }
    pub(crate) fn trnsp_one_remove_t(&mut self, trnsp_idx: usize, del_idx: usize) {
        if del_idx < self.deltafield.len() && trnsp_idx < self.deltafield[del_idx].trnsp_one.len() {
            self.deltafield[del_idx].trnsp_one.remove(trnsp_idx);
        }
    }
    pub(crate) fn trnsp_one_insert_dig(
        &mut self,
        trnsp_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
    ) {
        if del_idx < self.deltafield.len() && trnsp_idx < self.trnsp_all.len() {
            if !self.deltafield[del_idx].trnsp_one[trnsp_idx]
                .triggers
                .contains(&key_idx_val)
            {
                self.deltafield[del_idx].trnsp_one[trnsp_idx]
                    .triggers
                    .push(key_idx_val)
            }
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
        &mut self,
        trnsp_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
    ) {
        if del_idx < self.deltafield.len() && trnsp_idx < self.deltafield[del_idx].trnsp_one.len() {
            self.deltafield[del_idx].trnsp_one[trnsp_idx]
                .triggers
                .retain(|&idx| idx != key_idx_val);
        }
    }
}

#[duplicate_item(
    SetType     deltafield;
    [VFRSet]    [buttons];
    [ComboSet]  [combos];
)]
impl SetType<i64, f64> {
    pub(crate) fn trnsp_all_change_deltas(
        &mut self,
        trnsp_idx: usize,
        i_del_val: Option<i64>,
        x_del_val: Option<f64>,
    ) {
        if let Some(i_val) = i_del_val {
            self.trnsp_all[trnsp_idx].i_delta = i_val;
        }

        if let Some(x_val) = x_del_val {
            self.trnsp_all[trnsp_idx].x_delta = x_val;
        }
    }

    pub(crate) fn trnsp_one_change_deltas(
        &mut self,
        trnsp_idx: usize,
        del_idx: usize,
        i_del_val: Option<i64>,
        x_del_val: Option<f64>,
    ) {
        if let Some(i_val) = i_del_val {
            self.deltafield[del_idx].trnsp_one[trnsp_idx].i_delta = i_val;
        }

        if let Some(x_val) = x_del_val {
            self.deltafield[del_idx].trnsp_one[trnsp_idx].x_delta = x_val;
        }
    }
}

#[duplicate_item(
    SetType     deltafield;
    [VFRSet]    [buttons];
    [ComboSet]  [combos];
)]
impl SetType<Vec<i64>, Vec<f64>> {
    pub(crate) fn trnsp_all_change_deltas(
        &mut self,
        trnsp_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        for i in 0..i_del_vec.len() {
            if let Some(i_val) = i_del_vec[i] {
                self.trnsp_all[trnsp_idx].i_delta[i] = i_val;
            }
        }
        for x in 0..x_del_vec.len() {
            if let Some(x_val) = x_del_vec[x] {
                self.trnsp_all[trnsp_idx].x_delta[x] = x_val;
            }
        }
    }

    pub(crate) fn trnsp_one_change_deltas(
        &mut self,
        trnsp_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        for i in 0..i_del_vec.len() {
            if let Some(i_val) = i_del_vec[i] {
                self.deltafield[del_idx].trnsp_one[trnsp_idx].i_delta[i] = i_val;
            }
        }
        for x in 0..x_del_vec.len() {
            if let Some(x_val) = x_del_vec[x] {
                self.deltafield[del_idx].trnsp_one[trnsp_idx].x_delta[x] = x_val;
            }
        }
    }
}
