use crate::qopedit::{QopEdit, Pluck};

impl QopEdit {
    pub fn plk_insert_p(&mut self, p_idx: usize) {
    if p_idx <= self.plucks.len() {
        self.plucks.insert(p_idx, Pluck::default());
        for set in 0..self.valve_sets.len() {
            self.valve_sets[set].insert_pluck(p_idx);
        }
        for set in 0..self.fret_sets.len() {
            self.fret_sets[set].insert_pluck(p_idx);
        }
        for set in 0..self.radio_sets.len() {
            self.radio_sets[set].insert_pluck(p_idx);
        }
        for set in 0..self.aero_sets.len() {
            self.aero_sets[set].insert_pluck(p_idx);
        }
    }
}
pub fn plk_remove_p(&mut self, p_idx: usize) {
    if self.plucks.len() > 1 && p_idx <= self.plucks.len() {
        self.plucks.remove(p_idx);
        for set in 0..self.valve_sets.len() {
            self.valve_sets[set].remove_pluck(p_idx);
        }
        for set in 0..self.fret_sets.len() {
            self.fret_sets[set].remove_pluck(p_idx);
        }
        for set in 0..self.radio_sets.len() {
            self.radio_sets[set].remove_pluck(p_idx);
        }
        for set in 0..self.aero_sets.len() {
            self.aero_sets[set].remove_pluck(p_idx);
        }
    }
}
pub fn plk_insert_key(&mut self, p_idx: usize, key_idx_val: usize) {
    if p_idx < self.plucks.len() && key_idx_val < self.key_codes.len() {
        if !self.plucks[p_idx].pluck.togs.contains(&key_idx_val) {
            self.plucks[p_idx].pluck.togs.push(key_idx_val)
        };
    }
}
pub fn plk_remove_key(&mut self, p_idx: usize, key_idx_val: usize) {
    if p_idx < self.plucks.len() && key_idx_val < self.key_codes.len() {
        self.plucks[p_idx].pluck.togs.retain_mut(|idx: &mut usize| {
            if *idx < key_idx_val {
                return true;
            } else if *idx == key_idx_val {
                return false;
            } else {
                *idx -= 1;
                return true;
            }
        });
    }
}
pub fn plk_insert_hold_btn(&mut self, h_kind: u8, key_idx_val: usize) {
    if key_idx_val <= self.key_codes.len() {
        match h_kind {
            0 => {
                if !self.plk_holds.sustain.togs.contains(&key_idx_val) {
                    self.plk_holds.sustain.togs.push(key_idx_val)
                }
            }
            1 => {
                if !self.plk_holds.inv_sustain.togs.contains(&key_idx_val) {
                    self.plk_holds.inv_sustain.togs.push(key_idx_val)
                }
            }
            2 => {
                if !self.plk_holds.sostenuto.togs.contains(&key_idx_val) {
                    self.plk_holds.sostenuto.togs.push(key_idx_val)
                }
            }
            3 => {
                if !self.plk_holds.inv_sostenuto.togs.contains(&key_idx_val) {
                    self.plk_holds.inv_sostenuto.togs.push(key_idx_val)
                }
            }
            _ => {}
        }
    }
}
pub fn plk_remove_hold_btn(&mut self, h_kind: u8, key_idx_val: usize) {
    match h_kind {
        0 => self
            .plk_holds
            .sustain
            .togs
            .retain(|&idx| idx != key_idx_val),
        1 => self
            .plk_holds
            .inv_sustain
            .togs
            .retain(|&idx| idx != key_idx_val),
        2 => self
            .plk_holds
            .sostenuto
            .togs
            .retain(|&idx| idx != key_idx_val),
        3 => self
            .plk_holds
            .inv_sostenuto
            .togs
            .retain(|&idx| idx != key_idx_val),
        _ => {}
    }
}
pub fn plk_change_idx_out(&mut self, p_idx: usize, i_del_val: usize) {
    if p_idx < self.plucks.len() {
        self.plucks[p_idx].idx_out = i_del_val;
    }
}
pub fn plk_change_xtra_out(&mut self, p_idx: usize, x_del_val: f64) {
    if p_idx < self.plucks.len() {
        self.plucks[p_idx].xtra_out = x_del_val;
    }
}
}