
use crate::engine::*;
use better_default::Default;
use duplicate::duplicate_item;
use std::cmp::Ordering::{Equal, Greater, Less};
use winit::keyboard::KeyCode;

#[derive(Default, Clone)]
pub struct NewEnginePartParams {
    pub guts_len: usize,
    pub g_idx: usize,
    pub set_idx: usize,
    pub del_idx: usize,
    pub trnsp_idx: usize,
    pub c_btn_len: usize,
}

impl NewEnginePartParams {
    pub fn new(engine: &Engine) -> Self {
        NewEnginePartParams {
            guts_len: engine.guts.len(),
            c_btn_len: 1,
            ..Self::default()
        }
    }
}

pub trait NewTrait: Default {
    fn new(n: &mut NewEnginePartParams) -> Self;
}

impl NewTrait for VFSet {
    fn new(n: &mut NewEnginePartParams) -> Self {
        VFSet {
            buttons: vec![VFBtn::new(n)],
            i_mem: vec![0; n.guts_len],
            x_mem: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for VFBtn {
    fn new(n: &mut NewEnginePartParams) -> Self {
        VFBtn {
            i_delta: vec![0; n.guts_len],
            x_delta: vec![0.0; n.guts_len],
            i_mem: vec![0; n.guts_len],
            x_mem: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for ComboSet {
    fn new(n: &mut NewEnginePartParams) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            combos: vec![Combo::new(n)],
            i_mem: vec![0; n.guts_len],
            x_mem: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for Combo {
    fn new(n: &mut NewEnginePartParams) -> Self {
        Combo {
            combo: vec![false; n.c_btn_len],
            i_delta: vec![0; n.guts_len],
            x_delta: vec![0.0; n.guts_len],
            i_mem: vec![0; n.guts_len],
            x_mem: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for Trnsp {
    fn new(_n: &mut NewEnginePartParams) -> Self {
        Self::default()
    }
}

impl NewTrait for MulTrnsp {
    fn new(n: &mut NewEnginePartParams) -> Self {
        MulTrnsp {
            i_delta: vec![0; n.guts_len],
            x_delta: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for MulAnalogMod {
    fn new(n: &mut NewEnginePartParams) -> Self {
        MulAnalogMod {
            i_mem: vec![0; n.guts_len],
            x_mem: vec![0.0; n.guts_len],
            pot_input_nodes: vec![
                MulAnalogModNode {
                    pot_value: 0,
                    i_del: vec![0; n.guts_len],
                    x_del: vec![0.0; n.guts_len],
                };
                2
            ],
            ..Default::default()
        }
    }
}



/**************************** _dig_inputs.rs ********************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************/

macro_rules! assert_lt_expr {
    ($left:expr, $right:expr) => {
        if !($left < $right) {
            return Err(format!(
                "Assertion failed: `{}` < `{}`\n(left: `{:?}`, right: `{:?}`)",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

impl Engine<Edit> {
    pub fn dig_inputs_insert_k(&mut self, key_code: KeyCode) {
        if !self.dig_inputs.contains(&key_code) {
            self.dig_inputs.push(key_code)
        }
    }
    pub fn dig_inputs_global_vec_manip(&mut self, operation: impl Fn(&mut Vec<usize>)) {
        for g in 0..self.guts.len() {
            operation(&mut self.guts[g].togs);
            for tg in 0..self.guts[g].trnsp_gut.len() {
                operation(&mut self.guts[g].trnsp_gut[tg].triggers);
            }
        }
        operation(&mut self.gut_holds.sustain.togs);
        operation(&mut self.gut_holds.inv_sustain.togs);
        operation(&mut self.gut_holds.sostenuto.togs);
        operation(&mut self.gut_holds.inv_sostenuto.togs);
        for set in 0..self.v_multi.len() {
            self.v_multi[set].all_dig_idx_vecs(&operation);
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].all_dig_idx_vecs(&operation);
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].all_dig_idx_vecs(&operation);
        }
    }
    pub fn dig_inputs_purge_dig(&mut self, key_code: KeyCode) {
        if let Some(i) = self.dig_inputs.iter().position(|&key| key == key_code) {
            let dig_idx_purge = |key_idx_vec: &mut Vec<usize>| {
                key_idx_vec.retain_mut(|k: &mut usize| match (*k).cmp(&i) {
                    Less => true,
                    Equal => false,
                    Greater => {
                        *k -= 1;
                        true
                    }
                })
            };
            Engine::dig_inputs_global_vec_manip(self, dig_idx_purge);
            self.dig_inputs.remove(i);
        }
    }
    pub fn dig_inputs_swap_idxs(&mut self, kc1: KeyCode, kc2: KeyCode, swap_all_fields: bool) {
        // This function swaps 2 digital input values for 2 existing inputs in the dig_inputs field.
        // swap_all_fields == true -> swaps the nodes of 2 existing keys in all of rest of the Qop's fields,  so all of those affected usize values would be now pointing to the same keys as before.
        // swap_all_fields == false -> leaves all of the rest of the Qop's fields alone, so all of those affected usize values would be now pointing to swapped keys.
        let i1 = self.dig_inputs.iter().position(|&key| key == kc1);
        let i2 = self.dig_inputs.iter().position(|&key| key == kc2);
        if let (Some(i1), Some(i2)) = (i1, i2) {
            self.dig_inputs.swap(i1, i2);
            if swap_all_fields {
                let k_idxs_swap = |k_idx_vec: &mut Vec<usize>| {
                    k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                        match ((*k).cmp(&i1), (*k).cmp(&i2)) {
                            (Equal, _) => *k = i2,
                            (_, Equal) => *k = i1,
                            (_, _) => {}
                        }
                    });
                };
                Engine::dig_inputs_global_vec_manip(self, k_idxs_swap);
            }
        }
    }
    pub fn dig_inputs_change_idx_to(&mut self, kc_old: KeyCode, kc_new: KeyCode) {
        let i1 = self.dig_inputs.iter().position(|&key| key == kc_old);
        let i2 = self.dig_inputs.iter().position(|&key| key == kc_new);
        if let (Some(i1), Some(i2)) = (i1, i2) {
            let k_idx_update = |k_idx_vec: &mut Vec<usize>| {
                k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                    if *k == i1 {
                        *k = i2;
                    }
                });
                k_idx_vec.sort();
                k_idx_vec.dedup();
            };
            Engine::dig_inputs_global_vec_manip(self, k_idx_update);
        }
    }
    pub fn check_digitalref_invariants(&self) -> Result<(), String> {
        for g in 0..self.guts.len() {
            for t in 0..self.guts[g].togs.len() {
                assert_lt_expr!(self.guts[g].togs[t], self.dig_inputs.len())
            }
            for tg in 0..self.guts[g].trnsp_gut.len() {
                for t in 0..self.guts[g].trnsp_gut[tg].triggers.len() {
                    assert_lt_expr!(
                        self.guts[g].trnsp_gut[tg].triggers[t],
                        self.dig_inputs.len()
                    )
                }
            }
        }

        self.gut_holds
            .check_digitalref_invariants(self.dig_inputs.len())?;

        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_digitalref_invariants(self.dig_inputs.len())?;
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_digitalref_invariants(self.dig_inputs.len())?;
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_digitalref_invariants(self.dig_inputs.len())?;
        }
        Ok(())
    }
}

#[duplicate_item(
    SetType    tofield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub fn all_dig_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
        }
        for c in 0..self.tofield.len() {
            for to in 0..self.tofield[c].trnsp_one.len() {
                vec_closure(&mut self.tofield[c].trnsp_one[to].triggers);
            }
        }
        for ta in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[ta].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
    pub fn check_digitalref_invariants(&self, dig_vec_len: usize) -> Result<(), String> {
        for b in 0..self.buttons.len() {
            for t in 0..self.buttons[b].togs.len() {
                assert_lt_expr!(self.buttons[b].togs[t], dig_vec_len)
            }
        }
        for c in 0..self.tofield.len() {
            for to in 0..self.tofield[c].trnsp_one.len() {
                for t in 0..self.tofield[c].trnsp_one[to].triggers.len() {
                    assert_lt_expr!(self.tofield[c].trnsp_one[to].triggers[t], dig_vec_len)
                }
            }
        }
        for ta in 0..self.trnsp_all.len() {
            for t in 0..self.trnsp_all[ta].triggers.len() {
                assert_lt_expr!(self.trnsp_all[ta].triggers[t], dig_vec_len)
            }
        }
        return self.holds.check_digitalref_invariants(dig_vec_len);
    }
}

impl HoldBtns {
    pub fn check_digitalref_invariants(&self, dig_vec_len: usize) -> Result<(), String> {
        for sus in 0..self.sustain.togs.len() {
            assert_lt_expr!(self.sustain.togs[sus], dig_vec_len)
        }
        for isus in 0..self.inv_sustain.togs.len() {
            assert_lt_expr!(self.inv_sustain.togs[isus], dig_vec_len)
        }
        for sos in 0..self.sostenuto.togs.len() {
            assert_lt_expr!(self.sostenuto.togs[sos], dig_vec_len)
        }
        for isos in 0..self.inv_sostenuto.togs.len() {
            assert_lt_expr!(self.inv_sostenuto.togs[isos], dig_vec_len)
        }
        Ok(())
    }
}

/********************************* _gut.rs **********************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************/

impl Engine<Edit> {
    pub fn gut_insert_g(&mut self, g_idx: usize) {
        if g_idx <= self.guts.len() {
            self.guts.insert(g_idx, Gut::default());
            self.v_multi
                .iter_mut()
                .for_each(|element| element.insert_gut(g_idx));
            self.f_multi
                .iter_mut()
                .for_each(|element| element.insert_gut(g_idx));
            self.c_multi
                .iter_mut()
                .for_each(|element| element.insert_gut(g_idx));
        }
    }
    pub fn gut_remove_g(&mut self, g_idx: usize) {
        if self.guts.len() > 1 && g_idx < self.guts.len() {
            self.guts.remove(g_idx);
            self.v_multi
                .iter_mut()
                .for_each(|element| element.remove_gut(g_idx));
            self.f_multi
                .iter_mut()
                .for_each(|element| element.remove_gut(g_idx));
            self.c_multi
                .iter_mut()
                .for_each(|element| element.remove_gut(g_idx));
        }
    }
    pub fn gut_insert_dig(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            if !self.guts[g_idx].togs.contains(&key_idx_val) {
                self.guts[g_idx].togs.push(key_idx_val)
            };
        }
    }
    pub fn gut_remove_dig(&mut self, g_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && key_idx_val < self.dig_inputs.len() {
            self.guts[g_idx].togs.retain(|&idx| idx != key_idx_val);
        }
    }
    pub fn gut_toggle_radio_mode(&mut self) {
        self.gut_radio_mode = !self.gut_radio_mode;
    }
    pub fn gut_insert_trnsp_t(&mut self, g_idx: usize, trnsp_idx: usize) {
        if g_idx < self.guts.len() && trnsp_idx <= self.guts[g_idx].trnsp_gut.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.guts[g_idx]
                .trnsp_gut
                .insert(trnsp_idx, Trnsp::new(&mut n));
        }
    }
    pub fn gut_remove_trnsp_t(&mut self, g_idx: usize, trnsp_idx: usize) {
        if g_idx < self.guts.len() && trnsp_idx < self.guts[g_idx].trnsp_gut.len() {
            self.guts[g_idx].trnsp_gut.remove(trnsp_idx);
        }
    }
    pub fn gut_insert_trnsp_dig(&mut self, g_idx: usize, trnsp_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len()
            && trnsp_idx < self.guts[g_idx].trnsp_gut.len()
            && !self.guts[g_idx].trnsp_gut[trnsp_idx]
                .triggers
                .contains(&key_idx_val)
        {
            self.guts[g_idx].trnsp_gut[trnsp_idx]
                .triggers
                .push(key_idx_val);
        }
    }
    pub fn gut_remove_trnsp_dig(&mut self, g_idx: usize, trnsp_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len() && trnsp_idx < self.guts[g_idx].trnsp_gut.len() {
            self.guts[g_idx].trnsp_gut[trnsp_idx]
                .triggers
                .retain(|&idx| idx != key_idx_val);
        }
    }
    pub fn check_gut_vec_lengths(&self) -> Result<(), String> {
        for i in 0..self.gut_analogs.len() {
            self.gut_analogs[i].check_gut_vec_lengths(self.guts.len())?
        }
        for i in 0..self.v_multi.len() {
            self.v_multi[i].check_gut_vec_lengths(self.guts.len())?;
        }
        for i in 0..self.f_multi.len() {
            self.f_multi[i].check_gut_vec_lengths(self.guts.len())?;
        }
        for i in 0..self.c_multi.len() {
            self.c_multi[i].check_gut_vec_lengths(self.guts.len())?;
        }
        Ok(())
    }
}

#[duplicate_item(
    gut_change_delta_out         d_out       d_del_val   del_type   gut_change_minmax        minmaxval minmax_field      iscomparedto                gut_trnsp_change_deltas     d_type d_field;
    [gut_change_index_delta_out] [index_out] [i_del_val] [usize]    [gut_change_min_pressed] [min_val] [gut_min_pressed] [le(&self.gut_max_pressed)] [gut_trnsp_change_i_deltas] [i32]    [i_delta];
    [gut_change_extra_delta_out] [extra_out] [x_del_val] [f64]      [gut_change_max_pressed] [max_val] [gut_max_pressed] [ge(&self.gut_max_pressed)] [gut_trnsp_change_x_deltas] [f64]    [x_delta];
)]
impl Engine<Edit> {
    pub fn gut_change_delta_out(&mut self, g_idx: usize, d_del_val: del_type) {
        if g_idx < self.guts.len() {
            self.guts[g_idx].d_out = d_del_val;
        }
    }
    pub fn gut_change_minmax(&mut self, minmaxval: usize) {
        if minmaxval.iscomparedto {
            self.minmax_field = minmaxval;
        }
    }
    pub fn gut_trnsp_change_deltas(&mut self, g_idx: usize, trnsp_idx: usize, d_del_val: d_type) {
        if g_idx < self.guts.len() && trnsp_idx < self.guts[g_idx].trnsp_gut.len() {
            self.guts[g_idx].trnsp_gut[trnsp_idx].d_field = d_del_val;
        }
    }
}

#[duplicate_item(
    SetType    multi_insertremove_gut insertremove_i        insertremove_x          deltafield;
    [VFSet]    [insert_gut]           [insert(g_idx, 0)]    [insert(g_idx, 0.0)]    [buttons];
    [VFSet]    [remove_gut]           [remove(g_idx)]       [remove(g_idx)]         [buttons];
    [ComboSet] [insert_gut]           [insert(g_idx, 0)]    [insert(g_idx, 0.0)]    [combos];
    [ComboSet] [remove_gut]           [remove(g_idx)]       [remove(g_idx)]         [combos];
)]
impl SetType {
    pub fn multi_insertremove_gut(&mut self, g_idx: usize) {
        for del_idx in 0..self.deltafield.len() {
            self.deltafield[del_idx].i_delta.insertremove_i;
            self.deltafield[del_idx].x_delta.insertremove_x;
            self.deltafield[del_idx].i_mem.insertremove_i;
            self.deltafield[del_idx].x_mem.insertremove_x;
            for to in 0..self.deltafield[del_idx].trnsp_one.len() {
                self.deltafield[del_idx].trnsp_one[to]
                    .i_delta
                    .insertremove_i;
                self.deltafield[del_idx].trnsp_one[to]
                    .x_delta
                    .insertremove_x;
            }
        }
        self.i_mem.insertremove_i;
        self.x_mem.insertremove_x;
        for ta in 0..self.trnsp_all.len() {
            self.trnsp_all[ta].i_delta.insertremove_i;
            self.trnsp_all[ta].x_delta.insertremove_x;
        }
    }
}

macro_rules! assert_eq_expr {
    ($left:expr, $right:expr) => {
        if $left != $right {
            return Err(format!(
                "Assertion failed: `{}` == `{}`\n(left: `{:?}`, right: `{:?}`)",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

#[duplicate_item(
    SetType    field;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub fn check_gut_vec_lengths(&self, guts_len: usize) -> Result<(), String> {
        for d in 0..self.field.len() {
            assert_eq_expr!(self.field[d].i_delta.len(), guts_len);
            assert_eq_expr!(self.field[d].x_delta.len(), guts_len);
            assert_eq_expr!(self.field[d].i_mem.len(), guts_len);
            assert_eq_expr!(self.field[d].x_mem.len(), guts_len);
            for to in 0..self.field[d].trnsp_one.len() {
                assert_eq_expr!(self.field[d].trnsp_one[to].i_delta.len(), guts_len);
                assert_eq_expr!(self.field[d].trnsp_one[to].x_delta.len(), guts_len);
            }
        }
        assert_eq_expr!(self.i_mem.len(), guts_len);
        assert_eq_expr!(self.x_mem.len(), guts_len);
        for ta in 0..self.trnsp_all.len() {
            assert_eq_expr!(self.trnsp_all[ta].i_delta.len(), guts_len);
            assert_eq_expr!(self.trnsp_all[ta].x_delta.len(), guts_len);
        }
        Ok(())
    }
}

/**************************** _holds.rs *************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************/

#[duplicate_item(
    gut_hold_dig                    holdfield       operation                          ;
    [gut_sustain_insert_dig]        [sustain]       [push(key_idx_val)]                ;
    [gut_inv_sustain_insert_dig]    [inv_sustain]   [push(key_idx_val)]                ;
    [gut_sostenuto_insert_dig]      [sostenuto]     [push(key_idx_val)]                ;
    [gut_inv_sostenuto_insert_dig]  [inv_sostenuto] [push(key_idx_val)]                ;
    [gut_sustain_remove_dig]        [sustain]       [retain(|&idx| idx != key_idx_val)];
    [gut_inv_sustain_remove_dig]    [inv_sustain]   [retain(|&idx| idx != key_idx_val)];
    [gut_sostenuto_remove_dig]      [sostenuto]     [retain(|&idx| idx != key_idx_val)];
    [gut_inv_sostenuto_remove_dig]  [inv_sostenuto] [retain(|&idx| idx != key_idx_val)];
)]
impl Engine<Edit> {
    pub fn gut_hold_dig(&mut self, key_idx_val: usize) {
        self.gut_holds.holdfield.togs.operation;
        self.gut_holds.holdfield.togs.dedup()
    }
}

#[duplicate_item(
    multi_hold_dig                     multifield holdfield       operation                          ;
    [v_multi_sustain_insert_dig]       [v_multi]  [sustain]       [push(key_idx_val)]                ;
    [v_multi_inv_sustain_insert_dig]   [v_multi]  [inv_sustain]   [push(key_idx_val)]                ;
    [v_multi_sostenuto_insert_dig]     [v_multi]  [sostenuto]     [push(key_idx_val)]                ;
    [v_multi_inv_sostenuto_insert_dig] [v_multi]  [inv_sostenuto] [push(key_idx_val)]                ;
    [f_multi_sustain_insert_dig]       [f_multi]  [sustain]       [push(key_idx_val)]                ;
    [f_multi_inv_sustain_insert_dig]   [f_multi]  [inv_sustain]   [push(key_idx_val)]                ;
    [f_multi_sostenuto_insert_dig]     [f_multi]  [sostenuto]     [push(key_idx_val)]                ;
    [f_multi_inv_sostenuto_insert_dig] [f_multi]  [inv_sostenuto] [push(key_idx_val)]                ;
    [c_multi_sustain_insert_dig]       [c_multi]  [sustain]       [push(key_idx_val)]                ;
    [c_multi_inv_sustain_insert_dig]   [c_multi]  [inv_sustain]   [push(key_idx_val)]                ;
    [c_multi_sostenuto_insert_dig]     [c_multi]  [sostenuto]     [push(key_idx_val)]                ;
    [c_multi_inv_sostenuto_insert_dig] [c_multi]  [inv_sostenuto] [push(key_idx_val)]                ;
    [v_multi_sustain_remove_dig]       [v_multi]  [sustain]       [retain(|&idx| idx != key_idx_val)];
    [v_multi_inv_sustain_remove_dig]   [v_multi]  [inv_sustain]   [retain(|&idx| idx != key_idx_val)];
    [v_multi_sostenuto_remove_dig]     [v_multi]  [sostenuto]     [retain(|&idx| idx != key_idx_val)];
    [v_multi_inv_sostenuto_remove_dig] [v_multi]  [inv_sostenuto] [retain(|&idx| idx != key_idx_val)];
    [f_multi_sustain_remove_dig]       [f_multi]  [sustain]       [retain(|&idx| idx != key_idx_val)];
    [f_multi_inv_sustain_remove_dig]   [f_multi]  [inv_sustain]   [retain(|&idx| idx != key_idx_val)];
    [f_multi_sostenuto_remove_dig]     [f_multi]  [sostenuto]     [retain(|&idx| idx != key_idx_val)];
    [f_multi_inv_sostenuto_remove_dig] [f_multi]  [inv_sostenuto] [retain(|&idx| idx != key_idx_val)];
    [c_multi_sustain_remove_dig]       [c_multi]  [sustain]       [retain(|&idx| idx != key_idx_val)];
    [c_multi_inv_sustain_remove_dig]   [c_multi]  [inv_sustain]   [retain(|&idx| idx != key_idx_val)];
    [c_multi_sostenuto_remove_dig]     [c_multi]  [sostenuto]     [retain(|&idx| idx != key_idx_val)];
    [c_multi_inv_sostenuto_remove_dig] [c_multi]  [inv_sostenuto] [retain(|&idx| idx != key_idx_val)];
)]
impl Engine<Edit> {
    pub fn multi_hold_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].holds.holdfield.togs.operation;
            self.multifield[set_idx].holds.holdfield.togs.dedup();
        }
    }
}

