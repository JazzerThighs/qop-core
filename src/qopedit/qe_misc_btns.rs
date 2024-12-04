use crate::*;

// impl MultiSet {
//     pub(crate) fn insert_btn(&mut self, btn_idx: usize, guts: usize) {
//         if btn_idx <= self.buttons.len() {
//             self.buttons.insert(btn_idx, MultiTog::new(guts));
//         }
//     }
//     pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
//         if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
//             self.buttons.remove(btn_idx);
//         }
//     }
//     pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
//         for b in 0..self.buttons.len() {
//             vec_closure(&mut self.buttons[b].togs);
//             for t in 0..self.buttons[b].trnsp_one.len() {
//                 vec_closure(&mut self.buttons[b].trnsp_one[t].triggers);
//             }
//         }
//         for t in 0..self.trnsp_all.len() {
//             vec_closure(&mut self.trnsp_all[t].triggers);
//         }
//         vec_closure(&mut self.holds.sustain.togs);
//         vec_closure(&mut self.holds.inv_sustain.togs);
//         vec_closure(&mut self.holds.sostenuto.togs);
//         vec_closure(&mut self.holds.inv_sostenuto.togs);
//     }
//     pub(crate) fn btn_insert_key(&mut self, btn_idx: usize, key_idx_val: usize) {
//         if btn_idx < self.buttons.len() {
//             if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
//                 self.buttons[btn_idx].togs.push(key_idx_val);
//             }
//         }
//     }
//     pub(crate) fn btn_remove_key(&mut self, btn_idx: usize, key_idx_val: usize) {
//         if btn_idx < self.buttons.len() {
//             self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
//         }
//     }
//     pub(crate) fn sustain_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.sustain.togs.contains(&key_idx_val) {
//             self.holds.sustain.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn inv_sustain_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
//             self.holds.inv_sustain.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn sostenuto_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.sostenuto.togs.contains(&key_idx_val) {
//             self.holds.sostenuto.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn inv_sostenuto_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
//             self.holds.inv_sostenuto.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn sustain_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.sustain.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn inv_sustain_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.inv_sustain.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn sostenuto_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn inv_sostenuto_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.inv_sostenuto.togs.retain(|&idx| idx != key_idx_val)
//     }
// 
//     pub(crate) fn trnsp_all_params(
//         &mut self,
//         trnsp_idx: usize,
//         key_idx_vals: Vec<Option<usize>>,
//         i_del_vec: Vec<Option<i64>>,
//         x_del_vec: Vec<Option<f64>>,
//         guts: usize,
//     ) {
//         if trnsp_idx == self.trnsp_all.len() {
//             let mut tp: MultiTrnsp = MultiTrnsp::new(guts);
//             for (_i, &key) in key_idx_vals.iter().enumerate() {
//                 if let Some(k) = key {
//                     tp.triggers.push(k);
//                 }
//             }
//             for (i, &i_del) in i_del_vec.iter().enumerate() {
//                 if i < guts {
//                     if let Some(delta) = i_del {
//                         tp.i_deltas[i] = delta;
//                     }
//                 }
//             }
//             for (x, &x_del) in x_del_vec.iter().enumerate() {
//                 if x < guts {
//                     if let Some(delta) = x_del {
//                         tp.x_deltas[x] = delta;
//                     }
//                 }
//             }
//             self.trnsp_all.push(tp);
//         } else if trnsp_idx < self.trnsp_all.len() {
//             for (_i, &key) in key_idx_vals.iter().enumerate() {
//                 if let Some(k) = key {
//                     if !self.trnsp_all[trnsp_idx].triggers.contains(&k) {
//                         self.trnsp_all[trnsp_idx].triggers.push(k);
//                     }
//                 }
//             }
//             for (i, &i_del) in i_del_vec.iter().enumerate() {
//                 if i < guts {
//                     if let Some(delta) = i_del {
//                         self.trnsp_all[trnsp_idx].i_deltas[i] = delta;
//                     }
//                 }
//             }
//             for (x, &x_del) in x_del_vec.iter().enumerate() {
//                 if x < guts {
//                     if let Some(delta) = x_del {
//                         self.trnsp_all[trnsp_idx].x_deltas[x] = delta;
//                     }
//                 }
//             }
//         }
//     }
//     pub(crate) fn trnsp_all_remove_key(&mut self, trnsp_idx: usize, key_idx_val: usize) {
//         if trnsp_idx < self.trnsp_all.len() {
//             self.trnsp_all[trnsp_idx]
//                 .triggers
//                 .retain(|&idx| idx != key_idx_val);
//         }
//     }
//     pub(crate) fn trnsp_all_remove(&mut self, trnsp_idx: usize) {
//         if trnsp_idx < self.trnsp_all.len() {
//             self.trnsp_all.remove(trnsp_idx);
//         }
//     }
//     pub(crate) fn trnsp_one_params(
//         &mut self,
//         btn_idx: usize,
//         trnsp_idx: usize,
//         key_idx_vals: Vec<Option<usize>>,
//         i_del_vec: Vec<Option<i64>>,
//         x_del_vec: Vec<Option<f64>>,
//         guts: usize,
//     ) {
//         if btn_idx < self.buttons.len() {
//             if trnsp_idx == self.buttons[btn_idx].trnsp_one.len() {
//                 let mut tp: MultiTrnsp = MultiTrnsp::new(guts);
//                 for (_i, &key) in key_idx_vals.iter().enumerate() {
//                     if let Some(k) = key {
//                         tp.triggers.push(k);
//                     }
//                 }
//                 for (i, &i_del) in i_del_vec.iter().enumerate() {
//                     if i < guts {
//                         if let Some(delta) = i_del {
//                             tp.i_deltas[i] = delta;
//                         }
//                     }
//                 }
//                 for (x, &x_del) in x_del_vec.iter().enumerate() {
//                     if x < guts {
//                         if let Some(delta) = x_del {
//                             tp.x_deltas[x] = delta;
//                         }
//                     }
//                 }
//                 self.buttons[btn_idx].trnsp_one.push(tp);
//             } else if trnsp_idx < self.buttons[btn_idx].trnsp_one.len() {
//                 for (_i, &key) in key_idx_vals.iter().enumerate() {
//                     if let Some(k) = key {
//                         if !self.buttons[btn_idx].trnsp_one[trnsp_idx]
//                             .triggers
//                             .contains(&k)
//                         {
//                             self.buttons[btn_idx].trnsp_one[trnsp_idx].triggers.push(k);
//                         }
//                     }
//                 }
//                 for (i, &i_del) in i_del_vec.iter().enumerate() {
//                     if i < guts {
//                         if let Some(delta) = i_del {
//                             self.buttons[btn_idx].trnsp_one[trnsp_idx].i_deltas[i] = delta;
//                         }
//                     }
//                 }
//                 for (x, &x_del) in x_del_vec.iter().enumerate() {
//                     if x < guts {
//                         if let Some(delta) = x_del {
//                             self.buttons[btn_idx].trnsp_one[trnsp_idx].x_deltas[x] = delta;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     pub(crate) fn trnsp_one_remove_key(
//         &mut self,
//         btn_idx: usize,
//         trnsp_idx: usize,
//         key_idx_val: usize,
//     ) {
//         if btn_idx < self.buttons.len() {
//             if trnsp_idx < self.buttons[btn_idx].trnsp_one.len() {
//                 self.buttons[btn_idx].trnsp_one[trnsp_idx]
//                     .triggers
//                     .retain(|&idx| idx != key_idx_val);
//             }
//         }
//     }
//     pub(crate) fn trnsp_one_remove(&mut self, btn_idx: usize, trnsp_idx: usize) {
//         if btn_idx < self.buttons.len() {
//             if trnsp_idx < self.buttons[btn_idx].trnsp_one.len() {
//                 self.buttons[btn_idx].trnsp_one.remove(trnsp_idx);
//             }
//         }
//     }
// }

