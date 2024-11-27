use crate::qopedit::{TrnspSet, HoldBtns, DeltaTog, IndvSet, BtnTog, Combo, ComboSet, HoldType};

impl TrnspSet {
    pub(crate) fn new(guts: usize) -> Self {
        TrnspSet {
            triggers: vec![],
            i_deltas: vec![0i64; guts],
            x_deltas: vec![0.0f64; guts],
        }
    }
}

impl DeltaTog {
    pub(crate) fn new(guts: usize) -> Self {
        DeltaTog {
            togs: vec![],
            pressed: false,
            i_deltas: vec![0i64; guts],
            x_deltas: vec![0.0f64; guts],
            trnsp_one: vec![],
            tp_i_mem: vec![0i64; guts],
            tp_x_mem: vec![0.0f64; guts],
        }
    }
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        self.i_deltas.insert(g_idx, 0i64);
        self.x_deltas.insert(g_idx, 0.0f64);
        self.tp_i_mem.insert(g_idx, 0i64);
        self.tp_x_mem.insert(g_idx, 0.0f64);
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        self.i_deltas.remove(g_idx);
        self.x_deltas.remove(g_idx);
        self.tp_i_mem.remove(g_idx);
        self.tp_x_mem.remove(g_idx);
    }
}

impl IndvSet {
    pub(crate) fn new(guts: usize) -> Self {
        IndvSet {
            buttons: vec![DeltaTog::new(guts)],
            max_pressed: 1u8,
            min_pressed: 0u8,
            holds: HoldBtns::default(),
            trnsp_all: vec![],
            tp_i_mem: vec![0i64; guts],
            tp_x_mem: vec![0.0f64; guts],
        }
    }
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, guts: usize) {
        if btn_idx <= self.buttons.len() {
            self.buttons.insert(btn_idx, DeltaTog::new(guts));
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        if self.buttons.len() > 0 && btn_idx < self.buttons.len() {
            self.buttons.remove(btn_idx);
        }
    }
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].insert_gut(g_idx);
            self.tp_i_mem.insert(g_idx, 0i64);
            self.tp_x_mem.insert(g_idx, 0.0f64);
        }
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        for btn in 0..self.buttons.len() {
            self.buttons[btn].remove_gut(g_idx);
            self.tp_i_mem.remove(g_idx);
            self.tp_x_mem.remove(g_idx);
        }
    }
    pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
            for t in 0..self.buttons[b].trnsp_one.len() {
                vec_closure(&mut self.buttons[b].trnsp_one[t].triggers);
            }
        }
        for t in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[t].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
    pub(crate) fn btn_insert_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
                self.buttons[btn_idx].togs.push(key_idx_val);
            }
        }
    }
    pub(crate) fn btn_remove_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
        }
    }
    pub(crate) fn hold_insert_key(&mut self, h_kind: HoldType, key_idx_val: usize) {
        match h_kind {
            HoldType::Sustain => {
                if !self.holds.sustain.togs.contains(&key_idx_val) {
                    self.holds.sustain.togs.push(key_idx_val)
                }
            }
            HoldType::InvSustain => {
                if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
                    self.holds.inv_sustain.togs.push(key_idx_val)
                }
            }
            HoldType::Sostenuto => {
                if !self.holds.sostenuto.togs.contains(&key_idx_val) {
                    self.holds.sostenuto.togs.push(key_idx_val)
                }
            }
            HoldType::InvSostenuto => {
                if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
                    self.holds.inv_sostenuto.togs.push(key_idx_val)
                }
            }
            
        }
    }
    pub(crate) fn hold_remove_key(&mut self, h_kind: HoldType, key_idx_val: usize) {
        match h_kind {
            HoldType::Sustain => self.holds.sustain.togs.retain(|&idx| idx != key_idx_val),
            HoldType::InvSustain => self
                .holds
                .inv_sustain
                .togs
                .retain(|&idx| idx != key_idx_val),
            HoldType::Sostenuto => self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val),
            HoldType::InvSostenuto => self
                .holds
                .inv_sostenuto
                .togs
                .retain(|&idx| idx != key_idx_val),
            
        }
    }
    pub(crate) fn trnsp_all_params(
        &mut self,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        guts: usize,
    ) {
        if trnsp_idx == self.trnsp_all.len() {
            let mut tp: TrnspSet = TrnspSet::new(guts);
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    tp.triggers.push(k);
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < guts {
                    if let Some(delta) = i_del {
                        tp.i_deltas[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        tp.x_deltas[x] = delta;
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
                        self.trnsp_all[trnsp_idx].i_deltas[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        self.trnsp_all[trnsp_idx].x_deltas[x] = delta;
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_all_remove_key(&mut self, trnsp_idx: usize, key_idx_val: usize) {
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
                let mut tp: TrnspSet = TrnspSet::new(guts);
                for (_i, &key) in key_idx_vals.iter().enumerate() {
                    if let Some(k) = key {
                        tp.triggers.push(k);
                    }
                }
                for (i, &i_del) in i_del_vec.iter().enumerate() {
                    if i < guts {
                        if let Some(delta) = i_del {
                            tp.i_deltas[i] = delta;
                        }
                    }
                }
                for (x, &x_del) in x_del_vec.iter().enumerate() {
                    if x < guts {
                        if let Some(delta) = x_del {
                            tp.x_deltas[x] = delta;
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
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].i_deltas[i] = delta;
                        }
                    }
                }
                for (x, &x_del) in x_del_vec.iter().enumerate() {
                    if x < guts {
                        if let Some(delta) = x_del {
                            self.buttons[btn_idx].trnsp_one[trnsp_idx].x_deltas[x] = delta;
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_one_remove_key(
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

impl Combo {
    pub(crate) fn new(guts: usize, btns: usize) -> Self {
        Combo {
            combo: vec![false; btns],
            i_deltas: vec![0i64; guts],
            x_deltas: vec![0.0f64; guts],
            trnsp_one: vec![],
            tp_i_mem: vec![0i64; guts],
            tp_x_mem: vec![0.0f64; guts],
        }
    }
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        self.i_deltas.insert(g_idx, 0i64);
        self.x_deltas.insert(g_idx, 0.0f64);
        self.tp_i_mem.insert(g_idx, 0i64);
        self.tp_x_mem.insert(g_idx, 0.0f64);
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        self.i_deltas.remove(g_idx);
        self.x_deltas.remove(g_idx);
        self.tp_i_mem.remove(g_idx);
        self.tp_x_mem.remove(g_idx);
    }
}

impl ComboSet {
    pub(crate) fn new(guts: usize) -> Self {
        ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(guts, 1usize)],
            holds: HoldBtns::default(),
            trnsp_all: vec![],
            tp_i_mem: vec![0i64; guts],
            tp_x_mem: vec![0.0f64; guts],
        }
    }
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
    pub(crate) fn insert_combo(&mut self, c_idx: usize, guts: usize) {
        if c_idx <= self.combos.len() {
            self.combos
                .insert(c_idx, Combo::new(guts, self.buttons.len()));
        }
    }
    pub(crate) fn remove_combo(&mut self, c_idx: usize) {
        if self.combos.len() > 0 && c_idx < self.combos.len() {
            self.combos.remove(c_idx);
        }
    }
    pub(crate) fn insert_gut(&mut self, g_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].insert_gut(g_idx);
            self.tp_i_mem.insert(g_idx, 0i64);
            self.tp_x_mem.insert(g_idx, 0.0f64);
        }
    }
    pub(crate) fn remove_gut(&mut self, g_idx: usize) {
        for c in 0..self.combos.len() {
            self.combos[c].remove_gut(g_idx);
            self.tp_i_mem.remove(g_idx);
            self.tp_x_mem.remove(g_idx);
        }
    }
    pub(crate) fn all_key_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
        }
        for c in 0..self.combos.len() {
            for t in 0..self.combos[c].trnsp_one.len() {
                vec_closure(&mut self.combos[c].trnsp_one[t].triggers);
            }
        }
        for t in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[t].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
    pub(crate) fn btn_insert_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            if !self.buttons[btn_idx].togs.contains(&key_idx_val) {
                self.buttons[btn_idx].togs.push(key_idx_val);
            }
        }
    }
    pub(crate) fn btn_remove_key(&mut self, btn_idx: usize, key_idx_val: usize) {
        if btn_idx < self.buttons.len() {
            self.buttons[btn_idx].togs.retain(|&idx| idx != key_idx_val);
        }
    }
    pub(crate) fn hold_insert_key(&mut self, h_kind: HoldType, key_idx_val: usize) {
        match h_kind {
            HoldType::Sustain => {
                if !self.holds.sustain.togs.contains(&key_idx_val) {
                    self.holds.sustain.togs.push(key_idx_val)
                }
            }
            HoldType::InvSustain => {
                if !self.holds.inv_sustain.togs.contains(&key_idx_val) {
                    self.holds.inv_sustain.togs.push(key_idx_val)
                }
            }
            HoldType::Sostenuto => {
                if !self.holds.sostenuto.togs.contains(&key_idx_val) {
                    self.holds.sostenuto.togs.push(key_idx_val)
                }
            }
            HoldType::InvSostenuto => {
                if !self.holds.inv_sostenuto.togs.contains(&key_idx_val) {
                    self.holds.inv_sostenuto.togs.push(key_idx_val)
                }
            }
            
        }
    }
    pub(crate) fn hold_remove_key(&mut self, h_kind: HoldType, key_idx_val: usize) {
        match h_kind {
            HoldType::Sustain => self.holds.sustain.togs.retain(|&idx| idx != key_idx_val),
            HoldType::InvSustain => self
                .holds
                .inv_sustain
                .togs
                .retain(|&idx| idx != key_idx_val),
            HoldType::Sostenuto => self.holds.sostenuto.togs.retain(|&idx| idx != key_idx_val),
            HoldType::InvSostenuto => self
                .holds
                .inv_sostenuto
                .togs
                .retain(|&idx| idx != key_idx_val),
            
        }
    }
    pub(crate) fn trnsp_all_params(
        &mut self,
        trnsp_idx: usize,
        key_idx_vals: Vec<Option<usize>>,
        i_del_vec: Vec<Option<i64>>,
        x_del_vec: Vec<Option<f64>>,
        guts: usize,
    ) {
        if trnsp_idx == self.trnsp_all.len() {
            let mut tp: TrnspSet = TrnspSet::new(guts);
            for (_i, &key) in key_idx_vals.iter().enumerate() {
                if let Some(k) = key {
                    tp.triggers.push(k);
                }
            }
            for (i, &i_del) in i_del_vec.iter().enumerate() {
                if i < guts {
                    if let Some(delta) = i_del {
                        tp.i_deltas[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        tp.x_deltas[x] = delta;
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
                        self.trnsp_all[trnsp_idx].i_deltas[i] = delta;
                    }
                }
            }
            for (x, &x_del) in x_del_vec.iter().enumerate() {
                if x < guts {
                    if let Some(delta) = x_del {
                        self.trnsp_all[trnsp_idx].x_deltas[x] = delta;
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_all_remove_key(&mut self, trnsp_idx: usize, key_idx_val: usize) {
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
                    let mut tp: TrnspSet = TrnspSet::new(guts);
                    for (_i, &key) in key_idx_vals.iter().enumerate() {
                        if let Some(k) = key {
                            tp.triggers.push(k);
                        }
                    }
                    for (i, &i_del) in i_del_vec.iter().enumerate() {
                        if i < guts {
                            if let Some(delta) = i_del {
                                tp.i_deltas[i] = delta;
                            }
                        }
                    }
                    for (x, &x_del) in x_del_vec.iter().enumerate() {
                        if x < guts {
                            if let Some(delta) = x_del {
                                tp.x_deltas[x] = delta;
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
                                self.combos[c_idx].trnsp_one[trnsp_idx].i_deltas[i] = delta;
                            }
                        }
                    }
                    for (x, &x_del) in x_del_vec.iter().enumerate() {
                        if x < guts {
                            if let Some(delta) = x_del {
                                self.combos[c_idx].trnsp_one[trnsp_idx].x_deltas[x] = delta;
                            }
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn trnsp_one_remove_key(
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
