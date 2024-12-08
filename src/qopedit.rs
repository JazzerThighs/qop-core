use crate::*;
use duplicate::duplicate_item;

pub(crate) trait NewTrait: Default {
    fn new(n: &mut NewStuffPointers) -> Self;
}

#[duplicate_item(
    ForType             ConstructType;
    [Gut]               [Gut];
    [VFBtn<i64, f64>]   [VFBtn];
    [Trnsp<i64, f64>]   [Trnsp];
)]
impl NewTrait for ForType {
    fn new(_n: &mut NewStuffPointers) -> Self {
        ConstructType::default()
    }
}

impl NewTrait for VFSet<i64, f64> {
    fn new(n: &mut NewStuffPointers) -> Self {
        VFSet {
            buttons: vec![VFBtn::new(n)],
            ..Default::default()
        }
    }
}
impl NewTrait for VFSet<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewStuffPointers) -> Self {
        VFSet {
            buttons: vec![VFBtn::new(n)],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for VFBtn<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewStuffPointers) -> Self {
        VFBtn {
            i_delta: vec![0i64; n.guts_len],
            x_delta: vec![0.0f64; n.guts_len],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for ComboSet<i64, f64> {
    fn new(n: &mut NewStuffPointers) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(n)],
            ..Default::default()
        }
    }
}
impl NewTrait for ComboSet<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewStuffPointers) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(n)],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for Combo<i64, f64> {
    fn new(n: &mut NewStuffPointers) -> Self {
        Combo {
            combo: vec![false; n.c_btn_len],
            ..Default::default()
        }
    }
}
impl NewTrait for Combo<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewStuffPointers) -> Self {
        Combo {
            combo: vec![false; n.c_btn_len],
            i_delta: vec![0i64; n.guts_len],
            x_delta: vec![0.0f64; n.guts_len],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for Trnsp<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewStuffPointers) -> Self {
        Trnsp {
            i_delta: vec![0i64; n.guts_len],
            x_delta: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

mod _dig_inputs;
mod _gut;
mod _holds;
mod _trnsp;
mod _vfc_sets;
