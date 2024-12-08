use crate::{qopedit::NewTrait, *};
use duplicate::duplicate_item;

#[duplicate_item(
    SetType    multifield onefield multi_insert_set     multi_remove_set     multi_remove_btn     multi_insert_btn_dig     multi_remove_btn_dig     multi_toggle_radio_mode     multi_change_min_pressed     multi_change_max_pressed     one_insert_set     one_remove_set     one_remove_btn     one_insert_btn_dig     one_remove_btn_dig     one_toggle_radio_mode     one_change_min_pressed     one_change_max_pressed;
    [VFSet]    [v_multi]  [v_one]  [v_multi_insert_set] [v_multi_remove_set] [v_multi_remove_btn] [v_multi_insert_btn_dig] [v_multi_remove_btn_dig] [v_multi_toggle_radio_mode] [v_multi_change_min_pressed] [v_multi_change_max_pressed] [v_one_insert_set] [v_one_remove_set] [v_one_remove_btn] [v_one_insert_btn_dig] [v_one_remove_btn_dig] [v_one_toggle_radio_mode] [v_one_change_min_pressed] [v_one_change_max_pressed];        
    [VFSet]    [f_multi]  [f_one]  [f_multi_insert_set] [f_multi_remove_set] [f_multi_remove_btn] [f_multi_insert_btn_dig] [f_multi_remove_btn_dig] [f_multi_toggle_radio_mode] [f_multi_change_min_pressed] [f_multi_change_max_pressed] [f_one_insert_set] [f_one_remove_set] [f_one_remove_btn] [f_one_insert_btn_dig] [f_one_remove_btn_dig] [f_one_toggle_radio_mode] [f_one_change_min_pressed] [f_one_change_max_pressed];        
    [ComboSet] [c_multi]  [c_one]  [c_multi_insert_set] [c_multi_remove_set] [c_multi_remove_btn] [c_multi_insert_btn_dig] [c_multi_remove_btn_dig] [c_multi_toggle_radio_mode] [c_multi_change_min_pressed] [c_multi_change_max_pressed] [c_one_insert_set] [c_one_remove_set] [c_one_remove_btn] [c_one_insert_btn_dig] [c_one_remove_btn_dig] [c_one_toggle_radio_mode] [c_one_change_min_pressed] [c_one_change_max_pressed];        
)]
impl Qop<Edit> {
    pub fn multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.multifield.len() {
            self.multifield.insert(set_idx, SetType::new(&mut self.n));
        }
    }
    pub fn multi_remove_set(&mut self, set_idx: usize) {
        if set_idx < self.multifield.len()
            && self.multifield.len() > 0 
        {
            self.multifield.remove(set_idx);
        }
    }
    pub fn multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
        {
            self.multifield[set_idx].remove_btn(btn_idx)
        }
    }
    pub fn multi_insert_btn_dig(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.multifield[set_idx].btn_insert_dig(btn_idx, key_idx_val)
        }
    }
    pub fn multi_remove_btn_dig(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.multifield[set_idx].btn_remove_dig(btn_idx, key_idx_val)
        }
    }
    pub fn multi_toggle_radio_mode(&mut self, set_idx: usize) {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].radio_mode = !self.multifield[set_idx].radio_mode;
        }
    }
    pub fn multi_change_min_pressed(&mut self, set_idx: usize, min_val: usize) {
        if set_idx < self.multifield.len()
            && min_val <= self.multifield[set_idx].max_pressed
        {
            self.multifield[set_idx].min_pressed = min_val
        }
    }
    pub fn multi_change_max_pressed(&mut self, set_idx: usize, max_val: usize) {
        if set_idx < self.multifield.len()
            && max_val >= self.multifield[set_idx].min_pressed
        {
            self.multifield[set_idx].max_pressed = max_val
        }
    }
    
    pub fn one_insert_set(&mut self, g_idx: usize, set_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
        {
            self.guts[g_idx].onefield.insert(set_idx, SetType::new(&mut self.n));
        }
    }
    pub fn one_remove_set(&mut self, g_idx: usize, set_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
        {
            self.guts[g_idx].onefield.remove(set_idx);
        }
    }
    pub fn one_remove_btn(&mut self, g_idx: usize, set_idx: usize, btn_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && btn_idx < self.guts[g_idx].onefield[set_idx].buttons.len()
        {
            self.guts[g_idx].onefield[set_idx].remove_btn(btn_idx);
        }
    }
    pub fn one_insert_btn_dig(&mut self, g_idx: usize, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && btn_idx < self.guts[g_idx].onefield[set_idx].buttons.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.guts[g_idx].onefield[set_idx].btn_insert_dig(btn_idx, key_idx_val)
        }
    }
    pub fn one_remove_btn_dig(&mut self, g_idx: usize, set_idx: usize, btn_idx: usize, key_idx_val: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && btn_idx < self.guts[g_idx].onefield[set_idx].buttons.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.guts[g_idx].onefield[set_idx].btn_remove_dig(btn_idx, key_idx_val)
        }
    }
    pub fn one_toggle_radio_mode(&mut self, g_idx: usize, set_idx: usize) {
        if g_idx < self.guts.len() 
            && set_idx < self.guts[g_idx].onefield.len()
        {
            self.guts[g_idx].onefield[set_idx].radio_mode =
                !self.guts[g_idx].onefield[set_idx].radio_mode;
        }
    }
    pub fn one_change_min_pressed(&mut self, g_idx: usize, set_idx: usize, min_val: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && min_val <= self.guts[g_idx].onefield[set_idx].max_pressed
        {
            self.guts[g_idx].onefield[set_idx].min_pressed = min_val;
        }
    }
    pub fn one_change_max_pressed(&mut self, g_idx: usize, set_idx: usize, max_val: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && max_val >= self.guts[g_idx].onefield[set_idx].min_pressed
        {
            self.guts[g_idx].onefield[set_idx].max_pressed = max_val;
        }
    }
}