/**************************** _trnsp.rs ************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************/

#[duplicate_item(
    multifield deltafield multi_insert_trnsp_all_t     multi_remove_trnsp_all_t     multi_insert_trnsp_all_dig     multi_remove_trnsp_all_dig     multi_insert_trnsp_one_t     multi_remove_trnsp_one_t     multi_insert_trnsp_one_dig     multi_remove_trnsp_one_dig;
    [v_multi]  [buttons]  [v_multi_insert_trnsp_all_t] [v_multi_remove_trnsp_all_t] [v_multi_insert_trnsp_all_dig] [v_multi_remove_trnsp_all_dig] [v_multi_insert_trnsp_one_t] [v_multi_remove_trnsp_one_t] [v_multi_insert_trnsp_one_dig] [v_multi_remove_trnsp_one_dig];
    [f_multi]  [buttons]  [f_multi_insert_trnsp_all_t] [f_multi_remove_trnsp_all_t] [f_multi_insert_trnsp_all_dig] [f_multi_remove_trnsp_all_dig] [f_multi_insert_trnsp_one_t] [f_multi_remove_trnsp_one_t] [f_multi_insert_trnsp_one_dig] [f_multi_remove_trnsp_one_dig];
    [c_multi]  [combos]   [c_multi_insert_trnsp_all_t] [c_multi_remove_trnsp_all_t] [c_multi_insert_trnsp_all_dig] [c_multi_remove_trnsp_all_dig] [c_multi_insert_trnsp_one_t] [c_multi_remove_trnsp_one_t] [c_multi_insert_trnsp_one_dig] [c_multi_remove_trnsp_one_dig];
)]
impl Engine<Edit> {
    pub fn multi_insert_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].trnsp_all_insert_t(trnsp_idx, &mut n);
        }
    }
    pub fn multi_remove_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_remove_t(trnsp_idx);
        }
    }
    pub fn multi_insert_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_insert_dig(trnsp_idx, key_idx_val);
        }
    }
    pub fn multi_remove_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_remove_dig(trnsp_idx, key_idx_val);
        }
    }

    pub fn multi_insert_trnsp_one_t(&mut self, set_idx: usize, del_idx: usize, trnsp_idx: usize) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx <= self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].trnsp_one_insert_t(del_idx, trnsp_idx, &mut n);
        }
    }
    pub fn multi_remove_trnsp_one_t(&mut self, set_idx: usize, del_idx: usize, trnsp_idx: usize) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].trnsp_one_remove_t(del_idx, trnsp_idx);
        }
    }
    pub fn multi_insert_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.multifield[set_idx].trnsp_one_insert_dig(del_idx, trnsp_idx, key_idx_val);
        }
    }
    pub fn multi_remove_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.multifield[set_idx].trnsp_one_remove_dig(del_idx, trnsp_idx, key_idx_val);
        }
    }
}

