use nestify::nest;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct QopEdit {
        pub(crate) key_codes: Vec<KeyCode>,
        pub(crate) guts: Vec< pub(crate) struct Gut {
            pub(crate) gut_triggers: pub(crate) struct BtnTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
            },
            pub(crate) index_out: usize,
            pub(crate) extra_out: f64,
            pub(crate) trnsp_gut: Vec<pub(crate) struct TrnspGut {
                pub(crate) triggers: Vec<usize>,
                pub(crate) i_delta: i64,
                pub(crate) x_delta: f64,
            }>,
            pub(crate) tp_i_mem: i64,
            pub(crate) tp_x_mem: f64,
        }>,
        pub(crate) gut_holds: pub(crate) struct HoldBtns {
            pub(crate) sustain: BtnTog,
            pub(crate) inv_sustain: BtnTog,
            pub(crate) sostenuto: BtnTog,
            pub(crate) inv_sostenuto: BtnTog,
        },
        pub(crate) valve_sets: Vec<
            #[derive(Default)]/
            pub(crate) struct IndvSet {
                pub(crate) buttons: Vec<pub(crate) struct DeltaTog {
                    pub(crate) togs: Vec<usize>,
                    pub(crate) pressed: bool,
                    pub(crate) i_deltas: Vec<i64>,
                    pub(crate) x_deltas: Vec<f64>,
                    pub(crate) trnsp_one: Vec<pub(crate) struct TrnspSet {
                        triggers: Vec<usize>,
                        i_deltas: Vec<i64>,
                        x_deltas: Vec<f64>,
                    }>,
                    pub(crate) tp_i_mem: Vec<i64>,
                    pub(crate) tp_x_mem: Vec<f64>,
                }>,
                pub(crate) trnsp_all: Vec<TrnspSet>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
                pub(crate) holds: HoldBtns,
                pub(crate) max_pressed: u8,
                pub(crate) min_pressed: u8,
            }>,
        pub(crate) fret_sets: Vec<IndvSet>,
        pub(crate) radio_sets: Vec<IndvSet>,
        pub(crate) combo_sets: Vec<
            #[derive(Default)]/
            pub(crate) struct ComboSet {
                pub(crate) buttons: Vec<BtnTog>,
                pub(crate) combos: Vec<pub(crate) struct Combo {
                    pub(crate) combo: Vec<bool>,
                    pub(crate) i_deltas: Vec<i64>,
                    pub(crate) x_deltas: Vec<f64>,
                    pub(crate) trnsp_one: Vec<TrnspSet>,
                    pub(crate) tp_i_mem: Vec<i64>,
                    pub(crate) tp_x_mem: Vec<f64>,
                }>,
                pub(crate) holds: HoldBtns,
                pub(crate) trnsp_all: Vec<TrnspSet>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
        }>,
    }
}

impl QopEdit {
    pub fn new() -> Self {
        return Self {
            guts: vec![Gut::default()],
            ..Default::default()
        };
    }

