use crate::*;

impl Qop<Edit> {
    pub fn v_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.v_multi.len() {
            self.v_multi.insert(set_idx, MultiSet::new(self.guts.len()));
        }
    }
    pub fn f_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.f_multi.len() {
            self.f_multi.insert(set_idx, MultiSet::new(self.guts.len()));
        }
    }
    pub fn r_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.r_multi.len() {
            self.r_multi.insert(set_idx, MultiSet::new(self.guts.len()));
        }
    }
    pub fn c_multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.c_multi
                .insert(set_idx, MultiComboSet::new(self.guts.len()));
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
    pub fn v_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.v_multi[set_idx].insert_btn(btn_idx, self.guts.len())
    }
    pub fn f_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.f_multi[set_idx].insert_btn(btn_idx, self.guts.len())
    }
    pub fn r_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.r_multi[set_idx].insert_btn(btn_idx, self.guts.len())
    }
    pub fn c_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.c_multi[set_idx].insert_btn(btn_idx)
    }
    pub fn v_multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.v_multi[set_idx].remove_btn(btn_idx)
    }
    pub fn f_multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.f_multi[set_idx].remove_btn(btn_idx)
    }
    pub fn r_multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.r_multi[set_idx].remove_btn(btn_idx)
    }
    pub fn c_multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.c_multi[set_idx].remove_btn(btn_idx)
    }
    pub fn c_multi_insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.c_multi[set_idx].insert_combo(c_idx, self.guts.len());
        }
    }
    pub fn c_multi_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.c_multi[set_idx].remove_combo(c_idx);
        }
    }
    pub fn v_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.v_multi.len() {
                self.v_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn f_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.f_multi.len() {
                self.f_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn r_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.r_multi.len() {
                self.r_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn c_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.c_multi.len() {
                self.c_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn v_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.v_multi.len() {
                self.v_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn f_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.f_multi.len() {
                self.f_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn r_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.r_multi.len() {
                self.r_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn c_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.key_codes.len() {
            if set_idx < self.c_multi.len() {
                self.c_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }

    // pub fn set_change_i_deltas(
    //     &mut self,

    //     set_idx: usize,
    //     del_idx: usize,
    //     i_del_vec: Vec<Option<i64>>,
    // ) {
    //     if del_idx < self.guts.len() {
    //         for (i, &i_del) in i_del_vec.iter().enumerate() {
    //             if let Some(delta) = i_del {

    //                         if set_idx < self.v_multi.len() {
    //                             self.v_multi[set_idx].buttons[del_idx].i_deltas[i] = delta
    //                         }
    //                     }

    //                         if set_idx < self.f_multi.len() {
    //                             self.f_multi[set_idx].buttons[del_idx].i_deltas[i] = delta
    //                         }
    //                     }

    //                         if set_idx < self.r_multi.len() {
    //                             self.r_multi[set_idx].buttons[del_idx].i_deltas[i] = delta
    //                         }
    //                     }

    //                         if set_idx < self.c_multi.len() {
    //                             self.c_multi[set_idx].combos[del_idx].i_deltas[i] = delta
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     self.check_gutdelta_lengths();
    //     self.check_digitalref_invariants();
    // }
    // pub fn set_change_x_deltas(
    //     &mut self,

    //     set_idx: usize,
    //     del_idx: usize,
    //     x_del_vec: Vec<Option<f64>>,
    // ) {
    //     for (x, &x_del) in x_del_vec.iter().enumerate() {
    //         if let Some(delta) = x_del {

    //                     if set_idx < self.v_multi.len() {
    //                         self.v_multi[set_idx].buttons[del_idx].x_deltas[x] = delta
    //                     }
    //                 }

    //                     if set_idx < self.f_multi.len() {
    //                         self.f_multi[set_idx].buttons[del_idx].x_deltas[x] = delta
    //                     }
    //                 }

    //                     if set_idx < self.r_multi.len() {
    //                         self.r_multi[set_idx].buttons[del_idx].x_deltas[x] = delta
    //                     }
    //                 }

    //                     if set_idx < self.c_multi.len() {
    //                         self.c_multi[set_idx].combos[del_idx].x_deltas[x] = delta
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     self.check_gutdelta_lengths();
    //     self.check_digitalref_invariants();
    // }
    // pub fn set_insert_hold_btn_key(
    //     &mut self,

    //     set_idx: usize,
    //     h_kind: HoldType,
    //     key_idx_val: usize,
    // ) {
    //     if key_idx_val <= self.key_codes.len() {

    //                 if set_idx < self.v_multi.len() {
    //                     self.v_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
    //                 }
    //             }

    //                 if set_idx < self.f_multi.len() {
    //                     self.f_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
    //                 }
    //             }

    //                 if set_idx < self.r_multi.len() {
    //                     self.r_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
    //                 }
    //             }

    //                 if set_idx < self.c_multi.len() {
    //                     self.c_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
    //                 }
    //             }
    //         }
    //     }
    //     self.check_gutdelta_lengths();
    //     self.check_digitalref_invariants();
    // }
    // pub fn set_remove_hold_btn_key(
    //     &mut self,

    //     set_idx: usize,
    //     h_kind: HoldType,
    //     key_idx_val: usize,
    // ) {
    //     if key_idx_val <= self.key_codes.len() {

    //                 if set_idx < self.v_multi.len() {
    //                     self.v_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
    //                 }
    //             }

    //                 if set_idx < self.f_multi.len() {
    //                     self.f_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
    //                 }
    //             }

    //                 if set_idx < self.r_multi.len() {
    //                     self.r_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
    //                 }
    //             }

    //                 if set_idx < self.c_multi.len() {
    //                     self.c_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
    //                 }
    //             }
    //         }
    //     }
    //     self.check_gutdelta_lengths();
    //     self.check_digitalref_invariants();
    // }
}