#[duplicate_item(
    multifield deltafield d_del_vec   del_type_vec       multi_trnsp_all_change_deltas       multi_trnsp_one_change_deltas       trnsp_all_change_deltas     trnsp_one_change_deltas;
    [v_multi]  [buttons]  [i_del_vec] [Vec<Option<i32>>] [v_multi_trnsp_all_change_i_deltas] [v_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [f_multi]  [buttons]  [i_del_vec] [Vec<Option<i32>>] [f_multi_trnsp_all_change_i_deltas] [f_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [c_multi]  [combos]   [i_del_vec] [Vec<Option<i32>>] [c_multi_trnsp_all_change_i_deltas] [c_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [v_multi]  [buttons]  [x_del_vec] [Vec<Option<f64>>] [v_multi_trnsp_all_change_x_deltas] [v_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
    [f_multi]  [buttons]  [x_del_vec] [Vec<Option<f64>>] [f_multi_trnsp_all_change_x_deltas] [f_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
    [c_multi]  [combos]   [x_del_vec] [Vec<Option<f64>>] [c_multi_trnsp_all_change_x_deltas] [c_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
)]
impl Engine<Edit> {
    pub fn multi_trnsp_all_change_deltas(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type_vec,
    ) {
        if set_idx < self.multifield.len()
            && d_del_vec.len() == self.guts.len()
            && trnsp_idx < self.multifield[set_idx].trnsp_all.len()
        {
            self.multifield[set_idx].trnsp_all_change_deltas(trnsp_idx, d_del_vec);
        }
    }
    pub fn multi_trnsp_one_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type_vec,
    ) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
            && d_del_vec.len() == self.guts.len()
        {
            self.multifield[set_idx].trnsp_one_change_deltas(del_idx, trnsp_idx, d_del_vec);
        }
    }
}

#[duplicate_item(
    SetType    deltafield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub fn trnsp_all_insert_t(&mut self, trnsp_idx: usize, n: &mut NewEnginePartParams) {
        self.trnsp_all.insert(trnsp_idx, MulTrnsp::new(n))
    }
    pub fn trnsp_all_remove_t(&mut self, trnsp_idx: usize) {
        self.trnsp_all.remove(trnsp_idx);
    }
    pub fn trnsp_all_insert_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if !self.trnsp_all[trnsp_idx].triggers.contains(&key_idx_val) {
            self.trnsp_all[trnsp_idx].triggers.push(key_idx_val);
        }
    }
    pub fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        self.trnsp_all[trnsp_idx]
            .triggers
            .retain(|&idx| idx != key_idx_val);
    }

    pub fn trnsp_one_insert_t(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        n: &mut NewEnginePartParams,
    ) {
        self.deltafield[del_idx]
            .trnsp_one
            .insert(trnsp_idx, MulTrnsp::new(n))
    }
    pub fn trnsp_one_remove_t(&mut self, del_idx: usize, trnsp_idx: usize) {
        self.deltafield[del_idx].trnsp_one.remove(trnsp_idx);
    }
    pub fn trnsp_one_insert_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if !self.deltafield[del_idx].trnsp_one[trnsp_idx]
            .triggers
            .contains(&key_idx_val)
        {
            self.deltafield[del_idx].trnsp_one[trnsp_idx]
                .triggers
                .push(key_idx_val)
        }
    }
    pub fn trnsp_one_remove_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        self.deltafield[del_idx].trnsp_one[trnsp_idx]
            .triggers
            .retain(|&idx| idx != key_idx_val);
    }
}

