use crate::qopedit::{QopEdit, Gut};

impl QopEdit {
    pub fn gut_insert_g(&mut self, g_idx: usize) {
    if g_idx <= self.guts.len() {
        self.guts.insert(g_idx, Gut::default());
        for set in 0..self.valve_sets.len() {
            self.valve_sets[set].insert_gut(g_idx);
        }
        for set in 0..self.fret_sets.len() {
            self.fret_sets[set].insert_gut(g_idx);
        }
        for set in 0..self.radio_sets.len() {
            self.radio_sets[set].insert_gut(g_idx);
        }
        for set in 0..self.aero_sets.len() {
            self.aero_sets[set].insert_gut(g_idx);
        }
    }
}
pub fn gut_remove_g(&mut self, g_idx: usize) {
    if self.guts.len() > 1 && g_idx <= self.guts.len() {
        self.guts.remove(g_idx);
        for set in 0..self.valve_sets.len() {
            self.valve_sets[set].remove_gut(g_idx);
        }
        for set in 0..self.fret_sets.len() {
            self.fret_sets[set].remove_gut(g_idx);
        }
        for set in 0..self.radio_sets.len() {
            self.radio_sets[set].remove_gut(g_idx);
        }
        for set in 0..self.aero_sets.len() {
            self.aero_sets[set].remove_gut(g_idx);
        }
    }
}
pub fn gut_insert_key(&mut self, g_idx: usize, key_idx_val: usize) {
    if g_idx < self.guts.len() && key_idx_val < self.key_codes.len() {
        if !self.guts[g_idx].gut_triggers.togs.contains(&key_idx_val) {
            self.guts[g_idx].gut_triggers.togs.push(key_idx_val)
        };
    }
}
pub fn gut_remove_key(&mut self, g_idx: usize, key_idx_val: usize) {
    if g_idx < self.guts.len() && key_idx_val < self.key_codes.len() {
        self.guts[g_idx].gut_triggers.togs.retain_mut(|idx: &mut usize| {
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
pub fn gut_insert_hold_btn(&mut self, h_kind: u8, key_idx_val: usize) {
    if key_idx_val <= self.key_codes.len() {
        match h_kind {
            0 => {
                if !self.gut_holds.sustain.togs.contains(&key_idx_val) {
                    self.gut_holds.sustain.togs.push(key_idx_val)
                }
            }
            1 => {
                if !self.gut_holds.inv_sustain.togs.contains(&key_idx_val) {
                    self.gut_holds.inv_sustain.togs.push(key_idx_val)
                }
            }
            2 => {
                if !self.gut_holds.sostenuto.togs.contains(&key_idx_val) {
                    self.gut_holds.sostenuto.togs.push(key_idx_val)
                }
            }
            3 => {
                if !self.gut_holds.inv_sostenuto.togs.contains(&key_idx_val) {
                    self.gut_holds.inv_sostenuto.togs.push(key_idx_val)
                }
            }
            _ => {}
        }
    }
}
pub fn gut_remove_hold_btn(&mut self, h_kind: u8, key_idx_val: usize) {
    match h_kind {
        0 => self
            .gut_holds
            .sustain
            .togs
            .retain(|&idx| idx != key_idx_val),
        1 => self
            .gut_holds
            .inv_sustain
            .togs
            .retain(|&idx| idx != key_idx_val),
        2 => self
            .gut_holds
            .sostenuto
            .togs
            .retain(|&idx| idx != key_idx_val),
        3 => self
            .gut_holds
            .inv_sostenuto
            .togs
            .retain(|&idx| idx != key_idx_val),
        _ => {}
    }
}
pub fn gut_change_idx_out(&mut self, g_idx: usize, i_del_val: usize) {
    if g_idx < self.guts.len() {
        self.guts[g_idx].idx_out = i_del_val;
    }
}
pub fn gut_change_xtra_out(&mut self, g_idx: usize, x_del_val: f64) {
    if g_idx < self.guts.len() {
        self.guts[g_idx].xtra_out = x_del_val;
    }
}
}