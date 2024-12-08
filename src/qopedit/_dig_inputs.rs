use crate::*;
use winit::keyboard::KeyCode;

impl Qop<Edit> {
    pub fn dig_inputs_insert_k(&mut self, key_code: KeyCode) {
        if !self.dig_inputs.contains(&key_code) {
            self.dig_inputs.push(key_code)
        }
    }
    fn dig_inputs_global_vec_manip(&mut self, operation: impl Fn(&mut Vec<usize>)) {
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
        for set in 0..self.c_multi.len() {
            self.c_multi[set].all_key_idx_vecs(&operation);
        }
    }
    pub fn dig_inputs_purge_dig(&mut self, key_code: KeyCode) {
        for i in 0..self.dig_inputs.len() {
            if self.dig_inputs[i] == key_code {
                let dig_idx_purge = |key_idx_vec: &mut Vec<usize>| {
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
                Qop::dig_inputs_global_vec_manip(self, dig_idx_purge);
                self.dig_inputs.remove(i);
                break;
            }
        }
    }
    pub fn dig_inputs_swap_idxs(&mut self, kc1: KeyCode, kc2: KeyCode, swap_all_fields: bool) {
        // This function swaps 2 digital input values for 2 existing inputs in the dig_inputs field.
        // swap_all_fields == true -> swaps the nodes of 2 existing keys in all of rest of the Qop's fields,  so all of those affected usize values would be now pointing to the same keys as before.
        // swap_all_fields == false -> leaves all of the rest of the Qop's fields alone, so all of those affected usize values would be now pointing to swapped keys.
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
            if swap_all_fields {
                let k_idxs_swap = |k_idx_vec: &mut Vec<usize>| {
                    k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                        if *k == i1.unwrap() {
                            *k = i2.unwrap();
                        } else if *k == i2.unwrap() {
                            *k = i1.unwrap();
                        }
                    });
                };
                Qop::dig_inputs_global_vec_manip(self, k_idxs_swap);
            }
        }
    }
    pub fn dig_inputs_change_idx_to(&mut self, kc_old: KeyCode, kc_new: KeyCode) {
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
            Qop::dig_inputs_global_vec_manip(self, k_idx_update);
        }
    }
    pub(crate) fn check_digitalref_invariants(&self) {
        let message: &str = " is an index to an OOB Digital Input!";
        for g in 0..self.guts.len() {
            for t in 0..self.guts[g].gut_triggers.togs.len() {
                assert!(
                    self.guts[g].gut_triggers.togs[t] < self.dig_inputs.len(),
                    "self.guts[{g}].gut_triggers.togs[{t}]{message}"
                )
            }
            for tg in 0..self.guts[g].trnsp_gut.len() {
                for t in 0..self.guts[g].trnsp_gut[tg].triggers.len() {
                    assert!(
                        self.guts[g].trnsp_gut[tg].triggers[t] < self.dig_inputs.len(),
                        "self.guts[{g}].trnsp_gut[{tg}].triggers[{t}]{message}"
                    )
                }
            }

            for set in 0..self.guts[g].v_one.len() {
                self.guts[g].v_one[set].check_digitalref_invariants(
                    format!("self.guts[{g}].v_one[{set}]").as_str(),
                    message,
                    self.dig_inputs.len(),
                )
            }
            for set in 0..self.guts[g].f_one.len() {
                self.guts[g].f_one[set].check_digitalref_invariants(
                    format!("self.guts[{g}].f_one[{set}]").as_str(),
                    message,
                    self.dig_inputs.len(),
                )
            }
            for set in 0..self.guts[g].c_one.len() {
                self.guts[g].c_one[set].check_digitalref_invariants(
                    format!("self.guts[{g}].c_one[{set}]").as_str(),
                    message,
                    self.dig_inputs.len(),
                )
            }
        }

        self.gut_holds
            .check_digitalref_invariants("self.gutholds", message, self.dig_inputs.len());

        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_digitalref_invariants(
                format!("self.v_multi[{set}]").as_str(),
                message,
                self.dig_inputs.len(),
            )
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_digitalref_invariants(
                format!("self.f_multi[{set}]").as_str(),
                message,
                self.dig_inputs.len(),
            )
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_digitalref_invariants(
                format!("self.c_multi[{set}]").as_str(),
                message,
                self.dig_inputs.len(),
            )
        }
    }
}

