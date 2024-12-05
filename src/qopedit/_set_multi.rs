use crate::{*, qopedit::NewTrait};

impl Qop<Edit> {
    pub fn v_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.v_multi.len() {
            self.v_multi.insert(set_idx, VFRSet::new(&mut self.n));
        }
    }
    pub fn f_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.f_multi.len() {
            self.f_multi.insert(set_idx, VFRSet::new(&mut self.n));
        }
    }
    pub fn r_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.r_multi.len() {
            self.r_multi.insert(set_idx, VFRSet::new(&mut self.n));
        }
    }
    pub fn c_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.c_multi
                .insert(set_idx, ComboSet::new(&mut self.n));
        }
    }
    pub fn v_multi_remove_set(&mut self, set_idx: usize) {
        if self.v_multi.len() > 0 && self.v_multi.len() > set_idx {
            self.v_multi.remove(set_idx);
        }
    }
    pub fn f_multi_remove_set(&mut self, set_idx: usize) {
        if self.f_multi.len() > 0 && self.f_multi.len() > set_idx {
            self.f_multi.remove(set_idx);
        }
    }
    pub fn r_multi_remove_set(&mut self, set_idx: usize) {
        if self.r_multi.len() > 0 && self.r_multi.len() > set_idx {
            self.r_multi.remove(set_idx);
        }
    }
    pub fn c_multi_remove_set(&mut self, set_idx: usize) {
        if self.c_multi.len() > 0 && self.c_multi.len() > set_idx {
            self.c_multi.remove(set_idx);
        }
    }
}

impl Qop<Edit> {
//     pub fn set_insert_hold_btn_dig(
//         &mut self,
//         set_idx: usize,
//         h_kind: HoldType,
//         key_idx_val: usize,
//     ) {
//         if key_idx_val <= self.dig_inputs.len() {
//                     if set_idx < self.v_multi.len() {
//                         self.v_multi[set_idx].hold_insert_dig(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.f_multi.len() {
//                         self.f_multi[set_idx].hold_insert_dig(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.r_multi.len() {
//                         self.r_multi[set_idx].hold_insert_dig(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.c_multi.len() {
//                         self.c_multi[set_idx].hold_insert_dig(h_kind, key_idx_val)
//                     }
//                 }
//             }
//         }
//         self.check_gutdelta_lengths();
//         self.check_digitalref_invariants();
//     }
//     pub fn set_remove_hold_btn_dig(
//         &mut self,
//         set_idx: usize,
//         h_kind: HoldType,
//         key_idx_val: usize,
//     ) {
//         if key_idx_val <= self.dig_inputs.len() {
//                     if set_idx < self.v_multi.len() {
//                         self.v_multi[set_idx].hold_remove_dig(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.f_multi.len() {
//                         self.f_multi[set_idx].hold_remove_dig(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.r_multi.len() {
//                         self.r_multi[set_idx].hold_remove_dig(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.c_multi.len() {
//                         self.c_multi[set_idx].hold_remove_dig(h_kind, key_idx_val)
//                     }
//                 }
//             }
//         }
//         self.check_gutdelta_lengths();
//         self.check_digitalref_invariants();
//     }
}

mod _fields;
mod _holds;
mod _trnsp;