#[duplicate_item(
    SetType    deltafield trnsp_all_change_deltas     trnsp_one_change_deltas     d_field   d_del_val   del_type;
    [VFSet]    [buttons]  [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Vec<Option<i32>>];
    [VFSet]    [buttons]  [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Vec<Option<f64>>];
    [ComboSet] [combos]   [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Vec<Option<i32>>];
    [ComboSet] [combos]   [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Vec<Option<f64>>];
)]
impl SetType {
    pub fn trnsp_all_change_deltas(&mut self, trnsp_idx: usize, d_del_vec: del_type) {
        for d in 0..d_del_vec.len() {
            if let Some(i_val) = d_del_vec[d] {
                self.trnsp_all[trnsp_idx].d_field[d] = i_val;
            }
        }
    }

    pub fn trnsp_one_change_deltas(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type,
    ) {
        for d in 0..d_del_vec.len() {
            if let Some(d_val) = d_del_vec[d] {
                self.deltafield[del_idx].trnsp_one[trnsp_idx].d_field[d] = d_val;
            }
        }
    }
}

/**************************** _vfc_sets.rs *********************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************
 ****************************************************************************/

#[duplicate_item(
    SetType    multifield multi_insert_set     multi_remove_set     multi_remove_btn     multi_toggle_radio_mode    ;
    [VFSet]    [v_multi]  [v_multi_insert_set] [v_multi_remove_set] [v_multi_remove_btn] [v_multi_toggle_radio_mode];
    [VFSet]    [f_multi]  [f_multi_insert_set] [f_multi_remove_set] [f_multi_remove_btn] [f_multi_toggle_radio_mode];
    [ComboSet] [c_multi]  [c_multi_insert_set] [c_multi_remove_set] [c_multi_remove_btn] [c_multi_toggle_radio_mode];
)]
impl Engine<Edit> {
    pub fn multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.multifield.len() {
            self.multifield
                .insert(set_idx, SetType::new(&mut NewEnginePartParams::new(&self)));
        }
    }
    pub fn multi_remove_set(&mut self, set_idx: usize) {
        if set_idx < self.multifield.len() && self.multifield.len() > 0 {
            self.multifield.remove(set_idx);
        }
    }
    pub fn multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
            && self.multifield[set_idx].buttons.len() > 1
        {
            self.multifield[set_idx].remove_btn(btn_idx)
        }
    }
    pub fn multi_toggle_radio_mode(&mut self, set_idx: usize) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].radio_mode = !self.multifield[set_idx].radio_mode;
        }
    }
}

