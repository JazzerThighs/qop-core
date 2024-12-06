use crate::*;
use duplicate::duplicate_item;

pub(crate) trait NewTrait: Default {
    fn new(n: &mut NewStuffPointers) -> Self;
}

#[duplicate_item(
    ForType             ConstructType;
    [Gut]               [Gut];
    [VFRIndv<i64, f64>] [VFRIndv];
    [Trnsp<i64, f64>]   [Trnsp];
)]
impl NewTrait for ForType {
    fn new(_n: &mut NewStuffPointers) -> Self {
        ConstructType::default()
    }
}

impl<T, U> NewTrait for VFRSet<T, U>
where
    T: Default,
    U: Default,
    VFRIndv<T, U>: NewTrait,
{
    fn new(n: &mut NewStuffPointers) -> Self {
        VFRSet {
            buttons: vec![VFRIndv::new(n)],
            ..Default::default()
        }
    }
}

impl NewTrait for VFRIndv<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewStuffPointers) -> Self {
        VFRIndv {
            i_delta: vec![0i64; n.guts_len],
            x_delta: vec![0.0f64; n.guts_len],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl<T, U> NewTrait for ComboSet<T, U>
where
    T: Default,
    U: Default,
    Combo<T, U>: NewTrait,
{
    fn new(n: &mut NewStuffPointers) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(n)],
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
mod _vfrc_sets;
mod _trnsp;
