use crate::*;

impl Qop<Edit> {
    pub fn gut_sustain_insert_dig(&mut self, key_idx_val: usize) {
    if !self.gut_holds.sustain.togs.contains(&key_idx_val) {
        self.gut_holds.sustain.togs.push(key_idx_val)
    }
}
pub fn gut_inv_sustain_insert_dig(&mut self, key_idx_val: usize) {
    if !self.gut_holds.inv_sustain.togs.contains(&key_idx_val) {
        self.gut_holds.inv_sustain.togs.push(key_idx_val)
    }
}
pub fn gut_sostenuto_insert_dig(&mut self, key_idx_val: usize) {
    if !self.gut_holds.sostenuto.togs.contains(&key_idx_val) {
        self.gut_holds.sostenuto.togs.push(key_idx_val)
    }
}
pub fn gut_inv_sostenuto_insert_dig(&mut self, key_idx_val: usize) {
    if !self.gut_holds.inv_sostenuto.togs.contains(&key_idx_val) {
        self.gut_holds.inv_sostenuto.togs.push(key_idx_val)
    }
}
pub fn gut_sustain_remove_dig(&mut self, key_idx_val: usize) {
    self.gut_holds
        .sustain
        .togs
        .retain(|&idx| idx != key_idx_val)
}
pub fn gut_inv_sustain_remove_dig(&mut self, key_idx_val: usize) {
    self.gut_holds
        .inv_sustain
        .togs
        .retain(|&idx| idx != key_idx_val)
}
pub fn gut_sostenuto_remove_dig(&mut self, key_idx_val: usize) {
    self.gut_holds
        .sostenuto
        .togs
        .retain(|&idx| idx != key_idx_val)
}
pub fn gut_inv_sostenuto_remove_dig(&mut self, key_idx_val: usize) {
    self.gut_holds
        .inv_sostenuto
        .togs
        .retain(|&idx| idx != key_idx_val)
}
}