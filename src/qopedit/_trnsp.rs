use crate::{*, qopedit::NewTrait};
use duplicate::duplicate_item;

#[duplicate_item(
    setfield    deltafield  multi_insert_trnsp_all_t        multi_remove_trnsp_all_t        multi_remove_trnsp_all_dig      multi_trnsp_all_change_deltas       multi_insert_trnsp_one_t        multi_remove_trnsp_one_t        multi_remove_trnsp_one_dig      multi_trnsp_one_change_deltas;
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
        key_idx_vals: Vec<usize>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if set_idx < self.setfield.len()
            && i_del_vec.len() == self.guts.len()
            && x_del_vec.len() == self.guts.len()
            && trnsp_idx < self.setfield[set_idx].trnsp_all.len()
        {
            self.setfield[set_idx].trnsp_all_change_deltas(
                trnsp_idx,
                key_idx_vals,
                i_del_vec,
                x_del_vec,
            );
        }
    }

    // pub fn set_trnsp_one_params(
    //     &mut self,
    //     set_kind: SetType,
    //     set_idx: usize,
    //     btn_idx: usize,
    //     trnsp_idx: usize,
    //     key_idx_vals: Vec<Option<usize>>,
    //     i_del_vec: Vec<Option<i64>>,
    //     x_del_vec: Vec<Option<f64>>,
    // ) {
    //     match set_kind {
    //         SetType::ValveSet => {
    //             if set_idx <= self.v_multi.len() {
    //                 self.v_multi[set_idx].trnsp_one_params(
    //                     btn_idx,
    //                     trnsp_idx,
    //                     key_idx_vals,
    //                     i_del_vec,
    //                     x_del_vec,
    //                     self.guts.len(),
    //                 );
    //             }
    //         }
    //         SetType::FretSet => {
    //             if set_idx <= self.f_multi.len() {
    //                 self.f_multi[set_idx].trnsp_one_params(
    //                     btn_idx,
    //                     trnsp_idx,
    //                     key_idx_vals,
    //                     i_del_vec,
    //                     x_del_vec,
    //                     self.guts.len(),
    //                 );
    //             }
    //         }
    //         SetType::RadioSet => {
    //             if set_idx <= self.r_multi.len() {
    //                 self.r_multi[set_idx].trnsp_one_params(
    //                     btn_idx,
    //                     trnsp_idx,
    //                     key_idx_vals,
    //                     i_del_vec,
    //                     x_del_vec,
    //                     self.guts.len(),
    //                 );
    //             }
    //         }
    //         SetType::ComboSet => {
    //             if set_idx <= self.c_multi.len() {
    //                 self.c_multi[set_idx].trnsp_one_params(
    //                     btn_idx,
    //                     trnsp_idx,
    //                     key_idx_vals,
    //                     i_del_vec,
    //                     x_del_vec,
    //                     self.guts.len(),
    //                 );
    //             }
    //         }
    //     }
    // }
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
        key_idx_vals: Vec<usize>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if set_idx < self.setfield.len()
            && i_del_vec.len() == self.guts.len()
            && x_del_vec.len() == self.guts.len()
            && trnsp_idx < self.setfield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.setfield[set_idx].trnsp_one_change_deltas(
                trnsp_idx,
                del_idx,
                key_idx_vals,
                i_del_vec,
                x_del_vec,
            );
        }
    }
}

#[duplicate_item(
    SetType     deltafield;
    [VFRSet]    [buttons];
    [ComboSet]  [combos];
)]
impl SetType<Vec<i64>, Vec<f64>> {
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
    pub(crate) fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all[trnsp_idx]
                .triggers
                .retain(|&idx| idx != key_idx_val);
        }
    }
    
    pub(crate) fn trnsp_one_insert_t(&mut self, trnsp_idx: usize, del_idx: usize, n: &mut NewStuffPointers) {
        if trnsp_idx <= self.deltafield[del_idx].trnsp_one.len() {
            self.deltafield[del_idx].trnsp_one.insert(trnsp_idx, Trnsp::new(n))
        }
    }
    pub(crate) fn trnsp_one_remove_t(&mut self, trnsp_idx: usize, del_idx: usize) {
        if del_idx < self.deltafield.len() && trnsp_idx < self.deltafield[del_idx].trnsp_one.len() {
            self.deltafield[del_idx].trnsp_one.remove(trnsp_idx);
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
        &mut self,
        trnsp_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
    ) {
        if del_idx < self.deltafield.len() {
            if trnsp_idx < self.deltafield[del_idx].trnsp_one.len() {
                self.deltafield[del_idx].trnsp_one[trnsp_idx]
                    .triggers
                    .retain(|&idx| idx != key_idx_val);
            }
        }
    }
}
