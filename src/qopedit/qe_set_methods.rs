use super::{QopEdit, btns::{ComboSet, IndvSet}};

impl QopEdit {
    pub fn set_insert_set(&mut self, set_kind: u8, set_idx: usize) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets
                        .insert(set_idx, IndvSet::new(self.plucks.len()));
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets
                        .insert(set_idx, IndvSet::new(self.plucks.len()));
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets
                        .insert(set_idx, IndvSet::new(self.plucks.len()));
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets
                        .insert(set_idx, ComboSet::new(self.plucks.len()));
                }
            }
            _ => return,
        }
    }
    pub fn set_remove_set(&mut self, set_kind: u8, set_idx: usize) {
        match set_kind {
            0 => {
                if self.valve_sets.len() > 0 && self.valve_sets.len() > set_idx {
                    self.valve_sets.remove(set_idx);
                }
            }
            1 => {
                if self.fret_sets.len() > 0 && self.fret_sets.len() > set_idx {
                    self.fret_sets.remove(set_idx);
                }
            }
            2 => {
                if self.radio_sets.len() > 0 && self.radio_sets.len() > set_idx {
                    self.radio_sets.remove(set_idx);
                }
            }
            3 => {
                if self.aero_sets.len() > 0 && self.aero_sets.len() > set_idx {
                    self.aero_sets.remove(set_idx);
                }
            }
            _ => return,
        }
    }
    pub fn set_insert_btn(&mut self, set_kind: u8, set_idx: usize, btn_idx: usize) {
        match set_kind {
            0 => self.valve_sets[set_idx].insert_btn(btn_idx, self.plucks.len()),
            1 => self.fret_sets[set_idx].insert_btn(btn_idx, self.plucks.len()),
            2 => self.radio_sets[set_idx].insert_btn(btn_idx, self.plucks.len()),
            3 => self.aero_sets[set_idx].insert_btn(btn_idx),
            _ => return,
        }
    }
    pub fn set_remove_btn(&mut self, set_kind: u8, set_idx: usize, btn_idx: usize) {
        match set_kind {
            0 => self.valve_sets[set_idx].remove_btn(btn_idx),
            1 => self.fret_sets[set_idx].remove_btn(btn_idx),
            2 => self.radio_sets[set_idx].remove_btn(btn_idx),
            3 => self.aero_sets[set_idx].remove_btn(btn_idx),
            _ => return,
        }
    }
    pub fn set_insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.aero_sets.len() {
            self.aero_sets[set_idx].insert_combo(c_idx, self.plucks.len());
        }
    }
    pub fn set_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.aero_sets.len() {
            self.aero_sets[set_idx].remove_combo(c_idx);
        }
    }
    pub fn set_insert_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                0 => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                1 => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                2 => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                3 => {
                    if set_idx < self.aero_sets.len() {
                        self.aero_sets[set_idx].btn_insert_key(btn_idx, key_idx_val)
                    }
                }
                _ => return,
            }
        }
    }
    pub fn set_remove_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                0 => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                1 => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                2 => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                3 => {
                    if set_idx < self.aero_sets.len() {
                        self.aero_sets[set_idx].btn_remove_key(btn_idx, key_idx_val)
                    }
                }
                _ => return,
            }
        }
    }
    pub fn set_change_idx_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
    ) {
        if del_idx < self.plucks.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    match set_kind {
                        0 => {
                            if set_idx < self.valve_sets.len() {
                                self.valve_sets[set_idx].buttons[del_idx].idx_deltas[i] = delta
                            }
                        }
                        1 => {
                            if set_idx < self.fret_sets.len() {
                                self.fret_sets[set_idx].buttons[del_idx].idx_deltas[i] = delta
                            }
                        }
                        2 => {
                            if set_idx < self.radio_sets.len() {
                                self.radio_sets[set_idx].buttons[del_idx].idx_deltas[i] = delta
                            }
                        }
                        3 => {
                            if set_idx < self.aero_sets.len() {
                                self.aero_sets[set_idx].combos[del_idx].idx_deltas[i] = delta
                            }
                        }
                        _ => return,
                    }
                }
            }
        }
    }
    pub fn set_change_xrta_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        x_del_vec: Vec<Option<f64>>,
    ) {
        for (x, &x_del) in x_del_vec.iter().enumerate() {
            if let Some(delta) = x_del {
                match set_kind {
                    0 => {
                        if set_idx < self.valve_sets.len() {
                            self.valve_sets[set_idx].buttons[del_idx].xtra_deltas[x] = delta
                        }
                    }
                    1 => {
                        if set_idx < self.fret_sets.len() {
                            self.fret_sets[set_idx].buttons[del_idx].xtra_deltas[x] = delta
                        }
                    }
                    2 => {
                        if set_idx < self.radio_sets.len() {
                            self.radio_sets[set_idx].buttons[del_idx].xtra_deltas[x] = delta
                        }
                    }
                    3 => {
                        if set_idx < self.aero_sets.len() {
                            self.aero_sets[set_idx].combos[del_idx].xtra_deltas[x] = delta
                        }
                    }
                    _ => return,
                }
            }
        }
    }
    pub fn set_insert_hold_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        h_kind: u8,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                0 => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                1 => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                2 => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                3 => {
                    if set_idx < self.aero_sets.len() {
                        self.aero_sets[set_idx].hold_insert_key(h_kind, key_idx_val)
                    }
                }
                _ => return,
            }
        }
    }
    pub fn set_remove_hold_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        h_kind: u8,
        key_idx_val: usize,
    ) {
        if key_idx_val <= self.key_codes.len() {
            match set_kind {
                0 => {
                    if set_idx < self.valve_sets.len() {
                        self.valve_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                1 => {
                    if set_idx < self.fret_sets.len() {
                        self.fret_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                2 => {
                    if set_idx < self.radio_sets.len() {
                        self.radio_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                3 => {
                    if set_idx < self.aero_sets.len() {
                        self.aero_sets[set_idx].hold_remove_key(h_kind, key_idx_val)
                    }
                }
                _ => return,
            }
        }
    }
}