#[duplicate_item(
    multifield onefield vf_multi_insert_btn  vf_one_insert_btn;
    [v_multi]  [v_one]  [v_multi_insert_btn] [v_one_insert_btn];
    [f_multi]  [f_one]  [f_multi_insert_btn] [f_one_insert_btn];
)]
impl Qop<Edit> {
    pub fn vf_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        if set_idx < self.multifield.len() && btn_idx < self.multifield[set_idx].buttons.len() {
            self.multifield[set_idx].buttons.insert(btn_idx, VFBtn::new(&mut self.n));
        }
    }
    pub fn vf_one_insert_btn(&mut self, g_idx: usize, set_idx: usize, btn_idx: usize) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && btn_idx < self.guts[g_idx].onefield[set_idx].buttons.len()
        {
            self.guts[g_idx].onefield[set_idx].buttons.insert(btn_idx, VFBtn::new(&mut self.n));
        }
    }
}
impl Qop<Edit> {
    pub fn c_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        if set_idx < self.c_multi.len()
            && btn_idx <= self.c_multi[set_idx].buttons.len()
        {
            self.c_multi[set_idx].insert_btn(btn_idx);
        }
    }
    pub fn c_multi_insert_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx < self.c_multi.len()
            && c_idx <= self.c_multi[set_idx].combos.len() 
        {
            self.n.guts_len = self.guts.len();
            self.n.c_btn_len = self.c_multi[set_idx].buttons.len();
            self.c_multi[set_idx].insert_combo(c_idx, &mut self.n);
        }
    }
    pub fn c_multi_remove_combo(&mut self, set_idx: usize, c_idx: usize) {
        if set_idx < self.c_multi.len()
            && c_idx < self.c_multi[set_idx].combos.len()
        {
            self.c_multi[set_idx].remove_combo(c_idx);
        }
    }
    pub fn c_one_insert_btn(&mut self, g_idx: usize, set_idx: usize, btn_idx: usize) {
        if g_idx < self.guts.len() 
            && set_idx < self.guts[g_idx].c_one.len() 
            && btn_idx <= self.guts[g_idx].c_one[set_idx].buttons.len()
        {
            self.guts[g_idx].c_one[set_idx].insert_btn(btn_idx);
        }
    }
    pub fn c_one_insert_combo(&mut self, g_idx: usize, set_idx: usize, c_idx: usize) {
        if set_idx < self.c_multi.len()
            && c_idx <= self.c_multi[set_idx].combos.len()
        {
            self.n.guts_len = self.guts.len();
            self.n.c_btn_len = self.c_multi[set_idx].buttons.len();
            self.guts[g_idx].c_one[set_idx].insert_combo(c_idx, &mut self.n);
        }
    }
    pub fn c_one_remove_combo(&mut self, g_idx: usize, set_idx: usize, c_idx: usize) {
        if set_idx < self.c_multi.len()
            && c_idx < self.c_multi[set_idx].combos.len() {
            self.guts[g_idx].c_one[set_idx].remove_combo(c_idx);
        }
    }
}

