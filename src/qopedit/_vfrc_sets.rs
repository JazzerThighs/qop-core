use crate::{*, qopedit::NewTrait};
use duplicate::duplicate_item;

#[duplicate_item(
    multi_insert_set        multi_remove_set        SetType     multifield  radionum;
    [v_multi_insert_set]    [v_multi_remove_set]    [VFRSet]    [v_multi]   [0];
    [f_multi_insert_set]    [f_multi_remove_set]    [VFRSet]    [f_multi]   [0];
    [r_multi_insert_set]    [r_multi_remove_set]    [VFRSet]    [r_multi]   [1];
    [c_multi_insert_set]    [c_multi_remove_set]    [ComboSet]  [c_multi]   [0];
)]
impl Qop<Edit> {
    pub fn multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.multifield.len() {
            self.n.radio_num = radionum;
            self.multifield.insert(set_idx, SetType::new(&mut self.n));
        }
    }
    pub fn multi_remove_set(&mut self, set_idx: usize) {
        if self.multifield.len() > 0 && self.multifield.len() > set_idx {
            self.multifield.remove(set_idx);
        }
    }
}

#[duplicate_item(
    multifield  vfr_multi_insert_btn;
    [v_multi]   [v_multi_insert_btn];
    [f_multi]   [f_multi_insert_btn];
    [r_multi]   [r_multi_insert_btn];
)]
impl Qop<Edit> {  
    pub fn vfr_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.multifield[set_idx].insert_btn(btn_idx, &mut self.n);
    }
}

impl Qop<Edit> {
    pub fn c_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.c_multi[set_idx].insert_btn(btn_idx);
    }
    pub fn c_multi_insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.n.guts_len = self.guts.len();
            self.n.c_btn_len = self.c_multi[set_idx].buttons.len();
            self.c_multi[set_idx].insert_combo(c_idx, &mut self.n);
        }
    }
    pub fn c_multi_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx <= self.c_multi.len() {
            self.c_multi[set_idx].remove_combo(c_idx);
        }
    }
}

#[duplicate_item(
    multifield  onefield    minmaxpressed   vfr_multi_change_minmax_pressed vfr_one_change_minmax_pressed;
    [v_multi]   [v_one]     [min_pressed]   [v_multi_change_min_pressed]    [v_one_change_min_pressed];
    [f_multi]   [f_one]     [min_pressed]   [f_multi_change_min_pressed]    [f_one_change_min_pressed];
    [r_multi]   [r_one]     [min_pressed]   [r_multi_change_min_pressed]    [r_one_change_min_pressed];
    [v_multi]   [v_one]     [max_pressed]   [v_multi_change_max_pressed]    [v_one_change_max_pressed];
    [f_multi]   [f_one]     [max_pressed]   [f_multi_change_max_pressed]    [f_one_change_max_pressed];
    [r_multi]   [r_one]     [max_pressed]   [r_multi_change_max_pressed]    [r_one_change_max_pressed];
)]
impl Qop<Edit> {  
    pub fn vfr_multi_change_minmax_pressed(&mut self, set_idx: usize, max_val: usize) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].minmaxpressed = max_val
        }
    }
    pub fn vfr_one_change_minmax_pressed(&mut self, g_idx: usize, set_idx: usize, max_val: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len() 
        {
            self.guts[g_idx].onefield[set_idx].minmaxpressed = max_val;
        }
    }
}

#[duplicate_item(
    setfield    deltafield  multi_remove_btn        multi_insert_btn_dig        multi_remove_btn_dig        multi_change_deltas;
    [v_multi]   [buttons]   [v_multi_remove_btn]    [v_multi_insert_btn_dig]    [v_multi_remove_btn_dig]    [v_multi_change_deltas];
    [f_multi]   [buttons]   [f_multi_remove_btn]    [f_multi_insert_btn_dig]    [f_multi_remove_btn_dig]    [f_multi_change_deltas];
    [r_multi]   [buttons]   [r_multi_remove_btn]    [r_multi_insert_btn_dig]    [r_multi_remove_btn_dig]    [r_multi_change_deltas];
    [c_multi]   [combos]    [c_multi_remove_btn]    [c_multi_insert_btn_dig]    [c_multi_remove_btn_dig]    [c_multi_change_deltas];
)]
impl Qop<Edit> {
    pub fn multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.v_multi[set_idx].remove_btn(btn_idx)
    }
    pub fn multi_insert_btn_dig(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len() && set_idx < self.setfield.len() {
            self.setfield[set_idx].btn_insert_dig(btn_idx, key_idx_val)
        }
    }
    pub fn multi_remove_btn_dig(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len() && set_idx < self.setfield.len() {
            self.setfield[set_idx].btn_remove_dig(btn_idx, key_idx_val)
        }
    }
    pub fn multi_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        if del_idx < self.guts.len() {
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if let Some(delta) = i_del {
                    if set_idx < self.setfield.len() {
                        self.setfield[set_idx].deltafield[del_idx].i_delta[i] = delta
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if let Some(delta) = x_del {
                    if set_idx < self.setfield.len() {
                        self.setfield[set_idx].deltafield[del_idx].x_delta[x] = delta
                    }
                }
            }
        }
    }
}

impl<T, U> VFRSet<T, U>
where
    VFRBtn<T, U>: NewTrait,
{
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, n: &mut NewStuffPointers) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, VFRBtn::new(n));
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
}

impl<T, U> ComboSet<T, U>
where
    Combo<T, U>: NewTrait,
{
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
    pub(crate) fn insert_combo(&mut self, c_idx: usize, n: &mut NewStuffPointers) {
        n.c_btn_len = self.buttons.len();
        if c_idx <= self.combos.len() {
            self.combos.insert(c_idx, Combo::new(n));
        }
    }
    pub(crate) fn remove_combo(&mut self, c_idx: usize) {
        if self.combos.len() > 0 && c_idx < self.combos.len() {
            self.combos.remove(c_idx);
        }
    }
}

#[duplicate_item(
    SetType;
    [VFRSet];
    [ComboSet];
)]
impl<T, U> SetType<T, U> {
    pub(crate) fn btn_insert_dig(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
                self.buttons[btn_idx].togs.push(key_idx_val);
            }
        }
    }
    pub(crate) fn btn_remove_dig(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
        }
    }
}
