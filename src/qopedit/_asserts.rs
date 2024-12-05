use crate::*;

impl Qop {
    pub(crate) fn check_multi_delta_lengths(&self) {
        let message: &str = " does not have the same length as self.guts!";
        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_multi_delta_lengths(format!("self.v_multi[{set}]").as_str(), message, self.guts.len())
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_multi_delta_lengths(format!("self.f_multi[{set}]").as_str(), message, self.guts.len())
        }
        for set in 0..self.r_multi.len() {
            self.r_multi[set].check_multi_delta_lengths(format!("self.r_multi[{set}]").as_str(), message, self.guts.len())
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_multi_delta_lengths(format!("self.c_multi[{set}]").as_str(), message, self.guts.len())
        }
    }

    pub(crate) fn check_digitalref_invariants(&self) {
        let message: &str = " is an index to an OOB Digital Input!";
        for g in 0..self.guts.len() {
            for t in 0..self.guts[g].gut_triggers.togs.len() {
                assert!(self.guts[g].gut_triggers.togs[t] < self.dig_inputs.len(), "self.guts[{g}].gut_triggers.togs[{t}]{message}")
            }
            for tg in 0..self.guts[g].trnsp_gut.len() {
                for t in 0..self.guts[g].trnsp_gut[tg].triggers.len() {
                    assert!(self.guts[g].trnsp_gut[tg].triggers[t] < self.dig_inputs.len(), "self.guts[{g}].trnsp_gut[{tg}].triggers[{t}]{message}")
                }
            }

            for set in 0..self.guts[g].v_one.len() {
                self.guts[g].v_one[set].check_digitalref_invariants(format!("self.guts[{g}].v_one[{set}]").as_str(), message, self.dig_inputs.len())
            }
            for set in 0..self.guts[g].f_one.len() {
                self.guts[g].f_one[set].check_digitalref_invariants(format!("self.guts[{g}].f_one[{set}]").as_str(), message, self.dig_inputs.len())
            }
            for set in 0..self.guts[g].r_one.len() {
                self.guts[g].r_one[set].check_digitalref_invariants(format!("self.guts[{g}].r_one[{set}]").as_str(), message, self.dig_inputs.len())
            }
            for set in 0..self.guts[g].c_one.len() {
                self.guts[g].c_one[set].check_digitalref_invariants(format!("self.guts[{g}].c_one[{set}]").as_str(), message, self.dig_inputs.len())
            }
        }
        
        self.gut_holds.check_digitalref_invariants("self.gutholds", message, self.dig_inputs.len());

        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_digitalref_invariants(format!("self.v_multi[{set}]").as_str(), message, self.dig_inputs.len())
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_digitalref_invariants(format!("self.f_multi[{set}]").as_str(), message, self.dig_inputs.len())
        }
        for set in 0..self.r_multi.len() {
            self.r_multi[set].check_digitalref_invariants(format!("self.r_multi[{set}]").as_str(), message, self.dig_inputs.len())
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_digitalref_invariants(format!("self.c_multi[{set}]").as_str(), message, self.dig_inputs.len())
        }
    }
}

impl HoldBtns {
    pub(crate) fn check_digitalref_invariants(&self, leading_str: &str, message: &str, dig_vec_len: usize) {
        for sus in 0..self.sustain.togs.len() {
            assert!(self.sustain.togs[sus] < dig_vec_len, "{leading_str}.sustain.togs[{sus}]{message}")
        }
        for isus in 0..self.inv_sustain.togs.len() {
            assert!(self.inv_sustain.togs[isus] < dig_vec_len, "{leading_str}.inv_sustain.togs[{isus}]{message}")
        }
        for sos in 0..self.sostenuto.togs.len() {
            assert!(self.sostenuto.togs[sos] < dig_vec_len, "{leading_str}.sostenuto.togs[{sos}]{message}")
        }
        for isos in 0..self.inv_sostenuto.togs.len() {
            assert!(self.inv_sostenuto.togs[isos] < dig_vec_len, "{leading_str}.inv_sostenuto.togs[{isos}]{message}")
        }
    }
}

