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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaToggle {
    toggles: Vec<usize>,
    pressed: bool,
    idx_deltas: Vec<i64>,
    xtra_deltas: Vec<f64>,
    transpose_one: Vec<TransposeTrigger>,
}

impl Default for DeltaToggle {
    fn default() -> DeltaToggle {
        return DeltaToggle {
            toggles: vec![],
            pressed: false,
            idx_deltas: vec![0i64],
            xtra_deltas: vec![0.0f64],
            transpose_one: vec![],
        };
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValveSet {
    buttons: Vec<DeltaToggle>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl Default for ValveSet {
    fn default() -> ValveSet {
        return ValveSet {
            buttons: vec![DeltaToggle::default()],
            holds: HoldBtns::default(),
            transpose_all: vec![],
        };
    }
}

impl ValveSet {
    pub fn insert_pluck(&mut self, idx: usize) {
        for b in 1..self.buttons.len() {
            self.buttons[b].idx_deltas.insert(idx, 0i64);
            self.buttons[b].xtra_deltas.insert(idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.remove(idx);
            self.buttons[b].xtra_deltas.remove(idx);
        }
    }
    pub fn insert_set(&mut self, plucks: usize) {
        for _ in 0..plucks {
            self.buttons[0].idx_deltas.push(0i64);
            self.buttons[0].xtra_deltas.push(0.0f64);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FretSet {
    buttons: Vec<DeltaToggle>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl Default for FretSet {
    fn default() -> FretSet {
        return FretSet {
            buttons: vec![DeltaToggle::default()],
            holds: HoldBtns::default(),
            transpose_all: vec![],
        };
    }
}

impl FretSet {
    pub fn insert_pluck(&mut self, idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.insert(idx, 0i64);
            self.buttons[b].xtra_deltas.insert(idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.remove(idx);
            self.buttons[b].xtra_deltas.remove(idx);
        }
    }
    pub fn insert_set(&mut self, plucks: usize) {
        for _ in 1..plucks {
            self.buttons[0].idx_deltas.push(0i64);
            self.buttons[0].xtra_deltas.push(0.0f64);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioSet {
    buttons: Vec<DeltaToggle>,
    max_pressed: u8,
    min_pressed: u8,
    pressed: VecDeque<usize>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl Default for RadioSet {
    fn default() -> RadioSet {
        return RadioSet {
            buttons: vec![DeltaToggle::default()],
            max_pressed: 1u8,
            min_pressed: 0u8,
            pressed: VecDeque::new(),
            holds: HoldBtns::default(),
            transpose_all: vec![],
        };
    }
}

impl RadioSet {
    pub fn insert_pluck(&mut self, idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.insert(idx, 0i64);
            self.buttons[b].xtra_deltas.insert(idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, idx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].idx_deltas.remove(idx);
            self.buttons[b].xtra_deltas.remove(idx);
        }
    }
    pub fn insert_set(&mut self, plucks: usize) {
        for _ in 1..plucks {
            self.buttons[0].idx_deltas.push(0i64);
            self.buttons[0].xtra_deltas.push(0.0f64);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combo {
    combo: Vec<bool>,
    idx_deltas: Vec<i64>,
    xtra_deltas: Vec<f64>,
    transpose_one: Vec<TransposeTrigger>,
}

impl Default for Combo {
    fn default() -> Combo {
        return Combo {
            combo: vec![],
            idx_deltas: vec![0i64],
            xtra_deltas: vec![0.0f64],
            transpose_one: vec![],
        };
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AeroSet {
    buttons: Vec<BtnToggle>,
    combos: Vec<Combo>,
    holds: HoldBtns,
    transpose_all: Vec<TransposeTrigger>,
}

impl Default for AeroSet {
    fn default() -> AeroSet {
        let mut aero_set: AeroSet = AeroSet {
            buttons: vec![BtnToggle::default()],
            combos: vec![Combo::default()],
            holds: HoldBtns::default(),
            transpose_all: vec![],
        };
        aero_set.combos[0].combo.push(true);
        return aero_set;
    }
}

impl AeroSet {
    pub fn insert_pluck(&mut self, idx: usize) {
        for c in 0..self.buttons.len() {
            self.combos[c].idx_deltas.insert(idx, 0i64);
            self.combos[c].xtra_deltas.insert(idx, 0f64);
        }
    }
    pub fn remove_pluck(&mut self, idx: usize) {
        for c in 0..self.buttons.len() {
            self.combos[c].idx_deltas.remove(idx);
            self.combos[c].xtra_deltas.remove(idx);
        }
    }
    pub fn insert_set(&mut self, plucks: usize) {
        for _ in 1..plucks {
            self.combos[0].idx_deltas.push(0i64);
            self.combos[0].xtra_deltas.push(0.0f64);
        }
    }
}