impl<T, U> VFSet<T, U> {
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
    pub(crate) fn check_digitalref_invariants(
        &self,
        leading_str: &str,
        message: &str,
        dig_vec_len: usize,
    ) {
        for b in 0..self.buttons.len() {
            for t in 0..self.buttons[b].togs.len() {
                assert!(
                    self.buttons[b].togs[t] < dig_vec_len,
                    "{leading_str}.buttons[{b}].togs[{t}]{message}"
                )
            }
            for to in 0..self.buttons[b].trnsp_one.len() {
                for t in 0..self.buttons[b].trnsp_one[to].triggers.len() {
                    assert!(
                        self.buttons[b].trnsp_one[to].triggers[t] < dig_vec_len,
                        "{leading_str}.buttons[{b}].trnsp_one[{to}].triggers[{t}]{message}"
                    );
                }
            }
        }
        for ta in 0..self.trnsp_all.len() {
            for t in 0..self.trnsp_all[ta].triggers.len() {
                assert!(
                    self.trnsp_all[ta].triggers[t] < dig_vec_len,
                    "{leading_str}.trnsp_all[{ta}].triggers[{t}]{message}"
                );
            }
        }
        self.holds.check_digitalref_invariants(
            format!("{leading_str}.holds").as_str(),
            message,
            dig_vec_len,
        )
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
    pub(crate) fn check_digitalref_invariants(
        &self,
        leading_str: &str,
        message: &str,
        dig_vec_len: usize,
    ) {
        for b in 0..self.buttons.len() {
            for t in 0..self.buttons[b].togs.len() {
                assert!(
                    self.buttons[b].togs[t] < dig_vec_len,
                    "{leading_str}.buttons[{b}].togs[{t}]{message}"
                )
            }
        }
        for c in 0..self.combos.len() {
            for to in 0..self.combos[c].trnsp_one.len() {
                for t in 0..self.combos[c].trnsp_one[to].triggers.len() {
                    assert!(
                        self.combos[c].trnsp_one[to].triggers[t] < dig_vec_len,
                        "{leading_str}.combos[{c}].trnsp_one[{to}].triggers[{t}]{message}"
                    )
                }
            }
        }
        for ta in 0..self.trnsp_all.len() {
            for t in 0..self.trnsp_all[ta].triggers.len() {
                assert!(
                    self.trnsp_all[ta].triggers[t] < dig_vec_len,
                    "{leading_str}.trnsp_all[{ta}].triggers[{t}]{message}"
                )
            }
        }
        self.holds.check_digitalref_invariants(
            format!("{leading_str}.holds").as_str(),
            message,
            dig_vec_len,
        )
    }
}

impl HoldBtns {
    pub(crate) fn check_digitalref_invariants(
        &self,
        leading_str: &str,
        message: &str,
        dig_vec_len: usize,
    ) {
        for sus in 0..self.sustain.togs.len() {
            assert!(
                self.sustain.togs[sus] < dig_vec_len,
                "{leading_str}.sustain.togs[{sus}]{message}"
            )
        }
        for isus in 0..self.inv_sustain.togs.len() {
            assert!(
                self.inv_sustain.togs[isus] < dig_vec_len,
                "{leading_str}.inv_sustain.togs[{isus}]{message}"
            )
        }
        for sos in 0..self.sostenuto.togs.len() {
            assert!(
                self.sostenuto.togs[sos] < dig_vec_len,
                "{leading_str}.sostenuto.togs[{sos}]{message}"
            )
        }
        for isos in 0..self.inv_sostenuto.togs.len() {
            assert!(
                self.inv_sostenuto.togs[isos] < dig_vec_len,
                "{leading_str}.inv_sostenuto.togs[{isos}]{message}"
            )
        }
    }
}
