use crate::{*, qopedit::NewTrait};
use duplicate::duplicate_item;

#[duplicate_item(
    multi_insert_set        multi_remove_set        SetType     multifield;
    [v_multi_insert_set]    [v_multi_remove_set]    [VFRSet]    [v_multi];
    [f_multi_insert_set]    [f_multi_remove_set]    [VFRSet]    [f_multi];
    [r_multi_insert_set]    [r_multi_remove_set]    [VFRSet]    [r_multi];
    [c_multi_insert_set]    [c_multi_remove_set]    [ComboSet]  [c_multi];
)]
impl Qop<Edit> {
    pub fn multi_insert_set(&mut self, set_idx: usize) {
        if set_idx <= self.multifield.len() {
            self.multifield.insert(set_idx, SetType::new(&mut self.n));
        }
    }
    pub fn multi_remove_set(&mut self, set_idx: usize) {
        if self.multifield.len() > 0 && self.multifield.len() > set_idx {
            self.multifield.remove(set_idx);
        }
    }
}

impl Qop<Edit> {
    #[duplicate_item(
        vfr_multi_insert_btn    setfield;
        [v_multi_insert_btn]    [v_multi];
        [f_multi_insert_btn]    [f_multi];
        [r_multi_insert_btn]    [r_multi];
    )]
    pub fn vfr_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) {
        self.setfield[set_idx].insert_btn(btn_idx, &mut self.n);
    }
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
    VFRIndv<T, U>: NewTrait,
{
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, n: &mut NewStuffPointers) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, VFRIndv::new(n));
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