// impl ComboSet {
//     pub(crate) fn new(guts: usize) -> Self {
//         MultiComboSet {
//             buttons: vec![BtnTog::default()],
//             combos: vec![Combo::new(guts, 1usize)],
//             i_mem: vec![0i64; guts],
//             x_mem: vec![0.0f64; guts],
//             ..Default::default()
//         }
//     }
//     pub(crate) fn insert_btn(&mut self, btn_idx: usize) {
//         if btn_idx <= self.buttons.len() {
//             self.buttons.insert(btn_idx, BtnTog::default());
//             for c in 0..self.combos.len() {
//                 self.combos[c].combo.insert(btn_idx, false);
//             }
//         }
//     }
//     pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
//         if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
//             self.buttons.remove(btn_idx);
//             for c in 0..self.combos.len() {
//                 self.combos[c].combo.remove(btn_idx);
//             }
//         }
//     }
//     pub(crate) fn insert_combo(&mut self, c_idx: usize, guts: usize) {
//         if c_idx <= self.combos.len() {
//             self.combos
//                 .insert(c_idx, Combo::new(guts, self.buttons.len()));
//         }
//     }
//     pub(crate) fn remove_combo(&mut self, c_idx: usize) {
//         if self.combos.len() > 0 && c_idx < self.combos.len() {
//             self.combos.remove(c_idx);
//         }
//     }
//     pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
//         for b in 0..self.buttons.len() {
//             vec_closure(&mut self.buttons[b].togs);
//         }
//         for c in 0..self.combos.len() {
//             for t in 0..self.combos[c].trnsp_one.len() {
//                 vec_closure(&mut self.combos[c].trnsp_one[t].triggers);
//             }
//         }
//         for t in 0..self.trnsp_all.len() {
//             vec_closure(&mut self.trnsp_all[t].triggers);
//         }
//         vec_closure(&mut self.holds.sustain.togs);
//         vec_closure(&mut self.holds.inv_sustain.togs);
//         vec_closure(&mut self.holds.sostenuto.togs);
//         vec_closure(&mut self.holds.inv_sostenuto.togs);
//     }
//     pub(crate) fn btn_insert_key(&mut self, btn_idx: usize, key_idx_val: usize) {
//         if btn_idx < self.buttons.len() {
//             if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
//                 self.buttons[btn_idx].togs.push(key_idx_val);
//             }
//         }
//     }
//     pub(crate) fn btn_remove_key(&mut self, btn_idx: usize, key_idx_val: usize) {
//         if btn_idx < self.buttons.len() {
//             self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
//         }
//     }
//     pub(crate) fn sustain_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.sustain.togs.contains(&key_idx_val) {
//             self.holds.sustain.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn inv_sustain_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
//             self.holds.inv_sustain.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn sostenuto_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.sostenuto.togs.contains(&key_idx_val) {
//             self.holds.sostenuto.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn inv_sostenuto_insert_key(&mut self, key_idx_val: usize) {
//         if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
//             self.holds.inv_sostenuto.togs.push(key_idx_val)
//         }
//     }
//     pub(crate) fn sustain_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.sustain.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn inv_sustain_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.inv_sustain.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn sostenuto_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn inv_sostenuto_remove_key(&mut self, key_idx_val: usize) {
//         self.holds.inv_sostenuto.togs.retain(|&idx| idx != key_idx_val)
//     }
//     pub(crate) fn trnsp_all_params(
//         &mut self,
//         trnsp_idx: usize,
//         key_idx_vals: Vec<Option<usize>>,
//         i_del_vec: Vec<Option<i64>>,
//         x_del_vec: Vec<Option<f64>>,
//         guts: usize,
//     ) {
//         if trnsp_idx == self.trnsp_all.len() {
//             let mut tp: MultiTrnsp = MultiTrnsp::new(guts);
//             for (_i, &key) in key_idx_vals.iter().enumerate() {
//                 if let Some(k) = key {
//                     tp.triggers.push(k);
//                 }
//             }
//             for (i, &i_del) in i_del_vec.iter().enumerate() {
//                 if i < guts {
//                     if let Some(delta) = i_del {
//                         tp.i_deltas[i] = delta;
//                     }
//                 }
//             }
//             for (x, &x_del) in x_del_vec.iter().enumerate() {
//                 if x < guts {
//                     if let Some(delta) = x_del {
//                         tp.x_deltas[x] = delta;
//                     }
//                 }
//             }
//             self.trnsp_all.push(tp);
//         } else if trnsp_idx < self.trnsp_all.len() {
//             for (_i, &key) in key_idx_vals.iter().enumerate() {
//                 if let Some(k) = key {
//                     if !self.trnsp_all[trnsp_idx].triggers.contains(&k) {
//                         self.trnsp_all[trnsp_idx].triggers.push(k);
//                     }
//                 }
//             }
//             for (i, &i_del) in i_del_vec.iter().enumerate() {
//                 if i < guts {
//                     if let Some(delta) = i_del {
//                         self.trnsp_all[trnsp_idx].i_deltas[i] = delta;
//                     }
//                 }
//             }
//             for (x, &x_del) in x_del_vec.iter().enumerate() {
//                 if x < guts {
//                     if let Some(delta) = x_del {
//                         self.trnsp_all[trnsp_idx].x_deltas[x] = delta;
//                     }
//                 }
//             }
//         }
//     }
//     pub(crate) fn trnsp_all_remove_key(&mut self, trnsp_idx: usize, key_idx_val: usize) {
//         if trnsp_idx < self.trnsp_all.len() {
//             self.trnsp_all[trnsp_idx]
//                 .triggers
//                 .retain(|&idx| idx != key_idx_val);
//         }
//     }
//     pub(crate) fn trnsp_all_remove(&mut self, trnsp_idx: usize) {
//         if trnsp_idx < self.trnsp_all.len() {
//             self.trnsp_all.remove(trnsp_idx);
//         }
//     }
//     pub(crate) fn trnsp_one_params(
//         &mut self,
//         c_idx: usize,
//         trnsp_idx: usize,
//         key_idx_vals: Vec<Option<usize>>,
//         i_del_vec: Vec<Option<i64>>,
//         x_del_vec: Vec<Option<f64>>,
//         guts: usize,
//     ) {
//         if c_idx < self.combos.len() {
//             if c_idx < self.buttons.len() {
//                 if trnsp_idx == self.combos[c_idx].trnsp_one.len() {
//                     let mut tp: MultiTrnsp = MultiTrnsp::new(guts);
//                     for (_i, &key) in key_idx_vals.iter().enumerate() {
//                         if let Some(k) = key {
//                             tp.triggers.push(k);
//                         }
//                     }
//                     for (i, &i_del) in i_del_vec.iter().enumerate() {
//                         if i < guts {
//                             if let Some(delta) = i_del {
//                                 tp.i_deltas[i] = delta;
//                             }
//                         }
//                     }
//                     for (x, &x_del) in x_del_vec.iter().enumerate() {
//                         if x < guts {
//                             if let Some(delta) = x_del {
//                                 tp.x_deltas[x] = delta;
//                             }
//                         }
//                     }
//                     self.combos[c_idx].trnsp_one.push(tp);
//                 } else if trnsp_idx < self.combos[c_idx].trnsp_one.len() {
//                     for (_i, &key) in key_idx_vals.iter().enumerate() {
//                         if let Some(k) = key {
//                             if !self.combos[c_idx].trnsp_one[trnsp_idx]
//                                 .triggers
//                                 .contains(&k)
//                             {
//                                 self.combos[c_idx].trnsp_one[trnsp_idx].triggers.push(k);
//                             }
//                         }
//                     }
//                     for (i, &i_del) in i_del_vec.iter().enumerate() {
//                         if i < guts {
//                             if let Some(delta) = i_del {
//                                 self.combos[c_idx].trnsp_one[trnsp_idx].i_deltas[i] = delta;
//                             }
//                         }
//                     }
//                     for (x, &x_del) in x_del_vec.iter().enumerate() {
//                         if x < guts {
//                             if let Some(delta) = x_del {
//                                 self.combos[c_idx].trnsp_one[trnsp_idx].x_deltas[x] = delta;
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     pub(crate) fn trnsp_one_remove_key(
//         &mut self,
//         c_idx: usize,
//         trnsp_idx: usize,
//         key_idx_val: usize,
//     ) {
//         if c_idx < self.combos.len() {
//             if trnsp_idx < self.combos[c_idx].trnsp_one.len() {
//                 self.combos[c_idx].trnsp_one[trnsp_idx]
//                     .triggers
//                     .retain(|&idx| idx != key_idx_val);
//             }
//         }
//     }
//     pub(crate) fn trnsp_one_remove(&mut self, c_idx: usize, trnsp_idx: usize) {
//         if c_idx < self.combos.len() {
//             if trnsp_idx < self.combos[c_idx].trnsp_one.len() {
//                 self.combos[c_idx].trnsp_one.remove(trnsp_idx);
//             }
//         }
//     }
// }
