#![allow(dead_code)]
mod btns;
use btns::{AeroSet, FretSet, Pluck, RadioSet, ValveSet};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qop {
    key_idxs: Vec<usize>,
    plucks: Vec<Pluck>,
    valve_sets: Vec<ValveSet>,
    fret_sets: Vec<FretSet>,
    radio_sets: Vec<RadioSet>,
    aero_sets: Vec<AeroSet>,
}

impl Default for Qop {
    fn default() -> Self {
        return Qop {
            key_idxs: vec![0usize],
            plucks: vec![Pluck::default()],
            valve_sets: vec![],
            fret_sets: vec![],
            radio_sets: vec![],
            aero_sets: vec![],
        };
    }
}

impl Qop {
    pub fn push_key_idx(key_id_val: usize) {
        todo!()
    }
    pub fn remove_key_idx(key_id_val: usize) {
        todo!()
    }
    /* ********************************************************************* */
    pub fn insert_pluck(&mut self, p_idx: usize) {
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
    pub fn remove_pluck(&mut self, idx: usize) {
        if self.plucks.len() > 1 {
            self.plucks.remove(idx);
            for set in 0..self.valve_sets.len() {
                self.valve_sets[set].remove_pluck(idx);
            }
            for set in 0..self.fret_sets.len() {
                self.fret_sets[set].remove_pluck(idx);
            }
            for set in 0..self.radio_sets.len() {
                self.radio_sets[set].remove_pluck(idx);
            }
            for set in 0..self.aero_sets.len() {
                self.aero_sets[set].remove_pluck(idx);
            }
        }
    }
    pub fn push_pluck_key(&mut self, p_idx: usize, k_id_val: usize) {
        todo!()
    }
    pub fn remove_pluck_key(&mut self, p_idx: usize, k_id_val: usize) {
        todo!()
    }
    pub fn change_pluck_idx_out(&mut self, p_idx: usize, i_del_val: i64) {
        todo!()
    }
    pub fn change_pluck_xtra_out(&mut self, p_idx: usize, i_del_val: i64) {
        todo!()
    }
    /* ********************************************************************* */
    pub fn insert_set(&mut self, set_kind: u8, set_idx: usize) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets.insert(set_idx, ValveSet::default());
                    self.valve_sets[set_idx].init_set(self.plucks.len());
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets.insert(set_idx, FretSet::default());
                    self.fret_sets[set_idx].init_set(self.plucks.len());
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets.insert(set_idx, RadioSet::default());
                    self.radio_sets[set_idx].init_set(self.plucks.len());
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets.insert(set_idx, AeroSet::default());
                    self.aero_sets[set_idx].init_set(self.plucks.len());
                }
            }
            _ => return,
        }
    }
    pub fn remove_set(&mut self, set_kind: u8, set_idx: usize) {
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
    pub fn insert_btn(&mut self, set_kind: u8, set_idx: usize, btn_idx: usize) {
        match set_kind {
            0 => self.valve_sets[set_idx].insert_btn(btn_idx, self.plucks.len()),
            1 => self.fret_sets[set_idx].insert_btn(btn_idx, self.plucks.len()),
            2 => self.radio_sets[set_idx].insert_btn(btn_idx, self.plucks.len()),
            3 => self.aero_sets[set_idx].insert_btn(btn_idx),
            _ => return,
        }
    }
    pub fn remove_btn(&mut self, set_kind: u8, set_idx: usize, btn_idx: usize) {
        match set_kind {
            0 => self.valve_sets[set_idx].remove_btn(btn_idx),
            1 => self.fret_sets[set_idx].remove_btn(btn_idx),
            2 => self.radio_sets[set_idx].remove_btn(btn_idx),
            3 => self.aero_sets[set_idx].remove_btn(btn_idx),
            _ => return,
        }
    }
    pub fn insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        self.aero_sets[set_idx].insert_combo(c_idx, self.plucks.len());
    }
    pub fn remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        self.aero_sets[set_idx].remove_combo(c_idx);
    }
    /* ********************************************************************* */
    pub fn change_set_idx_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        i_del_idx: usize,
        i_del_val: i64,
    ) {
        todo!()
    }
    pub fn change_set_xrta_delta(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        del_idx: usize,
        x_del_idx: usize,
        x_del_val: i64,
    ) {
        todo!()
    }
    pub fn push_set_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        k_id_val: usize,
    ) {
        todo!()
    }
    pub fn remove_set_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        k_id_val: usize,
    ) {
        todo!()
    }
    pub fn push_set_hold_btn_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        h_kind: u8,
        k_id_val: usize,
    ) {
        todo!()
    }
    pub fn push_transpose_all_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        k_id_val: usize,
        i_delta: i64,
        x_delta: f64,
    ) {
        todo!()
    }
    pub fn remove_transpose_all_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        k_id_val: usize,
        i_delta: i64,
        x_delta: f64,
    ) {
        todo!()
    }
    pub fn push_transpose_one_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        d_idx: usize,
        k_id_val: usize,
        i_delta: i64,
        x_delta: f64,
    ) {
        todo!()
    }
    pub fn remove_transpose_one_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        d_idx: usize,
        k_id_val: usize,
        i_delta: i64,
        x_delta: f64,
    ) {
        todo!()
    }
}
