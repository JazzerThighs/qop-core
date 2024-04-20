use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BtnToggle {
    toggles: Vec<usize>,
    pressed: bool,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeltaToggle {
    toggles: Vec<usize>,
    pressed: bool,
    idx_deltas: Vec<i64>,
    xtra_deltas: Vec<f64>,
    transpose_one: Vec<TransposeTrigger>,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HoldBtns {
    sustain: BtnToggle,
    inv_sustain: BtnToggle,
    sostenuto: BtnToggle,
    inv_sostenuto: BtnToggle,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransposeTrigger {
    triggers: Vec<usize>,
    idx_delta: i64,
    xtra_delta: f64,
}

impl TransposeTrigger {
    pub fn new(key_id_val: usize, i_del_val: i64, x_del_val: f64) -> TransposeTrigger {
        return TransposeTrigger {
            triggers: vec![key_id_val],
            idx_delta: i_del_val,
            xtra_delta: x_del_val,
        };
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Pluck {
    pluck: BtnToggle,
    idx_out: usize,
    xtra_out: f64,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ValveSet {
    buttons: Vec<DeltaToggle>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl ValveSet {
    pub fn init_set(&mut self, plucks: usize) {
        self.insert_btn(0usize, plucks);
    }
    pub fn insert_btn(&mut self, btn_idx: usize, plucks: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, DeltaToggle::default());
            self.init_btn(btn_idx, plucks);
        }
    }
    pub fn init_btn(&mut self, btn_idx: usize, plucks: usize) {
        for _ in 0..plucks {
            self.buttons[btn_idx].idx_deltas.push(0i64);
            self.buttons[btn_idx].xtra_deltas.push(0.0f64);
        }
    }
    pub fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
    pub fn insert_pluck(&mut self, p_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].idx_deltas.insert(p_idx, 0i64);
            self.buttons[btn].xtra_deltas.insert(p_idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, p_idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.remove(p_idx);
            self.buttons[b].xtra_deltas.remove(p_idx);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FretSet {
    buttons: Vec<DeltaToggle>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl FretSet {
    pub fn init_set(&mut self, plucks: usize) {
        self.insert_btn(0usize, plucks);
    }
    pub fn insert_btn(&mut self, btn_idx: usize, plucks: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, DeltaToggle::default());
            self.init_btn(btn_idx, plucks);
        }
    }
    pub fn init_btn(&mut self, btn_idx: usize, plucks: usize) {
        for _ in 0..plucks {
            self.buttons[btn_idx].idx_deltas.push(0i64);
            self.buttons[btn_idx].xtra_deltas.push(0.0f64);
        }
    }
    pub fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
    pub fn insert_pluck(&mut self, p_idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.insert(p_idx, 0i64);
            self.buttons[b].xtra_deltas.insert(p_idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, p_idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.remove(p_idx);
            self.buttons[b].xtra_deltas.remove(p_idx);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadioSet {
    buttons: Vec<DeltaToggle>,
    max_pressed: u8,
    min_pressed: u8,
    pressed: VecDeque<usize>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl RadioSet {
    pub fn init_set(&mut self, plucks: usize) {
        self.insert_btn(0usize, plucks);
        self.max_pressed = 1u8;
    }
    pub fn insert_btn(&mut self, btn_idx: usize, plucks: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, DeltaToggle::default());
            self.init_btn(btn_idx, plucks);
        }
    }
    pub fn init_btn(&mut self, btn_idx: usize, plucks: usize) {
        for _ in 0..plucks {
            self.buttons[btn_idx].idx_deltas.push(0i64);
            self.buttons[btn_idx].xtra_deltas.push(0.0f64);
        }
    }
    pub fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
    pub fn insert_pluck(&mut self, p_idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.insert(p_idx, 0i64);
            self.buttons[b].xtra_deltas.insert(p_idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, p_idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.remove(p_idx);
            self.buttons[b].xtra_deltas.remove(p_idx);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Combo {
    combo: Vec<bool>,
    idx_deltas: Vec<i64>,
    xtra_deltas: Vec<f64>,
    transpose_one: Vec<TransposeTrigger>,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AeroSet {
    buttons: Vec<BtnToggle>,
    combos: Vec<Combo>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl AeroSet {
    pub fn init_set(&mut self, plucks: usize) {
        self.buttons.insert(0usize, BtnToggle::default());
        self.insert_combo(0usize, plucks);
    }

    pub fn insert_btn(&mut self, btn_idx: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, BtnToggle::default());
            for c in 0..self.combos.len() {
                self.combos[c].combo.insert(btn_idx, false);
            }
        }
    }
    pub fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
            for c in 0..self.combos.len() {
                self.combos[c].combo.remove(btn_idx);
            }
        }
    }
    pub fn insert_combo(&mut self, c_idx: usize, plucks: usize) {
        if c_idx <= self.combos.len() {
            self.combos.insert(c_idx, Combo::default());
            for _ in 0..self.buttons.len() {
                self.combos[c_idx].combo.push(false);
            }
            for _ in 0..plucks {
                self.combos[c_idx].idx_deltas.push(0i64);
                self.combos[c_idx].xtra_deltas.push(0.0f64);
            }
        }
    }
    pub fn remove_combo(&mut self, c_idx: usize) {
        if self.combos.len() > 0 && c_idx < self.combos.len() {
            self.combos.remove(c_idx);
        }
    }
    pub fn insert_pluck(&mut self, p_idx: usize) {
        for c in 0..self.buttons.len() {
            self.combos[c].idx_deltas.insert(p_idx, 0i64);
            self.combos[c].xtra_deltas.insert(p_idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, p_idx: usize) {
        for c in 0..self.buttons.len() {
            self.combos[c].idx_deltas.remove(p_idx);
            self.combos[c].xtra_deltas.remove(p_idx);
        }
    }
}
