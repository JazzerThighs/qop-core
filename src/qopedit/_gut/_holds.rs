use crate::*;
use duplicate::duplicate_item;

impl Qop<Edit> {
    #[duplicate_item(
        gut_hold_insert_dig_method      field;
        [gut_sustain_insert_dig]        [sustain];
        [gut_inv_sustain_insert_dig]    [inv_sustain];
        [gut_sostenuto_insert_dig]      [sostenuto];
        [gut_inv_sostenuto_insert_dig]  [inv_sostenuto];
      )]
    pub fn gut_hold_insert_dig_method(&mut self, key_idx_val: usize) {
        if !self.gut_holds.field.togs.contains(&key_idx_val) {
            self.gut_holds.field.togs.push(key_idx_val)
        }
    }
    // pub fn gut_inv_sustain_insert_dig(&mut self, key_idx_val: usize) {
    //     if !self.gut_holds.inv_sustain.togs.contains(&key_idx_val) {
    //         self.gut_holds.inv_sustain.togs.push(key_idx_val)
    //     }
    // }
    // pub fn gut_sostenuto_insert_dig(&mut self, key_idx_val: usize) {
    //     if !self.gut_holds.sostenuto.togs.contains(&key_idx_val) {
    //         self.gut_holds.sostenuto.togs.push(key_idx_val)
    //     }
    // }
    // pub fn gut_inv_sostenuto_insert_dig(&mut self, key_idx_val: usize) {
    //     if !self.gut_holds.inv_sostenuto.togs.contains(&key_idx_val) {
    //         self.gut_holds.inv_sostenuto.togs.push(key_idx_val)
    //     }
    // }
    
    #[duplicate_item(
        gut_hold_remove_dig_method      field;
        [gut_sustain_remove_dig]        [sustain];
        [gut_inv_sustain_remove_dig]    [inv_sustain];
        [gut_sostenuto_remove_dig]      [sostenuto];
        [gut_inv_sostenuto_remove_dig]  [inv_sostenuto];
      )]
    pub fn gut_hold_remove_dig_method(&mut self, key_idx_val: usize) {
        self.gut_holds
            .field
            .togs
            .retain(|&idx| idx != key_idx_val)
    }
    // pub fn gut_inv_sustain_remove_dig(&mut self, key_idx_val: usize) {
    //     self.gut_holds
    //         .inv_sustain
    //         .togs
    //         .retain(|&idx| idx != key_idx_val)
    // }
    // pub fn gut_sostenuto_remove_dig(&mut self, key_idx_val: usize) {
    //     self.gut_holds
    //         .sostenuto
    //         .togs
    //         .retain(|&idx| idx != key_idx_val)
    // }
    // pub fn gut_inv_sostenuto_remove_dig(&mut self, key_idx_val: usize) {
    //     self.gut_holds
    //         .inv_sostenuto
    //         .togs
    //         .retain(|&idx| idx != key_idx_val)
    // }
}