#[duplicate_item(
    SetType    multifield multi_change_minmax_pressed   minmaxval minmax_field  iscomparedto                             multi_insertremove_btn_dig btn_insertremove_dig;
    [VFSet]    [v_multi]  [v_multi_change_min_pressed]  [min_val] [min_pressed] [le(&self.v_multi[set_idx].max_pressed)] [v_multi_insert_btn_dig]   [btn_insert_dig]    ;
    [VFSet]    [f_multi]  [f_multi_change_min_pressed]  [min_val] [min_pressed] [le(&self.f_multi[set_idx].max_pressed)] [f_multi_insert_btn_dig]   [btn_insert_dig]    ;
    [ComboSet] [c_multi]  [c_multi_change_min_pressed]  [min_val] [min_pressed] [le(&self.c_multi[set_idx].max_pressed)] [c_multi_insert_btn_dig]   [btn_insert_dig]    ;
    [VFSet]    [v_multi]  [v_multi_change_max_pressed]  [max_val] [max_pressed] [ge(&self.v_multi[set_idx].min_pressed)] [v_multi_remove_btn_dig]   [btn_remove_dig]    ;
    [VFSet]    [f_multi]  [f_multi_change_max_pressed]  [max_val] [max_pressed] [ge(&self.f_multi[set_idx].min_pressed)] [f_multi_remove_btn_dig]   [btn_remove_dig]    ;
    [ComboSet] [c_multi]  [c_multi_change_max_pressed]  [max_val] [max_pressed] [ge(&self.c_multi[set_idx].min_pressed)] [c_multi_remove_btn_dig]   [btn_remove_dig]    ;
)]
impl Engine<Edit> {
    pub fn multi_change_minmax_pressed(&mut self, set_idx: usize, minmaxval: usize) {
        if set_idx < self.multifield.len() && minmaxval.iscomparedto {
            self.multifield[set_idx].minmax_field = minmaxval
        }
    }
    pub fn multi_insertremove_btn_dig(
        &mut self,
        set_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.multifield[set_idx].btn_insertremove_dig(btn_idx, key_idx_val)
        }
    }
}

