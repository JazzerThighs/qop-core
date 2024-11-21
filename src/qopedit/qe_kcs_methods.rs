use super::QopEdit;
use winit::keyboard::KeyCode;

impl QopEdit {
    pub fn kcs_insert_k(&mut self, key_code: KeyCode) {
        if !self.key_codes.contains(&key_code) {
            self.key_codes.push(key_code)
        };
    }
    pub fn kcs_swap_labels(&mut self, kc1: KeyCode, kc2: KeyCode) {
        // swaps only the keycode labels for 2 existing keys, which leaves alone all of the nodes' usize values in the rest of the QopEdit.
        let (mut i1, mut i2): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..self.key_codes.len() {
            if self.key_codes[i] == kc1 {
                i1 = Some(i);
            } else if self.key_codes[i] == kc2 {
                i2 = Some(i);
            }
        }
        if i1.is_some() && i2.is_some() {
            (self.key_codes[i1.unwrap()], self.key_codes[i2.unwrap()]) =
                (self.key_codes[i2.unwrap()], self.key_codes[i1.unwrap()]);
        }
    }
    fn kcs_global_vec_manip(&mut self, operation: impl Fn(&mut Vec<usize>)) {
        for g in 0..self.guts.len() {
            operation(&mut self.guts[g].gut_triggers.togs);
            for t in 0..self.guts[g].trnsp_gut.len() {
                operation(&mut self.guts[g].trnsp_gut[t].triggers);
            }
        }
        operation(&mut self.gut_holds.sustain.togs);
        operation(&mut self.gut_holds.inv_sustain.togs);
        operation(&mut self.gut_holds.sostenuto.togs);
        operation(&mut self.gut_holds.inv_sostenuto.togs);

        for set in 0..self.valve_sets.len() {
            self.valve_sets[set].all_key_idx_vecs(&operation);
        }
        for set in 0..self.fret_sets.len() {
            self.fret_sets[set].all_key_idx_vecs(&operation);
        }
        for set in 0..self.radio_sets.len() {
            self.radio_sets[set].all_key_idx_vecs(&operation);
        }
        for set in 0..self.aero_sets.len() {
            self.aero_sets[set].all_key_idx_vecs(&operation);
        }
    }
    pub fn kcs_remove_k(&mut self, key_code: KeyCode) {
        for i in 0..self.key_codes.len() {
            if self.key_codes[i] == key_code {
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
                QopEdit::kcs_global_vec_manip(self, k_idx_remove);
                self.key_codes.remove(i);
                break;
            }
        }
    }
    pub fn kcs_swap_idxs(&mut self, kc1: KeyCode, kc2: KeyCode) {
        // swaps the nodes of 2 existing keys in all of the plucks' and sets' various fields.
        let (mut i1, mut i2): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..self.key_codes.len() {
            if self.key_codes[i] == kc1 {
                i1 = Some(i);
            } else if self.key_codes[i] == kc2 {
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
            QopEdit::kcs_global_vec_manip(self, k_idxs_swap);
        }
    }
    pub fn kcs_change_idx_to(&mut self, kc_old: KeyCode, kc_new: KeyCode) {
        let (mut i1, mut i2): (Option<usize>, Option<usize>) = (None, None);
        for i in 0..self.key_codes.len() {
            if self.key_codes[i] == kc_old {
                i1 = Some(i);
            } else if self.key_codes[i] == kc_new {
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
            QopEdit::kcs_global_vec_manip(self, k_idx_update);
        }
    }
}
