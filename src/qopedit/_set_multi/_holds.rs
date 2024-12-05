use crate::{*, qopedit::NewTrait};

impl<T, U> VFRSet<T, U> {
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
    pub(crate) fn sustain_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.sustain.togs.contains(&key_idx_val) {
            self.holds.sustain.togs.push(key_idx_val)
        }
    }
    pub(crate) fn inv_sustain_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
            self.holds.inv_sustain.togs.push(key_idx_val)
        }
    }
    pub(crate) fn sostenuto_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.sostenuto.togs.contains(&key_idx_val) {
            self.holds.sostenuto.togs.push(key_idx_val)
        }
    }
    pub(crate) fn inv_sostenuto_insert_dig(&mut self, key_idx_val: usize) {
        if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
            self.holds.inv_sostenuto.togs.push(key_idx_val)
        }
    }
    pub(crate) fn sustain_remove_dig(&mut self, key_idx_val: usize) {
        self.holds.sustain.togs.retain(|&idx| idx != key_idx_val)
    }
    pub(crate) fn inv_sustain_remove_dig(&mut self, key_idx_val: usize) {
        self.holds
            .inv_sustain
            .togs
            .retain(|&idx| idx != key_idx_val)
    }
    pub(crate) fn sostenuto_remove_dig(&mut self, key_idx_val: usize) {
        self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val)
    }
    pub(crate) fn inv_sostenuto_remove_dig(&mut self, key_idx_val: usize) {
        self.holds
            .inv_sostenuto
            .togs
            .retain(|&idx| idx != key_idx_val)
    }
}