#[duplicate_item(
    multifield vf_multi_insert_btn ;
    [v_multi]  [v_multi_insert_btn];
    [f_multi]  [f_multi_insert_btn];
)]
impl Engine<Edit> {
    pub fn vf_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        if set_idx < self.multifield.len() && btn_idx < self.multifield[set_idx].buttons.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx]
                .buttons
                .insert(btn_idx, VFBtn::new(&mut n));
        }
    }
}

impl Engine<Edit> {
    pub fn c_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        if set_idx < self.c_multi.len() && btn_idx <= self.c_multi[set_idx].buttons.len() {
            self.c_multi[set_idx].insert_btn(btn_idx);
        }
    }
    pub fn c_multi_insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx < self.c_multi.len() && c_idx <= self.c_multi[set_idx].combos.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            n.c_btn_len = self.c_multi[set_idx].buttons.len();
            self.c_multi[set_idx].insert_combo(c_idx, &mut n);
        }
    }
    pub fn c_multi_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx < self.c_multi.len() && c_idx < self.c_multi[set_idx].combos.len() {
            self.c_multi[set_idx].remove_combo(c_idx);
        }
    }
}

#[duplicate_item(
    multifield deltafield multi_change_deltas       d_field   d_del_vec   del_type_vec;
    [v_multi]  [buttons]  [v_multi_change_i_deltas] [i_delta] [i_del_vec] [Vec<Option<i32>>];
    [f_multi]  [buttons]  [f_multi_change_i_deltas] [i_delta] [i_del_vec] [Vec<Option<i32>>];
    [c_multi]  [combos]   [c_multi_change_i_deltas] [i_delta] [i_del_vec] [Vec<Option<i32>>];
    [v_multi]  [buttons]  [v_multi_change_x_deltas] [x_delta] [x_del_vec] [Vec<Option<f64>>];
    [f_multi]  [buttons]  [f_multi_change_x_deltas] [x_delta] [x_del_vec] [Vec<Option<f64>>];
    [c_multi]  [combos]   [c_multi_change_x_deltas] [x_delta] [x_del_vec] [Vec<Option<f64>>];
)]
impl Engine<Edit> {
    pub fn multi_change_deltas(&mut self, set_idx: usize, del_idx: usize, d_del_vec: del_type_vec) {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && d_del_vec.len() == self.guts.len()
        {
            for (d, &d_del) in d_del_vec.iter().enumerate() {
                if let Some(d_val) = d_del {
                    if set_idx < self.multifield.len() {
                        self.multifield[set_idx].deltafield[del_idx].d_field[d] = d_val;
                    }
                }
            }
        }
    }
}

