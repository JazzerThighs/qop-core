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

    pub fn insert_valve_set(&mut self, idx: usize) {
        self.valve_sets.insert(idx, ValveSet::default());
        self.valve_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_valve_set(&mut self, idx: usize) {
        if self.valve_sets.len() > 0 && self.valve_sets.len() > idx {
            self.valve_sets.remove(idx);
        }
    }

    pub fn insert_fret_set(&mut self, idx: usize) {
        self.fret_sets.insert(idx, FretSet::default());
        self.fret_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_fret_set(&mut self, idx: usize) {
        if self.fret_sets.len() > 0 && self.fret_sets.len() > idx {
            self.fret_sets.remove(idx);
        }
    }

    pub fn insert_radio_set(&mut self, idx: usize) {
        self.radio_sets.insert(idx, RadioSet::default());
        self.radio_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_radio_set(&mut self, idx: usize) {
        if self.radio_sets.len() > 0 && self.radio_sets.len() > idx {
            self.radio_sets.remove(idx);
        }
    }

    pub fn insert_aero_set(&mut self, idx: usize) {
        self.aero_sets.insert(idx, AeroSet::default());
        self.aero_sets[idx].insert_set(self.plucks.len());
    }
    pub fn remove_aero_set(&mut self, idx: usize) {
        if self.aero_sets.len() > 0 && self.aero_sets.len() > idx {
            self.aero_sets.remove(idx);
        }
    }
}
