#![allow(dead_code)]
mod btns;
use btns::{ComboSet, HoldBtns, IndvSet, Pluck};
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QopEdit {
    key_codes: Vec<KeyCode>,
    plucks: Vec<Pluck>,
    plk_holds: HoldBtns,
    valve_sets: Vec<IndvSet>,
    fret_sets: Vec<IndvSet>,
    radio_sets: Vec<IndvSet>,
    aero_sets: Vec<ComboSet>,
}

impl Default for QopEdit {
    fn default() -> Self {
        return QopEdit {
            key_codes: vec![],
            plucks: vec![Pluck::default()],
            plk_holds: HoldBtns::default(),
            valve_sets: vec![],
            fret_sets: vec![],
            radio_sets: vec![],
            aero_sets: vec![],
        };
    }
}

impl QopEdit {
    pub fn kcs_insert_k(&mut self, key_code: KeyCode) {
        if !self.key_codes.contains(&key_code) {
            self.key_codes.push(key_code)
        };
    }
    pub fn kcs_remove_k(&mut self, key_code: KeyCode) {
        for i in 0..self.key_codes.len() {
            if self.key_codes[i] == key_code {
                for p in 0..self.plucks.len() {
                    self.plucks[p].pluck.togs.retain(|&k| k != i);
                    for t in 0..self.plucks[p].trnsp_pluck.len() {
                        self.plucks[p].trnsp_pluck[t].triggers.retain(|&k| k != i);
                    }
                }
                self.plk_holds.sustain.togs.retain(|&k| k != i);
                self.plk_holds.inv_sustain.togs.retain(|&k| k != i);
                self.plk_holds.sostenuto.togs.retain(|&k| k != i);
                self.plk_holds.inv_sostenuto.togs.retain(|&k| k != i);
                for set in 0..self.valve_sets.len() {
                    self.valve_sets[set].remove_key_idx(i);
                }
                for set in 0..self.fret_sets.len() {
                    self.fret_sets[set].remove_key_idx(i);
                }
                for set in 0..self.radio_sets.len() {
                    self.radio_sets[set].remove_key_idx(i);
                }
                for set in 0..self.aero_sets.len() {
                    self.aero_sets[set].remove_key_idx(i);
                }
            }
        }
    }
    pub fn kcs_swap_labels(&mut self, kc1: KeyCode, kc2: KeyCode) -> Option<(usize, usize)> {
        // swaps only the labels for 2 existing keys, which leaves alone all of the nodes' usize values in the rest of the QopEdit.
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
            return Some((i1.unwrap(), i2.unwrap()));
        }
        return None;
    }
    pub fn kcs_swap_idxs(&mut self, kc1: KeyCode, kc2: KeyCode) {
        // swaps the position of 2 existing keys, and in turn, would then need to swap all of the nodes' usize values in all of the plucks' and sets' various fields.
        let res: Option<(usize, usize)> = self.kcs_swap_labels(kc1, kc2);
        if res.is_some() {
            let (i1, i2) = (res.unwrap().0, res.unwrap().1);
            todo!()
        }
    }
    

    /* ********************************************************************* */

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
        if self.plucks.len() > 1 && 0 <= p_idx && p_idx <= self.plucks.len() {
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
        if p_idx < self.plucks.len() {
            if !self.plucks[p_idx].pluck.togs.contains(&key_idx_val) {
                self.plucks[p_idx].pluck.togs.push(key_idx_val)
            };
        }
    }
    pub fn plk_remove_key(&mut self, p_idx: usize, key_idx_val: usize) {
        if p_idx < self.plucks.len() {
            self.plucks[p_idx]
                .pluck
                .togs
                .retain(|&idx| idx != key_idx_val);
        }
    }
    pub fn plk_insert_hold_btn(&mut self, h_kind: u8, key_idx_val: usize) {
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

    /* ********************************************************************* */

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
        todo!()
    }
    pub fn set_remove_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        todo!()
    }
    pub fn set_change_idx_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
    ) {
        for (i, &i_del) in i_del_vec.iter().enumerate() {
            if let Some(delta) = i_del {
                match set_kind {
                    0 => self.valve_sets[set_idx].buttons[del_idx].idx_deltas[i] = delta,
                    1 => self.fret_sets[set_idx].buttons[del_idx].idx_deltas[i] = delta,
                    2 => self.radio_sets[set_idx].buttons[del_idx].idx_deltas[i] = delta,
                    3 => self.aero_sets[set_idx].combos[del_idx].idx_deltas[i] = delta,
                    _ => return,
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
                    0 => self.valve_sets[set_idx].buttons[del_idx].xtra_deltas[x] = delta,
                    1 => self.fret_sets[set_idx].buttons[del_idx].xtra_deltas[x] = delta,
                    2 => self.radio_sets[set_idx].buttons[del_idx].xtra_deltas[x] = delta,
                    3 => self.aero_sets[set_idx].combos[del_idx].xtra_deltas[x] = delta,
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
        todo!()
    }
    pub fn set_remove_hold_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        h_kind: u8,
        key_idx_val: usize,
    ) {
        todo!()
    }
    pub fn set_insert_trnsp_all_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        key_idx_val: Option<Vec<usize>>,
    ) {
        todo!()
    }
    pub fn set_remove_trnsp_all_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        todo!()
    }
    pub fn set_trnsp_all_idx_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        key_idx_val: usize,
        i_del_vec: Vec<Option<i64>>,
    ) {
        todo!()
    }
    pub fn set_trnsp_all_xrta_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        key_idx_val: usize,
        x_del_vec: Vec<Option<f64>>,
    ) {
        todo!()
    }
    pub fn set_insert_trnsp_one_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        d_idx: usize,
        key_idx_vals: Option<Vec<usize>>,
    ) {
        todo!()
    }
    pub fn set_remove_trnsp_one_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        d_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        todo!()
    }
    pub fn set_trnsp_one_idx_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
        i_del_vec: Vec<Option<i64>>,
    ) {
        todo!()
    }
    pub fn set_trnsp_one_xrta_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        key_idx_val: usize,
        x_del_vec: Vec<Option<f64>>,
    ) {
        todo!()
    }
}
