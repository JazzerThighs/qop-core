use crate::*;

impl Qop<Edit> {
    pub fn set_trnsp_all_params(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.v_multi.len() {
                    self.v_multi[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
            SetType::FretSet => {
                if set_idx <= self.f_multi.len() {
                    self.f_multi[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.r_multi.len() {
                    self.r_multi[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.c_multi.len() {
                    self.c_multi[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
        }
    }
    pub fn set_remove_trnsp_all_key(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.v_multi.len() {
                    self.v_multi[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            SetType::FretSet => {
                if set_idx <= self.f_multi.len() {
                    self.f_multi[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.r_multi.len() {
                    self.r_multi[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.c_multi.len() {
                    self.c_multi[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
        }
    }
    pub fn set_remove_trnsp_all(&mut self, set_kind: SetType, set_idx: usize, trnsp_idx: usize) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.v_multi.len() {
                    self.v_multi[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            SetType::FretSet => {
                if set_idx <= self.f_multi.len() {
                    self.f_multi[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.r_multi.len() {
                    self.r_multi[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.c_multi.len() {
                    self.c_multi[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
        }
    }

    /* ********************************************************************* */

    pub fn set_trnsp_one_params(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        btn_idx: usize,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.v_multi.len() {
                    self.v_multi[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
            SetType::FretSet => {
                if set_idx <= self.f_multi.len() {
                    self.f_multi[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.r_multi.len() {
                    self.r_multi[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.c_multi.len() {
                    self.c_multi[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.guts.len(),
                    );
                }
            }
        }
    }
    pub fn set_remove_trnsp_one_key(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        trnsp_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.v_multi.len() {
                    self.v_multi[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            SetType::FretSet => {
                if set_idx <= self.f_multi.len() {
                    self.f_multi[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.r_multi.len() {
                    self.r_multi[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.c_multi.len() {
                    self.c_multi[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
        }
    }
    pub fn set_remove_trnsp_one(
        &mut self,
        set_kind: SetType,
        set_idx: usize,
        btn_idx: usize,
        trnsp_idx: usize,
    ) {
        match set_kind {
            SetType::ValveSet => {
                if set_idx <= self.v_multi.len() {
                    self.v_multi[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            SetType::FretSet => {
                if set_idx <= self.f_multi.len() {
                    self.f_multi[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            SetType::RadioSet => {
                if set_idx <= self.r_multi.len() {
                    self.r_multi[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            SetType::ComboSet => {
                if set_idx <= self.c_multi.len() {
                    self.c_multi[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
        }
    }
}
