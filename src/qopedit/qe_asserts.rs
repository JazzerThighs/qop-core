use crate::*;

impl Qop {
    pub(super) fn check_gutdelta_lengths(&self){
        let message: &str = " does not have the same length as self.guts!";
        operate_on_all_multi!(self, [v_multi, f_multi, r_multi, c_multi], {
            field[set].check_gutdelta_lengths(
                &format!("self.{}[{}]", field_name, set),
                message,
                self.guts.len(),
            );
        });
    }

    // pub(super) fn check_gutdelta_lengths(&self) {
    //     let message: &str = " does not have the same length as self.guts!";
    //     for set in 0..self.v_multi.len() {
    //         self.v_multi[set].check_gutdelta_lengths(format!("self.v_multi[{set}]").as_str(), message, self.guts.len())
    //     }
    //     for set in 0..self.f_multi.len() {
    //         self.f_multi[set].check_gutdelta_lengths(format!("self.f_multi[{set}]").as_str(), message, self.guts.len())
    //     }
    //     for set in 0..self.r_multi.len() {
    //         self.r_multi[set].check_gutdelta_lengths(format!("self.r_multi[{set}]").as_str(), message, self.guts.len())
    //     }
    //     for set in 0..self.c_multi.len() {
    //         self.c_multi[set].check_gutdelta_lengths(format!("self.c_multi[{set}]").as_str(), message, self.guts.len())
    //     }
    // }

    pub(super) fn check_digitalref_invariants(&self) {
        let message: &str = " is an index to an OOB Digital Input!";
        for g in 0..self.guts.len() {
            for t in 0..self.guts[g].gut_triggers.togs.len() {
                assert!(self.guts[g].gut_triggers.togs[t] < self.key_codes.len(), "self.guts[{g}].gut_triggers.togs[{t}]{message}")
            }
            for tg in 0..self.guts[g].trnsp_gut.len() {
                for t in 0..self.guts[g].trnsp_gut[tg].triggers.len() {
                    assert!(self.guts[g].trnsp_gut[tg].triggers[t] < self.key_codes.len(), "self.guts[{g}].trnsp_gut[{tg}].triggers[{t}]{message}")
                }
            }
        }
        self.gut_holds.check_digitalref_invariants("self.gutholds", message, self.key_codes.len());

        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_digitalref_invariants(format!("self.v_multi[{set}]").as_str(), message, self.key_codes.len())
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_digitalref_invariants(format!("self.f_multi[{set}]").as_str(), message, self.key_codes.len())
        }
        for set in 0..self.r_multi.len() {
            self.r_multi[set].check_digitalref_invariants(format!("self.r_multi[{set}]").as_str(), message, self.key_codes.len())
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_digitalref_invariants(format!("self.c_multi[{set}]").as_str(), message, self.key_codes.len())
        }
    }
}

impl HoldBtns {
    pub(super) fn check_digitalref_invariants(&self, leading_str: &str, message: &str, dig_vec_len: usize) {
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

impl MultiSet {
    pub(super) fn check_gutdelta_lengths(&self, leading_str: &str, message: &str, gut_len: usize) {
        for b in 0..self.buttons.len() {
            assert_eq!(self.buttons[b].i_deltas.len(), gut_len, "{leading_str}.buttons[{b}].i_deltas{message}");
            assert_eq!(self.buttons[b].x_deltas.len(), gut_len, "{leading_str}.buttons[{b}].x_deltas{message}");
            for to in 0..self.buttons[b].trnsp_one.len() {
                assert_eq!(self.buttons[b].trnsp_one[to].i_deltas.len(), gut_len, "{leading_str}.buttons[{b}].trnsp_one[{to}].i_deltas{message}");
                assert_eq!(self.buttons[b].trnsp_one[to].x_deltas.len(), gut_len, "{leading_str}.buttons[{b}].trnsp_one[{to}].x_deltas{message}")
            }
        }
        for ta in 0..self.trnsp_all.len() {
            assert_eq!(self.trnsp_all[ta].i_deltas.len(), gut_len, "{leading_str}.trnsp_all[{ta}].i_deltas{message}");
            assert_eq!(self.trnsp_all[ta].x_deltas.len(), gut_len, "{leading_str}.trnsp_all[{ta}].x_deltas{message}");
        }
    }

    pub(super) fn check_digitalref_invariants(&self, leading_str: &str, message: &str, dig_vec_len: usize) {
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

impl MultiComboSet {
    pub(super) fn check_gutdelta_lengths(&self, leading_str: &str, message: &str, gut_len: usize) {
        for c in 0..self.combos.len() {
            assert_eq!(self.combos[c].i_deltas.len(), gut_len, "{leading_str}.combos[{c}].i_deltas{message}");
            assert_eq!(self.combos[c].x_deltas.len(), gut_len, "{leading_str}.combos[{c}].x_deltas{message}");
            for to in 0..self.combos[c].trnsp_one.len() {
                assert_eq!(self.combos[c].trnsp_one[to].i_deltas.len(), gut_len, "{leading_str}.combos[{c}].trnsp_one[{to}].i_deltas{message}");
                assert_eq!(self.combos[c].trnsp_one[to].x_deltas.len(), gut_len, "{leading_str}.combos[{c}].trnsp_one[{to}].x_deltas{message}")
            }
        }
        for ta in 0..self.trnsp_all.len() {
            assert_eq!(self.trnsp_all[ta].i_deltas.len(), gut_len, "{leading_str}.trnsp_all[{ta}].i_deltas{message}");
            assert_eq!(self.trnsp_all[ta].x_deltas.len(), gut_len, "{leading_str}.trnsp_all[{ta}].x_deltas{message}")
        }
    }

    pub(super) fn check_digitalref_invariants(&self, leading_str: &str, message: &str, dig_vec_len: usize) {
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