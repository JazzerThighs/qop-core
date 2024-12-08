use crate::{ComboSet, Edit, HoldType, IndvSet, Qop, SetType};

impl Qop<Edit> {
    pub fn set_insert_set(&mut self, set_kind: SetType, set_idx: usize) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets
                        .insert(set_idx, IndvSet::new(self.guts.len()));
                }
            }
            SetType::FretSet => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets
                        .insert(set_idx, IndvSet::new(self.guts.len()));
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets
                        .insert(set_idx, IndvSet::new(self.guts.len()));
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.combo_sets.len() {
                    self.combo_sets
                        .insert(set_idx, ComboSet::new(self.guts.len()));
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_remove_set(&mut self, set_kind: SetType, set_idx: usize) {
        match set_kind {
            SetType::ValveSet => {
                if self.valve_sets.len() > 0 && self.valve_sets.len() > set_idx {
                    self.valve_sets.remove(set_idx);
                }
            }
            SetType::FretSet => {
                if self.fret_sets.len() > 0 && self.fret_sets.len() > set_idx {
                    self.fret_sets.remove(set_idx);
                }
            }
            SetType::RadioSet => {
                if self.radio_sets.len() > 0 && self.radio_sets.len() > set_idx {
                    self.radio_sets.remove(set_idx);
                }
            }
            SetType::ComboSet => {
                if self.combo_sets.len() > 0 && self.combo_sets.len() > set_idx {
                    self.combo_sets.remove(set_idx);
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_insert_btn(&mut self, set_kind: SetType, set_idx: usize, btn_idx: usize) {
        match set_kind {
            SetType::ValveSet => self.valve_sets[set_idx].insert_btn(btn_idx, self.guts.len()),
            SetType::FretSet => self.fret_sets[set_idx].insert_btn(btn_idx, self.guts.len()),
            SetType::RadioSet => self.radio_sets[set_idx].insert_btn(btn_idx, self.guts.len()),
            SetType::ComboSet => self.combo_sets[set_idx].insert_btn(btn_idx),
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_remove_btn(&mut self, set_kind: SetType, set_idx: usize, btn_idx: usize) {
        match set_kind {
            SetType::ValveSet => self.valve_sets[set_idx].remove_btn(btn_idx),
            SetType::FretSet => self.fret_sets[set_idx].remove_btn(btn_idx),
            SetType::RadioSet => self.radio_sets[set_idx].remove_btn(btn_idx),
            SetType::ComboSet => self.combo_sets[set_idx].remove_btn(btn_idx),
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.combo_sets.len() {
            self.combo_sets[set_idx].insert_combo(c_idx, self.guts.len());
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.combo_sets.len() {
            self.combo_sets[set_idx].remove_combo(c_idx);
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_insert_btn_key(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                SetType::ValveSet => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                SetType::FretSet => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                SetType::RadioSet => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                SetType::ComboSet => {
                    if set_idx < self.combo_sets.len() {
                        self.combo_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_remove_btn_key(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                SetType::ValveSet => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                SetType::FretSet => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                SetType::RadioSet => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                SetType::ComboSet => {
                    if set_idx < self.combo_sets.len() {
                        self.combo_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_change_i_deltas(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
    ) {
        if del_idx < self.guts.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    match set_kind {
                        SetType::ValveSet => {
                            if set_idx < self.valve_sets.len() {
                                self.valve_sets[set_idx].buttons[del_idx].i_deltas[i] = delta
                            }
                        }
                        SetType::FretSet => {
                            if set_idx < self.fret_sets.len() {
                                self.fret_sets[set_idx].buttons[del_idx].i_deltas[i] = delta
                            }
                        }
                        SetType::RadioSet => {
                            if set_idx < self.radio_sets.len() {
                                self.radio_sets[set_idx].buttons[del_idx].i_deltas[i] = delta
                            }
                        }
                        SetType::ComboSet => {
                            if set_idx < self.combo_sets.len() {
                                self.combo_sets[set_idx].combos[del_idx].i_deltas[i] = delta
                            }
                        }
                    }
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_change_x_deltas(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        del_idx: usize,
        x_del_vec: Vec<Option<f64>>,
    ) {
        for (x, &x_del) in x_del_vec.iter().enumerate() {
            if let Some(delta) = x_del {
                match set_kind {
                    SetType::ValveSet => {
                        if set_idx < self.valve_sets.len() {
                            self.valve_sets[set_idx].buttons[del_idx].x_deltas[x] = delta
                        }
                    }
                    SetType::FretSet => {
                        if set_idx < self.fret_sets.len() {
                            self.fret_sets[set_idx].buttons[del_idx].x_deltas[x] = delta
                        }
                    }
                    SetType::RadioSet => {
                        if set_idx < self.radio_sets.len() {
                            self.radio_sets[set_idx].buttons[del_idx].x_deltas[x] = delta
                        }
                    }
                    SetType::ComboSet => {
                        if set_idx < self.combo_sets.len() {
                            self.combo_sets[set_idx].combos[del_idx].x_deltas[x] = delta
                        }
                    }
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_insert_hold_btn_key(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        h_kind: HoldType,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                SetType::ValveSet => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                SetType::FretSet => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                SetType::RadioSet => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                SetType::ComboSet => {
                    if set_idx < self.combo_sets.len() {
                        self.combo_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
    pub fn set_remove_hold_btn_key(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        h_kind: HoldType,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                SetType::ValveSet => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                SetType::FretSet => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                SetType::RadioSet => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                SetType::ComboSet => {
                    if set_idx < self.combo_sets.len() {
                        self.combo_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
            }
        }
        self.check_gutdelta_lengths();
        self.check_digitalref_invariants();
    }
}
