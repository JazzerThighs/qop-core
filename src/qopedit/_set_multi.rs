use crate::{*, qopedit::NewTrait};

impl Qop<Edit> {
    pub fn v_multi_insert_set(&mut self, set_idx: usize) {
        self.n.guts_len = self.guts.len();
        if set_idx <= self.v_multi.len() {
            self.v_multi.insert(set_idx, VFRSet::new(&mut self.n));
        }
    }
    pub fn f_multi_insert_set(&mut self, set_idx: usize) {
        self.n.guts_len = self.guts.len();
        if set_idx <= self.f_multi.len() {
            self.f_multi.insert(set_idx, VFRSet::new(&mut self.n));
        }
    }
    pub fn r_multi_insert_set(&mut self, set_idx: usize) {
        self.n.guts_len = self.guts.len();
        if set_idx <= self.r_multi.len() {
            self.r_multi.insert(set_idx, VFRSet::new(&mut self.n));
        }
    }
    pub fn c_multi_insert_set(&mut self, set_idx: usize) {
        self.n.guts_len = self.guts.len();
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

mod _fields;
mod _holds;
mod _trnsp;