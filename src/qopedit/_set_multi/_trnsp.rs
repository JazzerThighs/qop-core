use crate::{*, qopedit::NewTrait};
use duplicate::duplicate_item;

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
    pub(crate) fn trnsp_one_remove_t(&mut self, del_idx: usize, trnsp_idx: usize) {
        if del_idx < self.deltafield.len() && trnsp_idx < self.deltafield[del_idx].trnsp_one.len() {
            self.deltafield[del_idx].trnsp_one.remove(trnsp_idx);
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
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
