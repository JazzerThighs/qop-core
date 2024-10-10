use super::QopEdit;

impl QopEdit {
    pub fn set_trnsp_all_params(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets[set_idx].trnsp_all_params(
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            _ => return,
        }
    }
    pub fn set_remove_trnsp_all_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets[set_idx].trnsp_all_remove_key(trnsp_idx, key_idx_val);
                }
            }
            _ => return,
        }
    }
    pub fn set_remove_trnsp_all(&mut self, set_kind: u8, set_idx: usize, trnsp_idx: usize) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets[set_idx].trnsp_all_remove(trnsp_idx);
                }
            }
            _ => return,
        }
    }

    /* ********************************************************************* */

    pub fn set_trnsp_one_params(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
    ) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets[set_idx].trnsp_one_params(
                        btn_idx,
                        trnsp_idx,
                        key_idx_vals,
                        i_del_vec,
                        x_del_vec,
                        self.plucks.len(),
                    );
                }
            }
            _ => return,
        }
    }
    pub fn set_remove_trnsp_one_key(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        trnsp_idx: usize,
        btn_idx: usize,
        key_idx_val: usize,
    ) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets[set_idx].trnsp_one_remove_key(btn_idx, trnsp_idx, key_idx_val);
                }
            }
            _ => return,
        }
    }
    pub fn set_remove_trnsp_one(
        &mut self,
        set_kind: u8,
        set_idx: usize,
        btn_idx: usize,
        trnsp_idx: usize,
    ) {
        match set_kind {
            0 => {
                if set_idx <= self.valve_sets.len() {
                    self.valve_sets[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            1 => {
                if set_idx <= self.fret_sets.len() {
                    self.fret_sets[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            2 => {
                if set_idx <= self.radio_sets.len() {
                    self.radio_sets[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            3 => {
                if set_idx <= self.aero_sets.len() {
                    self.aero_sets[set_idx].trnsp_one_remove(btn_idx, trnsp_idx);
                }
            }
            _ => return,
        }
    }
}