    pub fn check_gutdelta_lengths(&self) {
        for set in 0..self.valve_sets.len() {
            for b in 0..self.valve_sets[set].buttons.len() {
                assert!(self.valve_sets[set].buttons[b].i_deltas.len() == self.guts.len(), "self.valve_sets[{set}].buttons[{b}].i_deltas does not have the same length as self.guts!");
                assert!(self.valve_sets[set].buttons[b].x_deltas.len() == self.guts.len(), "self.valve_sets[{set}].buttons[{b}].x_deltas does not have the same length as self.guts!");
                for to in 0..self.valve_sets[set].buttons[b].trnsp_one.len() {
                    assert!(self.valve_sets[set].buttons[b].trnsp_one[to].i_deltas.len() == self.guts.len(), "self.valve_sets[{set}].buttons[{b}].trnsp_one[{to}].i_deltas does not have the same length as self.guts!");
                    assert!(self.valve_sets[set].buttons[b].trnsp_one[to].x_deltas.len() == self.guts.len(), "self.valve_sets[{set}].buttons[{b}].trnsp_one[{to}].x_deltas does not have the same length as self.guts!");
                }
            }
            for ta in 0..self.valve_sets[set].trnsp_all.len() {
                assert!(self.valve_sets[set].trnsp_all[ta].i_deltas.len() == self.guts.len(), "self.valve_sets[{set}].trnsp_all[{ta}].i_deltas does not have the same length as self.guts!");
                assert!(self.valve_sets[set].trnsp_all[ta].x_deltas.len() == self.guts.len(), "self.valve_sets[{set}].trnsp_all[{ta}].x_deltas does not have the same length as self.guts!");
            }
        }
        for set in 0..self.fret_sets.len() {
            for b in 0..self.fret_sets[set].buttons.len() {
                assert!(self.fret_sets[set].buttons[b].i_deltas.len() == self.guts.len(), "self.fret_sets[{set}].buttons[{b}].i_deltas does not have the same length as self.guts!");
                assert!(self.fret_sets[set].buttons[b].x_deltas.len() == self.guts.len(), "self.fret_sets[{set}].buttons[{b}].x_deltas does not have the same length as self.guts!");
                for to in 0..self.fret_sets[set].buttons[b].trnsp_one.len() {
                    assert!(self.fret_sets[set].buttons[b].trnsp_one[to].i_deltas.len() == self.guts.len(), "self.fret_sets[{set}].buttons[{b}].trnsp_one[{to}].i_deltas does not have the same length as self.guts!");
                    assert!(self.fret_sets[set].buttons[b].trnsp_one[to].x_deltas.len() == self.guts.len(), "self.fret_sets[{set}].buttons[{b}].trnsp_one[{to}].x_deltas does not have the same length as self.guts!");
                }
            }
            for ta in 0..self.fret_sets[set].trnsp_all.len() {
                assert!(self.fret_sets[set].trnsp_all[ta].i_deltas.len() == self.guts.len(), "self.fret_sets[{set}].trnsp_all[{ta}].i_deltas does not have the same length as self.guts!");
                assert!(self.fret_sets[set].trnsp_all[ta].x_deltas.len() == self.guts.len(), "self.fret_sets[{set}].trnsp_all[{ta}].x_deltas does not have the same length as self.guts!");
            }
        }
        for set in 0..self.radio_sets.len() {
            for b in 0..self.radio_sets[set].buttons.len() {
                assert!(self.radio_sets[set].buttons[b].i_deltas.len() == self.guts.len(), "self.radio_sets[{set}].buttons[{b}].i_deltas does not have the same length as self.guts!");
                assert!(self.radio_sets[set].buttons[b].x_deltas.len() == self.guts.len(), "self.radio_sets[{set}].buttons[{b}].x_deltas does not have the same length as self.guts!");
                for to in 0..self.radio_sets[set].buttons[b].trnsp_one.len() {
                    assert!(self.radio_sets[set].buttons[b].trnsp_one[to].i_deltas.len() == self.guts.len(), "self.radio_sets[{set}].buttons[{b}].trnsp_one[{to}].i_deltas does not have the same length as self.guts!");
                    assert!(self.radio_sets[set].buttons[b].trnsp_one[to].x_deltas.len() == self.guts.len(), "self.radio_sets[{set}].buttons[{b}].trnsp_one[{to}].x_deltas does not have the same length as self.guts!");
                }
            }
            for ta in 0..self.radio_sets[set].trnsp_all.len() {
                assert!(self.radio_sets[set].trnsp_all[ta].i_deltas.len() == self.guts.len(), "self.radio_sets[{set}].trnsp_all[{ta}].i_deltas does not have the same length as self.guts!");
                assert!(self.radio_sets[set].trnsp_all[ta].x_deltas.len() == self.guts.len(), "self.radio_sets[{set}].trnsp_all[{ta}].x_deltas does not have the same length as self.guts!");
            }
        }
        for set in 0..self.combo_sets.len() {
            for c in 0..self.combo_sets[set].combos.len() {
                assert!(self.combo_sets[set].combos[c].i_deltas.len() == self.guts.len(), "self.combo_sets[{set}].combos[{c}].i_deltas does not have the same length as self.guts!");
                assert!(self.combo_sets[set].combos[c].x_deltas.len() == self.guts.len(), "self.combo_sets[{set}].combos[{c}].x_deltas does not have the same length as self.guts!");
                for to in 0..self.combo_sets[set].combos[c].trnsp_one.len() {
                    assert!(self.combo_sets[set].combos[c].trnsp_one[to].i_deltas.len() == self.guts.len(), "self.combo_sets[{set}].combos[{c}].trnsp_one[{to}].i_deltas does not have the same length as self.guts!");
                    assert!(self.combo_sets[set].combos[c].trnsp_one[to].x_deltas.len() == self.guts.len(), "self.combo_sets[{set}].combos[{c}].trnsp_one[{to}].x_deltas does not have the same length as self.guts!");
                }
            }
            for ta in 0..self.combo_sets[set].trnsp_all.len() {
                assert!(self.combo_sets[set].trnsp_all[ta].i_deltas.len() == self.guts.len(), "self.combo_sets[{set}].trnsp_all[{ta}].i_deltas does not have the same length as self.guts!");
                assert!(self.combo_sets[set].trnsp_all[ta].x_deltas.len() == self.guts.len(), "self.combo_sets[{set}].trnsp_all[{ta}].x_deltas does not have the same length as self.guts!");
            }
        }
    }