impl VFRSet<Vec<i64>, Vec<f64>> {
    pub(crate) fn check_multi_delta_lengths(&self, leading_str: &str, message: &str, gut_len: usize) {
        for b in 0..self.buttons.len() {
            assert_eq!(self.buttons[b].i_delta.len(), gut_len, "{leading_str}.buttons[{b}].i_delta{message}");
            assert_eq!(self.buttons[b].x_delta.len(), gut_len, "{leading_str}.buttons[{b}].x_delta{message}");
            for to in 0..self.buttons[b].trnsp_one.len() {
                assert_eq!(self.buttons[b].trnsp_one[to].i_delta.len(), gut_len, "{leading_str}.buttons[{b}].trnsp_one[{to}].i_delta{message}");
                assert_eq!(self.buttons[b].trnsp_one[to].x_delta.len(), gut_len, "{leading_str}.buttons[{b}].trnsp_one[{to}].x_delta{message}")
            }
        }
        for ta in 0..self.trnsp_all.len() {
            assert_eq!(self.trnsp_all[ta].i_delta.len(), gut_len, "{leading_str}.trnsp_all[{ta}].i_delta{message}");
            assert_eq!(self.trnsp_all[ta].x_delta.len(), gut_len, "{leading_str}.trnsp_all[{ta}].x_delta{message}");
        }
    }
}

impl<T, U> VFRSet<T, U> {
    pub(crate) fn check_digitalref_invariants(&self, leading_str: &str, message: &str, dig_vec_len: usize) {
        for b in 0..self.buttons.len() {
            for t in 0..self.buttons[b].togs.len() {
                assert!(self.buttons[b].togs[t] < dig_vec_len, "{leading_str}.buttons[{b}].togs[{t}]{message}")
            }
            for to in 0..self.buttons[b].trnsp_one.len() {
                for t in 0..self.buttons[b].trnsp_one[to].triggers.len() {
                    assert!(self.buttons[b].trnsp_one[to].triggers[t] < dig_vec_len, "{leading_str}.buttons[{b}].trnsp_one[{to}].triggers[{t}]{message}");
                }
            }
        }
        for ta in 0..self.trnsp_all.len() {
            for t in 0..self.trnsp_all[ta].triggers.len() {
                assert!(self.trnsp_all[ta].triggers[t] < dig_vec_len, "{leading_str}.trnsp_all[{ta}].triggers[{t}]{message}");
            }
        }
        self.holds.check_digitalref_invariants(format!("{leading_str}.holds").as_str(), message, dig_vec_len)
    }
}

impl ComboSet<Vec<i64>, Vec<f64>> {
    pub(crate) fn check_multi_delta_lengths(&self, leading_str: &str, message: &str, gut_len: usize) {
        for c in 0..self.combos.len() {
            assert_eq!(self.combos[c].i_delta.len(), gut_len, "{leading_str}.combos[{c}].i_delta{message}");
            assert_eq!(self.combos[c].x_delta.len(), gut_len, "{leading_str}.combos[{c}].x_delta{message}");
            for to in 0..self.combos[c].trnsp_one.len() {
                assert_eq!(self.combos[c].trnsp_one[to].i_delta.len(), gut_len, "{leading_str}.combos[{c}].trnsp_one[{to}].i_delta{message}");
                assert_eq!(self.combos[c].trnsp_one[to].x_delta.len(), gut_len, "{leading_str}.combos[{c}].trnsp_one[{to}].x_delta{message}")
            }
        }
        for ta in 0..self.trnsp_all.len() {
            assert_eq!(self.trnsp_all[ta].i_delta.len(), gut_len, "{leading_str}.trnsp_all[{ta}].i_delta{message}");
            assert_eq!(self.trnsp_all[ta].x_delta.len(), gut_len, "{leading_str}.trnsp_all[{ta}].x_delta{message}")
        }
    }
}

impl<T, U> ComboSet<T, U> {
    pub(crate) fn check_digitalref_invariants(&self, leading_str: &str, message: &str, dig_vec_len: usize) {
        for b in 0..self.buttons.len() {
            for t in 0..self.buttons[b].togs.len() {
                assert!(self.buttons[b].togs[t] < dig_vec_len, "{leading_str}.buttons[{b}].togs[{t}]{message}")
            }
        }
        for c in 0..self.combos.len() {
            for to in 0..self.combos[c].trnsp_one.len() {
                for t in 0..self.combos[c].trnsp_one[to].triggers.len() {
                    assert!(self.combos[c].trnsp_one[to].triggers[t] < dig_vec_len, "{leading_str}.combos[{c}].trnsp_one[{to}].triggers[{t}]{message}")
                }
            }
        }
        for ta in 0..self.trnsp_all.len() {
            for t in 0..self.trnsp_all[ta].triggers.len() {
                assert!(self.trnsp_all[ta].triggers[t] < dig_vec_len, "{leading_str}.trnsp_all[{ta}].triggers[{t}]{message}")
            }
        }
        self.holds.check_digitalref_invariants(format!("{leading_str}.holds").as_str(), message, dig_vec_len)
    }
}