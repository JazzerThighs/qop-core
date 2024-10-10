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
    pub(crate) fn new(plucks: usize) -> Self {
        return TrnspSet {
            triggers: vec![],
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
        };
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
    pub(crate) tp_i_mem: Vec<i64>,
    pub(crate) tp_x_mem: Vec<f64>,
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
            tp_i_mem: vec![0i64; plucks],
            tp_x_mem: vec![0.0f64; plucks],
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
            self.tp_i_mem.insert(p_idx, 0i64);
            self.tp_x_mem.insert(p_idx, 0.0f64);
        }
    }
    pub(crate) fn remove_pluck(&mut self, p_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].remove_pluck(p_idx);
            self.tp_i_mem.remove(p_idx);
            self.tp_x_mem.remove(p_idx);
        }
    }
    pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
            for t in 0..self.buttons[b].trnsp_one.len() {
                vec_closure(&mut self.buttons[b].trnsp_one[t].triggers);
            }
        }
        for t in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[t].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
    pub(crate) fn btn_insert_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
                self.buttons[btn_idx].togs.push(key_idx_val);
            }
        }
    }
    pub(crate) fn btn_remove_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
        }
    }
    pub(crate) fn hold_insert_key(&mut self, h_kind: u8, key_idx_val: usize) {
        match h_kind {
            0 => {
                if !self.holds.sustain.togs.contains(&key_idx_val) {
                    self.holds.sustain.togs.push(key_idx_val)
                }
            }
            1 => {
                if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
                    self.holds.inv_sustain.togs.push(key_idx_val)
                }
            }
            2 => {
                if !self.holds.sostenuto.togs.contains(&key_idx_val) {
                    self.holds.sostenuto.togs.push(key_idx_val)
                }
            }
            3 => {
                if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
                    self.holds.inv_sostenuto.togs.push(key_idx_val)
                }
            }
            _ => {}
        }
    }
    pub(crate) fn hold_remove_key(&mut self, h_kind: u8, key_idx_val: usize) {
        match h_kind {
            0 => self.holds.sustain.togs.retain(|&idx| idx != key_idx_val),
            1 => self
                .holds
                .inv_sustain
                .togs
                .retain(|&idx| idx != key_idx_val),
            2 => self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val),
            3 => self
                .holds
                .inv_sostenuto
                .togs
                .retain(|&idx| idx != key_idx_val),
            _ => {}
        }
    }
    pub(crate) fn trnsp_all_params(
        &mut self,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        plucks: usize,
    ) {
        if trnsp_idx == self.trnsp_all.len() {
            let mut tp: TrnspSet = TrnspSet::new(plucks);
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    tp.triggers.push(k);
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < plucks {
                    if let Some(delta) = i_del {
                        tp.idx_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < plucks {
                    if let Some(delta) = x_del {
                        tp.xtra_delta[x] = delta;
                    }
                }
            }
            self.trnsp_all.push(tp);
        } else if trnsp_idx < self.trnsp_all.len() {
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    if !self.trnsp_all[trnsp_idx].triggers.contains(&k) {
                        self.trnsp_all[trnsp_idx].triggers.push(k);
                    }
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < plucks {
                    if let Some(delta) = i_del {
                        self.trnsp_all[trnsp_idx].idx_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < plucks {
                    if let Some(delta) = x_del {
                        self.trnsp_all[trnsp_idx].xtra_delta[x] = delta;
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_all_remove_key(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all[trnsp_idx]
                .triggers
                .retain(|&idx| idx != key_idx_val);
        }
    }
    pub(crate) fn trnsp_all_remove(&mut self, trnsp_idx: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all.remove(trnsp_idx);
        }
    }
    pub(crate) fn trnsp_one_params(
        &mut self,
        btn_idx: usize,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        plucks: usize,
    ) {
        if btn_idx < self.buttons.len() {
            if trnsp_idx == self.buttons[btn_idx].trnsp_one.len() {
                let mut tp: TrnspSet = TrnspSet::new(plucks);
                for (_i, &key) in key_idx_vals.iter().enumerate() {
                    if let Some(k) = key {
                        tp.triggers.push(k);
                    }
                }
                for (i, &i_del) in i_del_vec.iter().enumerate() {
                    if i < plucks {
                        if let Some(delta) = i_del {
                            tp.idx_delta[i] = delta;
                        }
                    }
                }
                for (x, &x_del) in x_del_vec.iter().enumerate() {
                    if x < plucks {
                        if let Some(delta) = x_del {
                            tp.xtra_delta[x] = delta;
                        }
                    }
                }
                self.buttons[btn_idx].trnsp_one.push(tp);
            } else if trnsp_idx < self.buttons[btn_idx].trnsp_one.len() {
                for (_i, &key) in key_idx_vals.iter().enumerate() {
                    if let Some(k) = key {
                        if !self.buttons[btn_idx].trnsp_one[trnsp_idx]
                            .triggers
                            .contains(&k)
                        {
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].triggers.push(k);
                        }
                    }
                }
                for (i, &i_del) in i_del_vec.iter().enumerate() {
                    if i < plucks {
                        if let Some(delta) = i_del {
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].idx_delta[i] = delta;
                        }
                    }
                }
                for (x, &x_del) in x_del_vec.iter().enumerate() {
                    if x < plucks {
                        if let Some(delta) = x_del {
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].xtra_delta[x] = delta;
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_one_remove_key(
        &mut self,
        btn_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if btn_idx < self.buttons.len() {
            if trnsp_idx < self.buttons[btn_idx].trnsp_one.len() {
                self.buttons[btn_idx].trnsp_one[trnsp_idx]
                    .triggers
                    .retain(|&idx| idx != key_idx_val);
            }
        }
    }
    pub(crate) fn trnsp_one_remove(&mut self, btn_idx: usize, trnsp_idx: usize) {
        if btn_idx < self.buttons.len() {
            if trnsp_idx < self.buttons[btn_idx].trnsp_one.len() {
                self.buttons[btn_idx].trnsp_one.remove(trnsp_idx);
            }
        }
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
        };
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
    pub(crate) tp_i_mem: Vec<i64>,
    pub(crate) tp_x_mem: Vec<f64>,
}

impl ComboSet {
    pub(crate) fn new(plucks: usize) -> Self {
        return ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(plucks, 1usize)],
            holds: HoldBtns::default(),
            trnsp_all: vec![],
            tp_i_mem: vec![0i64; plucks],
            tp_x_mem: vec![0.0f64; plucks],
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
            self.combos
                .insert(c_idx, Combo::new(plucks, self.buttons.len()));
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
            self.tp_i_mem.insert(p_idx, 0i64);
            self.tp_x_mem.insert(p_idx, 0.0f64);
        }
    }
    pub(crate) fn remove_pluck(&mut self, p_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].remove_pluck(p_idx);
            self.tp_i_mem.remove(p_idx);
            self.tp_x_mem.remove(p_idx);
        }
    }
    pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
        }
        for c in 0..self.combos.len() {
            for t in 0..self.combos[c].trnsp_one.len() {
                vec_closure(&mut self.combos[c].trnsp_one[t].triggers);
            }
        }
        for t in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[t].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
    pub(crate) fn btn_insert_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
                self.buttons[btn_idx].togs.push(key_idx_val);
            }
        }
    }
    pub(crate) fn btn_remove_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
        }
    }
    pub(crate) fn hold_insert_key(&mut self, h_kind: u8, key_idx_val: usize) {
        match h_kind {
            0 => {
                if !self.holds.sustain.togs.contains(&key_idx_val) {
                    self.holds.sustain.togs.push(key_idx_val)
                }
            }
            1 => {
                if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
                    self.holds.inv_sustain.togs.push(key_idx_val)
                }
            }
            2 => {
                if !self.holds.sostenuto.togs.contains(&key_idx_val) {
                    self.holds.sostenuto.togs.push(key_idx_val)
                }
            }
            3 => {
                if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
                    self.holds.inv_sostenuto.togs.push(key_idx_val)
                }
            }
            _ => {}
        }
    }
    pub(crate) fn hold_remove_key(&mut self, h_kind: u8, key_idx_val: usize) {
        match h_kind {
            0 => self.holds.sustain.togs.retain(|&idx| idx != key_idx_val),
            1 => self
                .holds
                .inv_sustain
                .togs
                .retain(|&idx| idx != key_idx_val),
            2 => self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val),
            3 => self
                .holds
                .inv_sostenuto
                .togs
                .retain(|&idx| idx != key_idx_val),
            _ => {}
        }
    }
    pub(crate) fn trnsp_all_params(
        &mut self,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        plucks: usize,
    ) {
        if trnsp_idx == self.trnsp_all.len() {
            let mut tp: TrnspSet = TrnspSet::new(plucks);
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    tp.triggers.push(k);
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < plucks {
                    if let Some(delta) = i_del {
                        tp.idx_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < plucks {
                    if let Some(delta) = x_del {
                        tp.xtra_delta[x] = delta;
                    }
                }
            }
            self.trnsp_all.push(tp);
        } else if trnsp_idx < self.trnsp_all.len() {
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    if !self.trnsp_all[trnsp_idx].triggers.contains(&k) {
                        self.trnsp_all[trnsp_idx].triggers.push(k);
                    }
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < plucks {
                    if let Some(delta) = i_del {
                        self.trnsp_all[trnsp_idx].idx_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < plucks {
                    if let Some(delta) = x_del {
                        self.trnsp_all[trnsp_idx].xtra_delta[x] = delta;
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_all_remove_key(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all[trnsp_idx]
                .triggers
                .retain(|&idx| idx != key_idx_val);
        }
    }
    pub(crate) fn trnsp_all_remove(&mut self, trnsp_idx: usize) {
        if trnsp_idx < self.trnsp_all.len() {
            self.trnsp_all.remove(trnsp_idx);
        }
    }
    pub(crate) fn trnsp_one_params(
        &mut self,
        c_idx: usize,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        plucks: usize,
    ) {
        if c_idx < self.combos.len() {
            if c_idx < self.buttons.len() {
                if trnsp_idx == self.combos[c_idx].trnsp_one.len() {
                    let mut tp: TrnspSet = TrnspSet::new(plucks);
                    for (_i, &key) in key_idx_vals.iter().enumerate() {
                        if let Some(k) = key {
                            tp.triggers.push(k);
                        }
                    }
                    for (i, &i_del) in i_del_vec.iter().enumerate() {
                        if i < plucks {
                            if let Some(delta) = i_del {
                                tp.idx_delta[i] = delta;
                            }
                        }
                    }
                    for (x, &x_del) in x_del_vec.iter().enumerate() {
                        if x < plucks {
                            if let Some(delta) = x_del {
                                tp.xtra_delta[x] = delta;
                            }
                        }
                    }
                    self.combos[c_idx].trnsp_one.push(tp);
                } else if trnsp_idx < self.combos[c_idx].trnsp_one.len() {
                    for (_i, &key) in key_idx_vals.iter().enumerate() {
                        if let Some(k) = key {
                            if !self.combos[c_idx].trnsp_one[trnsp_idx]
                                .triggers
                                .contains(&k)
                            {
                                self.combos[c_idx].trnsp_one[trnsp_idx].triggers.push(k);
                            }
                        }
                    }
                    for (i, &i_del) in i_del_vec.iter().enumerate() {
                        if i < plucks {
                            if let Some(delta) = i_del {
                                self.combos[c_idx].trnsp_one[trnsp_idx].idx_delta[i] = delta;
                            }
                        }
                    }
                    for (x, &x_del) in x_del_vec.iter().enumerate() {
                        if x < plucks {
                            if let Some(delta) = x_del {
                                self.combos[c_idx].trnsp_one[trnsp_idx].xtra_delta[x] = delta;
                            }
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_one_remove_key(
        &mut self,
        c_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if c_idx < self.combos.len() {
            if trnsp_idx < self.combos[c_idx].trnsp_one.len() {
                self.combos[c_idx].trnsp_one[trnsp_idx]
                    .triggers
                    .retain(|&idx| idx != key_idx_val);
            }
        }
    }
    pub(crate) fn trnsp_one_remove(&mut self, c_idx: usize, trnsp_idx: usize) {
        if c_idx < self.combos.len() {
            if trnsp_idx < self.combos[c_idx].trnsp_one.len() {
                self.combos[c_idx].trnsp_one.remove(trnsp_idx);
            }
        }
    }
}
