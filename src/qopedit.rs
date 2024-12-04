use crate::*;

pub trait NewTrait: Default {
    fn new(n: &mut NewStuffPointers) -> Self;
}

impl NewTrait for Gut {
    fn new(n: &mut NewStuffPointers) -> Self {
        n.guts_len += 1;
        Gut::default()
    }
}

impl<T: Default, U: Default> NewTrait for VFRSet<T, U> where VFRIndv<T, U>: NewTrait {
    fn new(n: &mut NewStuffPointers) -> Self {
        VFRSet {
            buttons: vec![VFRIndv::new(n)],
            ..Default::default()
        }
    }
}

impl NewTrait for VFRIndv<i64, f64> {
    fn new(_n: &mut NewStuffPointers) -> Self {
        VFRIndv::default()
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

impl NewTrait for Trnsp<i64, f64> {
    fn new(_n: &mut NewStuffPointers) -> Self {
        Trnsp::default()
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

mod qe_asserts;
mod qe_gut_methods;
mod qe_kcs_methods;
mod qe_misc_btns;
mod qe_set_methods;
mod qe_trnsp_methods;
