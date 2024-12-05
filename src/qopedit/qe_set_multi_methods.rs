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
    
    pub fn v_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.n.guts_len = self.guts.len();
        self.v_multi[set_idx].insert_btn(btn_idx, &mut self.n);
    }
    pub fn f_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.n.guts_len = self.guts.len();
        self.f_multi[set_idx].insert_btn(btn_idx, &mut self.n);
    }
    pub fn r_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.n.guts_len = self.guts.len();
        self.r_multi[set_idx].insert_btn(btn_idx, &mut self.n);
    }
    pub fn c_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.n.guts_len = self.guts.len();
        self.c_multi[set_idx].insert_btn(btn_idx);
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
            self.n.guts_len = self.guts.len();
            self.n.c_btn_len = self.c_multi[set_idx].buttons.len();
            self.c_multi[set_idx].insert_combo(c_idx, &mut self.n);
        }
    }
    pub fn c_multi_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.c_multi[set_idx].remove_combo(c_idx);
        }
    }
    
    pub fn v_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.v_multi.len() {
                self.v_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn f_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.f_multi.len() {
                self.f_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn r_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.r_multi.len() {
                self.r_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn c_multi_insert_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.c_multi.len() {
                self.c_multi[set_idx].btn_insert_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn v_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.v_multi.len() {
                self.v_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn f_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.f_multi.len() {
                self.f_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn r_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.r_multi.len() {
                self.r_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    pub fn c_multi_remove_btn_key(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val <= self.dig_inputs.len() {
            if set_idx < self.c_multi.len() {
                self.c_multi[set_idx].btn_remove_key(btn_idx, key_idx_val)
            }
        }
    }
    
    pub fn v_multi_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if del_idx < self.guts.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    if set_idx < self.v_multi.len() {
                        self.v_multi[set_idx].buttons[del_idx].i_delta[i] = delta
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if let Some(delta) = x_del {
                    if set_idx < self.v_multi.len() {
                        self.v_multi[set_idx].buttons[del_idx].x_delta[x] = delta
                    }
                }
            }
        }
    }
    pub fn f_multi_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if del_idx < self.guts.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    if set_idx < self.f_multi.len() {
                        self.f_multi[set_idx].buttons[del_idx].i_delta[i] = delta
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if let Some(delta) = x_del {
                    if set_idx < self.f_multi.len() {
                        self.f_multi[set_idx].buttons[del_idx].x_delta[x] = delta
                    }
                }
            }
        }
    }
    pub fn r_multi_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if del_idx < self.guts.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    if set_idx < self.r_multi.len() {
                        self.r_multi[set_idx].buttons[del_idx].i_delta[i] = delta
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if let Some(delta) = x_del {
                    if set_idx < self.r_multi.len() {
                        self.r_multi[set_idx].buttons[del_idx].x_delta[x] = delta
                    }
                }
            }
        }
    }
    pub fn c_multi_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if del_idx < self.guts.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    if set_idx < self.c_multi.len() {
                        self.c_multi[set_idx].buttons[del_idx].i_delta[i] = delta
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if let Some(delta) = x_del {
                    if set_idx < self.c_multi.len() {
                        self.c_multi[set_idx].buttons[del_idx].x_delta[x] = delta
                    }
                }
            }
        }
    }
}

impl<T, U> VFRSet<T, U>
where
    VFRIndv<T, U>: NewTrait,
{
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, n: &mut NewStuffPointers) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, VFRIndv::new(n));
        }
    }
}

impl<T, U> VFRSet<T, U> {
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
}

impl<T, U> ComboSet<T, U> {
    pub(crate) fn insert_btn(&mut self, btn_idx: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, BtnTog::default());
            for c in 0..self.combos.len() {
                self.combos[c].combo.insert(btn_idx, false);
            }
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
            for c in 0..self.combos.len() {
                self.combos[c].combo.remove(btn_idx);
            }
        }
    }
    pub(crate) fn remove_combo(&mut self, c_idx: usize) {
        if self.combos.len() > 0 && c_idx < self.combos.len() {
            self.combos.remove(c_idx);
        }
    }
}
impl<T, U> ComboSet<T, U>
where
    Combo<T, U>: NewTrait,
{
    pub(crate) fn insert_combo(&mut self, c_idx: usize, n: &mut NewStuffPointers) {
        n.c_btn_len = self.buttons.len();
        if c_idx <= self.combos.len() {
            self.combos
                .insert(c_idx, Combo::new(n));
        }
    }
}

impl Qop<Edit> {
//     pub fn set_insert_hold_btn_key(
//         &mut self,
//         set_idx: usize,
//         h_kind: HoldType,
//         key_idx_val: usize,
//     ) {
//         if key_idx_val <= self.dig_inputs.len() {
//                     if set_idx < self.v_multi.len() {
//                         self.v_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.f_multi.len() {
//                         self.f_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.r_multi.len() {
//                         self.r_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.c_multi.len() {
//                         self.c_multi[set_idx].hold_insert_key(h_kind, key_idx_val)
//                     }
//                 }
//             }
//         }
//         self.check_gutdelta_lengths();
//         self.check_digitalref_invariants();
//     }
//     pub fn set_remove_hold_btn_key(
//         &mut self,
//         set_idx: usize,
//         h_kind: HoldType,
//         key_idx_val: usize,
//     ) {
//         if key_idx_val <= self.dig_inputs.len() {
//                     if set_idx < self.v_multi.len() {
//                         self.v_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.f_multi.len() {
//                         self.f_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.r_multi.len() {
//                         self.r_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
//                     }
//                 }
//                     if set_idx < self.c_multi.len() {
//                         self.c_multi[set_idx].hold_remove_key(h_kind, key_idx_val)
//                     }
//                 }
//             }
//         }
//         self.check_gutdelta_lengths();
//         self.check_digitalref_invariants();
//     }
}