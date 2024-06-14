use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct BtnTog {
    pub(crate) togs: Vec<usize>,
    pub(crate) pressed: bool,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct HoldBtns {
    pub(crate) sustain: BtnTog,
    pub(crate) inv_sustain: BtnTog,
    pub(crate) sostenuto: BtnTog,
    pub(crate) inv_sostenuto: BtnTog,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct TrnspPluck {
    pub(crate) triggers: Vec<usize>,
    pub(crate) idx_delta: i64,
    pub(crate) xtra_delta: f64,
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct Pluck {
    pub(crate) pluck: BtnTog,
    pub(crate) idx_out: usize,
    pub(crate) xtra_out: f64,
    pub(crate) trnsp_pluck: Vec<TrnspPluck>,
    pub(crate) tp_i_mem: i64,
    pub(crate) tp_x_mem: f64,
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
    pub(crate) togs: Vec<usize>,
    pub(crate) pressed: bool,
    pub(crate) idx_deltas: Vec<i64>,
    pub(crate) xtra_deltas: Vec<f64>,
    pub(crate) trnsp_one: Vec<TrnspSet>,
    pub(crate) tp_i_mem: Vec<i64>,
    pub(crate) tp_x_mem: Vec<f64>,
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
    pub(crate) buttons: Vec<DeltaTog>,
    pub(crate) max_pressed: u8,
    pub(crate) min_pressed: u8,
    pub(crate) pressed: VecDeque<usize>,
    pub(crate) holds: HoldBtns,
    pub(crate) trnsp_all: Vec<TrnspSet>,
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
    pub(crate) fn remove_key_idx(&mut self, kdx: usize) {
        for b in 0..self.buttons.len() {
            self.buttons[b].togs.retain(|&k| k != kdx);
            for t in 0..self.buttons[b].trnsp_one.len() {
                self.buttons[b].trnsp_one[t].triggers.retain(|&k| k != kdx);
            }
        }
        for t in 0..self.trnsp_all.len() {
            self.trnsp_all[t].triggers.retain(|&k| k != kdx);
        }
        self.holds.sustain.togs.retain(|&k| k != kdx);
        self.holds.inv_sostenuto.togs.retain(|&k| k != kdx);
        self.holds.sostenuto.togs.retain(|&k| k != kdx);
        self.holds.inv_sostenuto.togs.retain(|&k| k != kdx);
    }
}

/* ************************************************************************* */

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Combo {
    pub(crate) combo: Vec<bool>,
    pub(crate) idx_deltas: Vec<i64>,
    pub(crate) xtra_deltas: Vec<f64>,
    pub(crate) trnsp_one: Vec<TrnspSet>,
    pub(crate) tp_i_mem: Vec<i64>,
    pub(crate) tp_x_mem: Vec<f64>,
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
    pub(crate) buttons: Vec<BtnTog>,
    pub(crate) combos: Vec<Combo>,
    pub(crate) holds: HoldBtns,
    pub(crate) trnsp_all: Vec<TrnspSet>,
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