impl VFSet {
    pub fn insert_btn(&mut self, btn_idx: usize, n: &mut NewEnginePartParams) {
        self.buttons.insert(btn_idx, VFBtn::new(n));
        self.pressed.insert(btn_idx, false);
    }
    pub fn remove_btn(&mut self, btn_idx: usize) {
        self.buttons.remove(btn_idx);
        self.pressed.remove(btn_idx);
    }
}

impl ComboSet {
    pub fn insert_btn(&mut self, btn_idx: usize) {
        self.buttons.insert(btn_idx, ComboTog::default());
        self.pressed.insert(btn_idx, false);
        for c in 0..self.combos.len() {
            self.combos[c].combo.insert(btn_idx, false);
        }
    }
    pub fn remove_btn(&mut self, btn_idx: usize) {
        self.buttons.remove(btn_idx);
        self.pressed.remove(btn_idx);
        for c in 0..self.combos.len() {
            self.combos[c].combo.remove(btn_idx);
        }
    }
    pub fn insert_combo(&mut self, c_idx: usize, n: &mut NewEnginePartParams) {
        self.combos.insert(c_idx, Combo::new(n));
    }
    pub fn remove_combo(&mut self, c_idx: usize) {
        self.combos.remove(c_idx);
    }
}

#[duplicate_item(
    SetType     btn_dig          operation                          ;
    [VFSet]     [btn_insert_dig] [push(key_idx_val)]                ;
    [ComboSet]  [btn_insert_dig] [push(key_idx_val)]                ;
    [VFSet]     [btn_remove_dig] [retain(|&idx| idx != key_idx_val)];
    [ComboSet]  [btn_remove_dig] [retain(|&idx| idx != key_idx_val)];
)]
impl SetType {
    pub fn btn_dig(&mut self, btn_idx: usize, key_idx_val: usize) {
        self.buttons[btn_idx].togs.operation;
        self.buttons[btn_idx].togs.dedup();
    }
}