    pub fn check_digitalref_invariants(&self) {
        for g in 0..self.guts.len() {
            for t in 0..self.guts[g].gut_triggers.togs.len() {
                assert!(self.guts[g].gut_triggers.togs[t] < self.key_codes.len(), "self.guts[{g}].gut_triggers.togs[{t}] is an index to an OOB Digital Input!");
            }
            for tg in 0..self.guts[g].trnsp_gut.len() {
                for t in 0..self.guts[g].trnsp_gut[tg].triggers.len() {
                    assert!(self.guts[g].trnsp_gut[tg].triggers[t] < self.key_codes.len(), "self.guts[{g}].trnsp_gut[{tg}].triggers[{t}] is an index to an OOB Digital Input!")
                }
            }
        }
        for sus in 0..self.gut_holds.sustain.togs.len() {
            assert!(self.gut_holds.sustain.togs[sus] < self.key_codes.len(), "self.gut_holds.sustain.togs[{sus}] is an index to an OOB Digital Input!")
        }
        for isus in 0..self.gut_holds.inv_sustain.togs.len() {
            assert!(self.gut_holds.inv_sustain.togs[isus] < self.key_codes.len(), "self.gut_holds.inv_sustain.togs[{isus}] is an index to an OOB Digital Input!")
        }
        for sos in 0..self.gut_holds.sostenuto.togs.len() {
            assert!(self.gut_holds.sostenuto.togs[sos] < self.key_codes.len(), "self.gut_holds.sostenuto.togs[{sos}] is an index to an OOB Digital Input!")
        }
        for isos in 0..self.gut_holds.inv_sostenuto.togs.len() {
            assert!(self.gut_holds.inv_sostenuto.togs[isos] < self.key_codes.len(), "self.gut_holds.inv_sostenuto.togs[{isos}] is an index to an OOB Digital Input!")
        }

        for set in 0..self.valve_sets.len() {
            for b in 0..self.valve_sets[set].buttons.len() {
                for t in 0..self.valve_sets[set].buttons[b].togs.len() {
                    assert!(self.valve_sets[set].buttons[b].togs[t] < self.key_codes.len(), "self.valve_sets[{set}].buttons[{b}].togs[{t}] is an index to an OOB Digital Input!")
                }
                for to in 0..self.valve_sets[set].buttons[b].trnsp_one.len() {
                    for t in 0..self.valve_sets[set].buttons[b].trnsp_one[to].triggers.len() {
                        assert!(self.valve_sets[set].buttons[b].trnsp_one[to].triggers[t] < self.key_codes.len(), "self.valve_sets[{set}].buttons[{b}].trnsp_one[{to}].triggers[{t}] is an index to an OOB Digital Input!")
                    }
                }
            }
            for ta in 0..self.valve_sets[set].trnsp_all.len() {
                for t in 0..self.valve_sets[set].trnsp_all[ta].triggers.len() {
                    assert!(self.valve_sets[set].trnsp_all[ta].triggers[t] < self.key_codes.len(), "self.valve_sets[{set}].trnsp_all[{ta}].triggers[{t}] is an index to an OOB Digital Input!")
                }
            }
            for sus in 0..self.valve_sets[set].holds.sustain.togs.len() {
                assert!(self.valve_sets[set].holds.sustain.togs[sus] < self.key_codes.len(), "self.valve_sets[{set}].holds.sustain.togs[{sus}] is an index to an OOB Digital Input!")
            }
            for isus in 0..self.valve_sets[set].holds.inv_sustain.togs.len() {
                assert!(self.valve_sets[set].holds.inv_sustain.togs[isus] < self.key_codes.len(), "self.valve_sets[{set}].holds.inv_sustain.togs[{isus}] is an index to an OOB Digital Input!")
            }
            for sos in 0..self.valve_sets[set].holds.sostenuto.togs.len() {
                assert!(self.valve_sets[set].holds.sostenuto.togs[sos] < self.key_codes.len(), "self.valve_sets[{set}].holds.sostenuto.togs[{sos}] is an index to an OOB Digital Input!")
            }
            for isos in 0..self.valve_sets[set].holds.inv_sostenuto.togs.len() {
                assert!(self.valve_sets[set].holds.inv_sostenuto.togs[isos] < self.key_codes.len(), "self.valve_sets[{set}].holds.inv_sostenuto.togs[{isos}] is an index to an OOB Digital Input!")
            }
        }
        for set in 0..self.fret_sets.len() {
            for b in 0..self.fret_sets[set].buttons.len() {
                for t in 0..self.fret_sets[set].buttons[b].togs.len() {
                    assert!(self.fret_sets[set].buttons[b].togs[t] < self.key_codes.len(), "self.fret_sets[{set}].buttons[{b}].togs[{t}] is an index to an OOB Digital Input!")
                }
                for to in 0..self.fret_sets[set].buttons[b].trnsp_one.len() {
                    for t in 0..self.fret_sets[set].buttons[b].trnsp_one[to].triggers.len() {
                        assert!(self.fret_sets[set].buttons[b].trnsp_one[to].triggers[t] < self.key_codes.len(), "self.fret_sets[{set}].buttons[{b}].trnsp_one[{to}].triggers[{t}] is an index to an OOB Digital Input!")
                    }
                }
            }
            for ta in 0..self.fret_sets[set].trnsp_all.len() {
                for t in 0..self.fret_sets[set].trnsp_all[ta].triggers.len() {
                    assert!(self.fret_sets[set].trnsp_all[ta].triggers[t] < self.key_codes.len(), "self.fret_sets[{set}].trnsp_all[{ta}].triggers[{t}] is an index to an OOB Digital Input!")
                }
            }
            for sus in 0..self.fret_sets[set].holds.sustain.togs.len() {
                assert!(self.fret_sets[set].holds.sustain.togs[sus] < self.key_codes.len(), "self.fret_sets[{set}].holds.sustain.togs[{sus}] is an index to an OOB Digital Input!")
            }
            for isus in 0..self.fret_sets[set].holds.inv_sustain.togs.len() {
                assert!(self.fret_sets[set].holds.inv_sustain.togs[isus] < self.key_codes.len(), "self.fret_sets[{set}].holds.inv_sustain.togs[{isus}] is an index to an OOB Digital Input!")
            }
            for sos in 0..self.fret_sets[set].holds.sostenuto.togs.len() {
                assert!(self.fret_sets[set].holds.sostenuto.togs[sos] < self.key_codes.len(), "self.fret_sets[{set}].holds.sostenuto.togs[{sos}] is an index to an OOB Digital Input!")
            }
            for isos in 0..self.fret_sets[set].holds.inv_sostenuto.togs.len() {
                assert!(self.fret_sets[set].holds.inv_sostenuto.togs[isos] < self.key_codes.len(), "self.fret_sets[{set}].holds.inv_sostenuto.togs[{isos}] is an index to an OOB Digital Input!")
            }
        }
        for set in 0..self.radio_sets.len() {
            for b in 0..self.radio_sets[set].buttons.len() {
                for t in 0..self.radio_sets[set].buttons[b].togs.len() {
                    assert!(self.radio_sets[set].buttons[b].togs[t] < self.key_codes.len(), "self.radio_sets[{set}].buttons[{b}].togs[{t}] is an index to an OOB Digital Input!")
                }
                for to in 0..self.radio_sets[set].buttons[b].trnsp_one.len() {
                    for t in 0..self.radio_sets[set].buttons[b].trnsp_one[to].triggers.len() {
                        assert!(self.radio_sets[set].buttons[b].trnsp_one[to].triggers[t] < self.key_codes.len(), "self.radio_sets[{set}].buttons[{b}].trnsp_one[{to}].triggers[{t}] is an index to an OOB Digital Input!")
                    }
                }
            }
            for ta in 0..self.radio_sets[set].trnsp_all.len() {
                for t in 0..self.radio_sets[set].trnsp_all[ta].triggers.len() {
                    assert!(self.radio_sets[set].trnsp_all[ta].triggers[t] < self.key_codes.len(), "self.radio_sets[{set}].trnsp_all[{ta}].triggers[{t}] is an index to an OOB Digital Input!")
                }
            }
            for sus in 0..self.radio_sets[set].holds.sustain.togs.len() {
                assert!(self.radio_sets[set].holds.sustain.togs[sus] < self.key_codes.len(), "self.radio_sets[{set}].holds.sustain.togs[{sus}] is an index to an OOB Digital Input!")
            }
            for isus in 0..self.radio_sets[set].holds.inv_sustain.togs.len() {
                assert!(self.radio_sets[set].holds.inv_sustain.togs[isus] < self.key_codes.len(), "self.radio_sets[{set}].holds.inv_sustain.togs[{isus}] is an index to an OOB Digital Input!")
            }
            for sos in 0..self.radio_sets[set].holds.sostenuto.togs.len() {
                assert!(self.radio_sets[set].holds.sostenuto.togs[sos] < self.key_codes.len(), "self.radio_sets[{set}].holds.sostenuto.togs[{sos}] is an index to an OOB Digital Input!")
            }
            for isos in 0..self.radio_sets[set].holds.inv_sostenuto.togs.len() {
                assert!(self.radio_sets[set].holds.inv_sostenuto.togs[isos] < self.key_codes.len(), "self.radio_sets[{set}].holds.inv_sostenuto.togs[{isos}] is an index to an OOB Digital Input!")
            }
        }
        for set in 0..self.combo_sets.len() {
            for b in 0..self.combo_sets[set].buttons.len() {
                for t in 0..self.combo_sets[set].buttons[b].togs.len() {
                    assert!(self.combo_sets[set].buttons[b].togs[t] < self.key_codes.len(), "self.combo_sets[{set}].buttons[{b}].togs[{t}] is an index to an OOB Digital Input!")
                }
            }
            for c in 0..self.combo_sets[set].combos.len() {
                for to in 0..self.combo_sets[set].combos[c].trnsp_one.len() {
                    for t in 0..self.combo_sets[set].combos[c].trnsp_one[to].triggers.len() {
                        assert!(self.combo_sets[set].combos[c].trnsp_one[to].triggers[t] < self.key_codes.len(), "self.combo_sets[{set}].combos[{c}].trnsp_one[{to}].triggers[{t}] is an index to an OOB Digital Input!")
                    }
                }
            }
            for ta in 0..self.combo_sets[set].trnsp_all.len() {
                for t in 0..self.combo_sets[set].trnsp_all[ta].triggers.len() {
                    assert!(self.combo_sets[set].trnsp_all[ta].triggers[t] < self.key_codes.len(), "self.combo_sets[{set}].trnsp_all[{ta}].triggers[{t}] is an index to an OOB Digital Input!")
                }
            }
            for sus in 0..self.combo_sets[set].holds.sustain.togs.len() {
                assert!(self.combo_sets[set].holds.sustain.togs[sus] < self.key_codes.len(), "self.combo_sets[{set}].holds.sustain.togs[{sus}] is an index to an OOB Digital Input!")
            }
            for isus in 0..self.combo_sets[set].holds.inv_sustain.togs.len() {
                assert!(self.combo_sets[set].holds.inv_sustain.togs[isus] < self.key_codes.len(), "self.combo_sets[{set}].holds.inv_sustain.togs[{isus}] is an index to an OOB Digital Input!")
            }
            for sos in 0..self.combo_sets[set].holds.sostenuto.togs.len() {
                assert!(self.combo_sets[set].holds.sostenuto.togs[sos] < self.key_codes.len(), "self.combo_sets[{set}].holds.sostenuto.togs[{sos}] is an index to an OOB Digital Input!")
            }
            for isos in 0..self.combo_sets[set].holds.inv_sostenuto.togs.len() {
                assert!(self.combo_sets[set].holds.inv_sostenuto.togs[isos] < self.key_codes.len(), "self.combo_sets[{set}].holds.inv_sostenuto.togs[{isos}] is an index to an OOB Digital Input!")
            }
        }
    }
}

pub enum SetType {
    ValveSet,
    FretSet,
    RadioSet,
    ComboSet,
}

pub enum HoldType {
    Sustain,
    InvSustain,
    Sostenuto,
    InvSostenuto,
}

mod qe_gut_methods;
mod qe_kcs_methods;
mod qe_misc_btns;
mod qe_set_methods;
mod qe_trnsp_methods;
