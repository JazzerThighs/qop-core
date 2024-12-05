use qopedit::NewTrait;
use crate::*;

impl<T, U> VFRSet<T, U> {
    pub(crate) fn trnsp_all_params(
        &mut self,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        guts: usize,
    ) {
        if trnsp_idx == self.trnsp_all.len() {
            let mut tp: Trnsp<T, U> = Trnsp::new(guts);
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    tp.triggers.push(k);
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < guts {
                    if let Some(delta) = i_del {
                        tp.i_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        tp.x_delta[x] = delta;
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
                if i < guts {
                    if let Some(delta) = i_del {
                        self.trnsp_all[trnsp_idx].i_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        self.trnsp_all[trnsp_idx].x_delta[x] = delta;
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
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
        guts: usize,
    ) {
        if btn_idx < self.buttons.len() {
            if trnsp_idx == self.buttons[btn_idx].trnsp_one.len() {
                let mut tp: Trnsp<T, U> = Trnsp::new(guts);
                for (_i, &key) in key_idx_vals.iter().enumerate() {
                    if let Some(k) = key {
                        tp.triggers.push(k);
                    }
                }
                for (i, &i_del) in i_del_vec.iter().enumerate() {
                    if i < guts {
                        if let Some(delta) = i_del {
                            tp.i_delta[i] = delta;
                        }
                    }
                }
                for (x, &x_del) in x_del_vec.iter().enumerate() {
                    if x < guts {
                        if let Some(delta) = x_del {
                            tp.x_delta[x] = delta;
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
                    if i < guts {
                        if let Some(delta) = i_del {
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].i_delta[i] = delta;
                        }
                    }
                }
                for (x, &x_del) in x_del_vec.iter().enumerate() {
                    if x < guts {
                        if let Some(delta) = x_del {
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].x_delta[x] = delta;
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
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



impl<T, U> ComboSet<T, U> {
    pub(crate) fn trnsp_all_params(
        &mut self,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        guts: usize,
    ) {
        if trnsp_idx == self.trnsp_all.len() {
            let mut tp: Trnsp = Trnsp::new(guts);
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    tp.triggers.push(k);
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < guts {
                    if let Some(delta) = i_del {
                        tp.i_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        tp.x_delta[x] = delta;
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
                if i < guts {
                    if let Some(delta) = i_del {
                        self.trnsp_all[trnsp_idx].i_delta[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        self.trnsp_all[trnsp_idx].x_delta[x] = delta;
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
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
        guts: usize,
    ) {
        if c_idx < self.combos.len() {
            if c_idx < self.buttons.len() {
                if trnsp_idx == self.combos[c_idx].trnsp_one.len() {
                    let mut tp: Trnsp = Trnsp::new(guts);
                    for (_i, &key) in key_idx_vals.iter().enumerate() {
                        if let Some(k) = key {
                            tp.triggers.push(k);
                        }
                    }
                    for (i, &i_del) in i_del_vec.iter().enumerate() {
                        if i < guts {
                            if let Some(delta) = i_del {
                                tp.i_delta[i] = delta;
                            }
                        }
                    }
                    for (x, &x_del) in x_del_vec.iter().enumerate() {
                        if x < guts {
                            if let Some(delta) = x_del {
                                tp.x_delta[x] = delta;
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
                        if i < guts {
                            if let Some(delta) = i_del {
                                self.combos[c_idx].trnsp_one[trnsp_idx].i_delta[i] = delta;
                            }
                        }
                    }
                    for (x, &x_del) in x_del_vec.iter().enumerate() {
                        if x < guts {
                            if let Some(delta) = x_del {
                                self.combos[c_idx].trnsp_one[trnsp_idx].x_delta[x] = delta;
                            }
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
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
