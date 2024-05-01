use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct BtnTog {
    togs: Vec<usize>,
    pressed: bool,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct HoldBtns {
    sustain: BtnTog,
    inv_sustain: BtnTog,
    sostenuto: BtnTog,
    inv_sostenuto: BtnTog,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct TrnspPluck {
    triggers: Vec<usize>,
    idx_delta: i64,
    xtra_delta: f64,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct Pluck {
    pluck: BtnTog,
    idx_out: usize,
    xtra_out: f64,
    trnsp_pluck: Vec<TrnspPluck>,
    tp_i_mem: i64,
    tp_x_mem: f64,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TrnspSet {
    triggers: Vec<usize>,
    idx_delta: Vec<i64>,
    xtra_delta: Vec<f64>,
}

impl TrnspSet {
    pub(crate) fn new_tp_key(key_idx_vals: Vec<usize>, plucks: usize) -> Self {
        return TrnspSet {
            triggers: key_idx_vals,
            idx_delta: vec![0i64; plucks],
            xtra_delta: vec![0.0f64; plucks],
        };
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DeltaTog {
    togs: Vec<usize>,
    pressed: bool,
    idx_deltas: Vec<i64>,
    xtra_deltas: Vec<f64>,
    trnsp_one: Vec<TrnspSet>,
    tp_i_mem: Vec<i64>,
    tp_x_mem: Vec<f64>,
}

impl DeltaTog {
    pub(crate) fn new(plucks: usize) -> Self {
        return DeltaTog {
            togs: vec![],
            pressed: false,
            idx_deltas: vec![0i64; plucks],
            xtra_deltas: vec![0.0f64; plucks],
            trnsp_one: vec![],
            tp_i_mem: vec![0i64; plucks],
            tp_x_mem: vec![0.0f64; plucks],
        }
    }
    pub(crate) fn insert_pluck(&mut self, p_idx: usize) {
        self.idx_deltas.insert(p_idx, 0i64);
        self.xtra_deltas.insert(p_idx, 0.0f64);
        self.tp_i_mem.insert(p_idx, 0i64);
        self.tp_x_mem.insert(p_idx, 0.0f64);
    }
    pub(crate) fn remove_pluck(&mut self, p_idx: usize) {
        self.idx_deltas.remove(p_idx);
        self.xtra_deltas.remove(p_idx);
        self.tp_i_mem.remove(p_idx);
        self.tp_x_mem.remove(p_idx);
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IndvSet {
    buttons: Vec<DeltaTog>,
    max_pressed: u8,
    min_pressed: u8,
    pressed: VecDeque<usize>,
    holds: HoldBtns,
    trnsp_all: Vec<TrnspSet>,
}

impl IndvSet {
    pub(crate) fn new(plucks: usize) -> Self {
        return IndvSet {
            buttons: vec![DeltaTog::new(plucks)],
            max_pressed: 1u8,
            min_pressed: 0u8,
            pressed: VecDeque::new(),
            holds: HoldBtns::default(),
            trnsp_all: vec![],
        };
    }
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, plucks: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, DeltaTog::new(plucks));
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
    pub(crate) fn insert_pluck(&mut self, p_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].insert_pluck(p_idx);
        }
    }
    pub(crate) fn remove_pluck(&mut self, p_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].remove_pluck(p_idx);
        }
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Combo {
    combo: Vec<bool>,
    idx_deltas: Vec<i64>,
    xtra_deltas: Vec<f64>,
    trnsp_one: Vec<TrnspSet>,
    tp_i_mem: Vec<i64>,
    tp_x_mem: Vec<f64>,
}

impl Combo {
    pub(crate) fn new(plucks: usize, btns: usize) -> Self {
        return Combo {
            combo: vec![false; btns],
            idx_deltas: vec![0i64; plucks],
            xtra_deltas: vec![0.0f64; plucks],
            trnsp_one: vec![],
            tp_i_mem: vec![0i64; plucks],
            tp_x_mem: vec![0.0f64; plucks],
        }
    }
    pub(crate) fn insert_pluck(&mut self, p_idx: usize) {
        self.idx_deltas.insert(p_idx, 0i64);
        self.xtra_deltas.insert(p_idx, 0.0f64);
        self.tp_i_mem.insert(p_idx, 0i64);
        self.tp_x_mem.insert(p_idx, 0.0f64);
    }
    pub(crate) fn remove_pluck(&mut self, p_idx: usize) {
        self.idx_deltas.remove(p_idx);
        self.xtra_deltas.remove(p_idx);
        self.tp_i_mem.remove(p_idx);
        self.tp_x_mem.remove(p_idx);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ComboSet {
    buttons: Vec<BtnTog>,
    combos: Vec<Combo>,
    holds: HoldBtns,
    trnsp_all: Vec<TrnspSet>,
}

impl ComboSet {
    pub(crate) fn new(plucks: usize) -> Self {
        return ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(plucks, 1usize)],
            holds: HoldBtns::default(),
            trnsp_all: vec![],
        };
    }
    pub(crate) fn insert_btn(&mut self, btn_idx: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, BtnTog::default());
            for c in 0..self.combos.len() {
                self.combos[c].combo.insert(btn_idx, false);
            }
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
            for c in 0..self.combos.len() {
                self.combos[c].combo.remove(btn_idx);
            }
        }
    }
    pub(crate) fn insert_combo(&mut self, c_idx: usize, plucks: usize) {
        if c_idx <= self.combos.len() {
            self.combos.insert(c_idx, Combo::new(plucks, self.buttons.len()));
        }
    }
    pub(crate) fn remove_combo(&mut self, c_idx: usize) {
        if self.combos.len() > 0 && c_idx < self.combos.len() {
            self.combos.remove(c_idx);
        }
    }
    pub(crate) fn insert_pluck(&mut self, p_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].insert_pluck(p_idx);
        }
    }
    pub(crate) fn remove_pluck(&mut self, p_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].remove_pluck(p_idx);
        }
    }
}
