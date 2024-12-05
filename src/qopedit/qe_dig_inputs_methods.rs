use crate::*;
use winit::keyboard::KeyCode;

impl Qop<Edit> {
    pub fn kcs_insert_k(&mut self, key_code: KeyCode) {
        if !self.dig_inputs.contains(&key_code) {
            self.dig_inputs.push(key_code)
        }
    }
    pub fn kcs_swap_labels(&mut self, kc1: KeyCode, kc2: KeyCode) {
        // swaps only the keycode labels for 2 existing keys, which leaves alone all of the nodes' usize values in the rest of the QopEdit.
        let (mut i1, mut i2): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..self.dig_inputs.len() {
            if self.dig_inputs[i] == kc1 {
                i1 = Some(i);
            } else if self.dig_inputs[i] == kc2 {
                i2 = Some(i);
            }
        }
        if i1.is_some() && i2.is_some() {
            (self.dig_inputs[i1.unwrap()], self.dig_inputs[i2.unwrap()]) =
                (self.dig_inputs[i2.unwrap()], self.dig_inputs[i1.unwrap()]);
        }
    }
    fn kcs_global_vec_manip(&mut self, operation: impl Fn(&mut Vec<usize>)) {
        for g in 0..self.guts.len() {
            operation(&mut self.guts[g].gut_triggers.togs);
            for tg in 0..self.guts[g].trnsp_gut.len() {
                operation(&mut self.guts[g].trnsp_gut[tg].triggers);
            }
            for set in 0..self.guts[g].v_one.len() {
                self.guts[g].v_one[set].all_key_idx_vecs(&operation);
            }
            for set in 0..self.guts[g].f_one.len() {
                self.guts[g].f_one[set].all_key_idx_vecs(&operation);
            }
            for set in 0..self.guts[g].r_one.len() {
                self.guts[g].r_one[set].all_key_idx_vecs(&operation);
            }
            for set in 0..self.guts[g].c_one.len() {
                self.guts[g].c_one[set].all_key_idx_vecs(&operation);
            }
        }
        operation(&mut self.gut_holds.sustain.togs);
        operation(&mut self.gut_holds.inv_sustain.togs);
        operation(&mut self.gut_holds.sostenuto.togs);
        operation(&mut self.gut_holds.inv_sostenuto.togs);
        for set in 0..self.v_multi.len() {
            self.v_multi[set].all_key_idx_vecs(&operation);
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].all_key_idx_vecs(&operation);
        }
        for set in 0..self.r_multi.len() {
            self.r_multi[set].all_key_idx_vecs(&operation);
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].all_key_idx_vecs(&operation);
        }
    }
    pub fn kcs_remove_k(&mut self, key_code: KeyCode) {
        for i in 0..self.dig_inputs.len() {
            if self.dig_inputs[i] == key_code {
                let k_idx_remove = |key_idx_vec: &mut Vec<usize>| {
                    key_idx_vec.retain_mut(|k: &mut usize| -> bool {
                        if *k < i {
                            return true;
                        } else if *k == i {
                            return false;
                        } else {
                            *k -= 1;
                            return true;
                        }
                    })
                };
                Qop::kcs_global_vec_manip(self, k_idx_remove);
                self.dig_inputs.remove(i);
                break;
            }
        }
    }
    pub fn kcs_swap_idxs(&mut self, kc1: KeyCode, kc2: KeyCode) {
        // swaps the nodes of 2 existing keys in all of the plucks' and sets' various fields.
        let (mut i1, mut i2): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..self.dig_inputs.len() {
            if self.dig_inputs[i] == kc1 {
                i1 = Some(i);
            } else if self.dig_inputs[i] == kc2 {
                i2 = Some(i);
            }
        }
        if i1.is_some() && i2.is_some() {
            let k_idxs_swap = |k_idx_vec: &mut Vec<usize>| {
                k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                    if *k == i1.unwrap() {
                        *k = i2.unwrap();
                    } else if *k == i2.unwrap() {
                        *k = i1.unwrap();
                    }
                });
            };
            Qop::kcs_global_vec_manip(self, k_idxs_swap);
        }
    }
    pub fn kcs_change_idx_to(&mut self, kc_old: KeyCode, kc_new: KeyCode) {
        let (mut i1, mut i2): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..self.dig_inputs.len() {
            if self.dig_inputs[i] == kc_old {
                i1 = Some(i);
            } else if self.dig_inputs[i] == kc_new {
                i2 = Some(i);
            }
        }
        if i1.is_some() && i2.is_some() {
            let k_idx_update = |k_idx_vec: &mut Vec<usize>| {
                k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                    if *k == i1.unwrap() {
                        *k = i2.unwrap();
                    }
                });
                k_idx_vec.sort();
                k_idx_vec.dedup();
            };
            Qop::kcs_global_vec_manip(self, k_idx_update);
        }
    }
}

impl<T, U> VFRSet<T, U> {
    pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
            for to in 0..self.buttons[b].trnsp_one.len() {
                vec_closure(&mut self.buttons[b].trnsp_one[to].triggers);
            }
        }
        for ta in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[ta].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
}

impl<T, U> ComboSet<T, U> {
    pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
        }
        for c in 0..self.combos.len() {
            for to in 0..self.combos[c].trnsp_one.len() {
                vec_closure(&mut self.combos[c].trnsp_one[to].triggers);
            }
        }
        for ta in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[ta].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
}