use crate::{*, qopedit::NewTrait};
use duplicate::duplicate_item;

impl Qop<Edit> {
    #[duplicate_item(
        multi_insert_set        SetType     field;
        [v_multi_insert_set]    [VFRSet]    [v_multi];
        [f_multi_insert_set]    [VFRSet]    [f_multi];
        [r_multi_insert_set]    [VFRSet]    [r_multi];
        [c_multi_insert_set]    [ComboSet]  [c_multi];
    )]
    pub fn multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.field.len() {
            self.field.insert(set_idx, SetType::new(&mut self.n));
        }
    }

    #[duplicate_item(
        multi_remove_set        field;
        [v_multi_remove_set]    [v_multi];
        [f_multi_remove_set]    [f_multi];
        [r_multi_remove_set]    [r_multi];
        [c_multi_remove_set]    [c_multi];
    )]
    pub fn multi_remove_set(&mut self, set_idx: usize) {
        if self.field.len() > 0 && self.field.len() > set_idx {
            self.field.remove(set_idx);
        }
    }
}

mod _fields;
mod _holds;
mod _trnsp;