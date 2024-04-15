#![allow(dead_code)]
mod btns;
use btns::{AeroSet, FretSet, Pluck, RadioSet, ValveSet};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qop {
    plucks: Vec<Pluck>,
    valve_sets: Vec<ValveSet>,
    fret_sets: Vec<FretSet>,
    radio_sets: Vec<RadioSet>,
    aero_sets: Vec<AeroSet>,
    key_idxs: Vec<usize>,
}

impl Default for Qop {
    fn default() -> Self {
        return Qop {
            plucks: vec![Pluck::default()],
            valve_sets: vec![],
            fret_sets: vec![],
            radio_sets: vec![],
            aero_sets: vec![],
            key_idxs: vec![0usize],
        };
    }
}

impl Qop {
    pub fn insert_pluck(&mut self, idx: usize) {
        self.plucks.insert(idx, Pluck::default());
        for set in 0..self.valve_sets.len() {
            self.valve_sets[set].insert_pluck(idx);
        }
        for set in 0..self.fret_sets.len() {
            self.fret_sets[set].insert_pluck(idx);
        }
        for set in 0..self.radio_sets.len() {
            self.radio_sets[set].insert_pluck(idx);
        }
        for set in 0..self.aero_sets.len() {
            self.aero_sets[set].insert_pluck(idx);
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
    pub fn change_pluck_keys(&mut self, p_idx: usize, k_ids: &[usize]) {todo!()}
    
    pub fn insert_valve_set(&mut self, idx: usize) {
        self.valve_sets.insert(idx, ValveSet::default());
        self.valve_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_valve_set(&mut self, idx: usize) {
        if self.valve_sets.len() > 0 && self.valve_sets.len() > idx {
            self.valve_sets.remove(idx);
        }
    }
    pub fn insert_valve(&mut self, set_idx: usize, v_idx: usize) {todo!()}
    pub fn remove_valve(&mut self, set_idx: usize, v_idx: usize) {todo!()}

    pub fn insert_fret_set(&mut self, idx: usize) {
        self.fret_sets.insert(idx, FretSet::default());
        self.fret_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_fret_set(&mut self, idx: usize) {
        if self.fret_sets.len() > 0 && self.fret_sets.len() > idx {
            self.fret_sets.remove(idx);
        }
    }
    pub fn insert_fret(&mut self, set_idx: usize, f_idx: usize) {todo!()}
    pub fn remove_fret(&mut self, set_idx: usize, f_idx: usize) {todo!()}

    pub fn insert_radio_set(&mut self, idx: usize) {
        self.radio_sets.insert(idx, RadioSet::default());
        self.radio_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_radio_set(&mut self, idx: usize) {
        if self.radio_sets.len() > 0 && self.radio_sets.len() > idx {
            self.radio_sets.remove(idx);
        }
    }
    pub fn insert_radio(&mut self, set_idx: usize, r_idx: usize) {todo!()}
    pub fn remove_radio(&mut self, set_idx: usize, r_idx: usize) {todo!()}

    pub fn insert_aero_set(&mut self, idx: usize) {
        self.aero_sets.insert(idx, AeroSet::default());
        self.aero_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_aero_set(&mut self, idx: usize) {
        if self.aero_sets.len() > 0 && self.aero_sets.len() > idx {
            self.aero_sets.remove(idx);
        }
    }
    pub fn insert_aero(&mut self, set_idx: usize, a_idx: usize) {todo!()}
    pub fn remove_aero(&mut self, set_idx: usize, a_idx: usize) {todo!()}
    pub fn insert_combo(&mut self, set_idx: usize, c_idx: usize) {todo!()}
    pub fn remove_combo(&mut self, set_idx: usize, c_idx: usize) {todo!()}

    pub fn change_set_btn_keys(&mut self, set_kind: u8, set_idx: usize, btn_idx: usize, k_ids: &[usize]) {todo!()}
    pub fn change_hold_btn(&mut self, set_idx: usize, k_ids: &[usize], h_kind: u8) {todo!()}
    pub fn change_transpose_all(&mut self, set_idx: usize, k_ids: &[usize], i_delta: i64, x_delta: f64) {todo!()}
    pub fn change_transpose_one(&mut self, set_idx: usize, btn_idx: usize, k_ids: &[usize], i_delta: i64, x_delta: f64) {todo!()}
}