#[duplicate_item(
    multifield onefield deltafield multi_change_deltas       one_change_deltas       d_field   d_del_val   del_type      d_del_vec   del_type_vec;
    [v_multi]  [v_one]  [buttons]  [v_multi_change_i_deltas] [v_one_change_i_deltas] [i_delta] [i_del_val] [Option<i64>] [i_del_vec] [Vec<Option<i64>>];
    [f_multi]  [f_one]  [buttons]  [f_multi_change_i_deltas] [f_one_change_i_deltas] [i_delta] [i_del_val] [Option<i64>] [i_del_vec] [Vec<Option<i64>>];
    [c_multi]  [c_one]  [combos]   [c_multi_change_i_deltas] [c_one_change_i_deltas] [i_delta] [i_del_val] [Option<i64>] [i_del_vec] [Vec<Option<i64>>];
    [v_multi]  [v_one]  [buttons]  [v_multi_change_x_deltas] [v_one_change_x_deltas] [x_delta] [x_del_val] [Option<f64>] [x_del_vec] [Vec<Option<f64>>];
    [f_multi]  [f_one]  [buttons]  [f_multi_change_x_deltas] [f_one_change_x_deltas] [x_delta] [x_del_val] [Option<f64>] [x_del_vec] [Vec<Option<f64>>];
    [c_multi]  [c_one]  [combos]   [c_multi_change_x_deltas] [c_one_change_x_deltas] [x_delta] [x_del_val] [Option<f64>] [x_del_vec] [Vec<Option<f64>>];
)]
impl Qop<Edit> {
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
    pub fn one_change_deltas(
        &mut self,
        g_idx: usize,
        set_idx: usize,
        del_idx: usize,
        d_del_val: del_type,
    ) {
        if g_idx < self.guts.len()
            && set_idx < self.guts[g_idx].onefield.len()
            && del_idx < self.guts[g_idx].onefield[set_idx].deltafield.len()
        {
            if let Some(d_val) = d_del_val {
                if set_idx < self.multifield.len() {
                    self.guts[g_idx].onefield[set_idx].deltafield[del_idx].d_field = d_val;
                }
            }
        }
    }
}

impl<T, U> VFSet<T, U>
where
    VFBtn<T, U>: NewTrait,
{
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, n: &mut NewStuffPointers) {
        self.buttons.insert(btn_idx, VFBtn::new(n));
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        self.buttons.remove(btn_idx);
    }
}

impl<T, U> ComboSet<T, U>
where
    Combo<T, U>: NewTrait,
{
    pub(crate) fn insert_btn(&mut self, btn_idx: usize) {
        self.buttons.insert(btn_idx, BtnTog::default());
        for c in 0..self.combos.len() {
            self.combos[c].combo.insert(btn_idx, false);
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        self.buttons.remove(btn_idx);
        for c in 0..self.combos.len() {
            self.combos[c].combo.remove(btn_idx);
        }
    }
    pub(crate) fn insert_combo(&mut self, c_idx: usize, n: &mut NewStuffPointers) {
        self.combos.insert(c_idx, Combo::new(n));
    }
    pub(crate) fn remove_combo(&mut self, c_idx: usize) {
        self.combos.remove(c_idx);
    }
}

#[duplicate_item(
    SetType;
    [VFSet];
    [ComboSet];
)]
impl<T, U> SetType<T, U> {
    pub(crate) fn btn_insert_dig(&mut self, btn_idx: usize, key_idx_val: usize) {
        if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
            self.buttons[btn_idx].togs.push(key_idx_val);
        }
    }
    pub(crate) fn btn_remove_dig(&mut self, btn_idx: usize, key_idx_val: usize) {
        self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
    }
}
