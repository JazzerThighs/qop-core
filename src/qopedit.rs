use crate::*

mod qe_asserts;
mod qe_gut_methods;
mod qe_kcs_methods;
mod qe_misc_btns;
mod qe_set_methods;
mod qe_trnsp_methods;

pub trait NewTrait: Default {
    fn new(n: &NewStuffPointers) -> Self;
}

impl<T: Default, U: Default> NewTrait for VFRSet<T, U> {
    fn new(_n: &NewStuffPointers) -> Self {
        VFRSet {
            max_pressed: 1usize, 
            ..Default::default()
        }
    }
}
impl NewTrait for VFRIndv<i64, f64> {
    fn new(_n: &NewStuffPointers) -> Self {
        VFRIndv::default()
    }
}
impl NewTrait for VFRIndv<Vec<i64>, Vec<f64>> {
    fn new(n: &NewStuffPointers) -> Self {
        VFRIndv {
            i_delta: vec![0i64; n.len_guts],
            x_delta: vec![0.0f64; n.len_guts],
            i_mem: vec![0i64; n.len_guts],
            x_mem: vec![0.0f64; n.len_guts],
            ..Default::default()
        }
    }
}

impl<T: Default, U: Default> NewTrait for ComboSet<T, U> where Combo<T, U>: NewTrait {
    fn new(n: &NewStuffPointers) -> Self {
        ComboSet {
            buttons: vec![BtnTog::default()],
            combos: vec![Combo::new(n)],
            ..Default::default()
        }
    }
}

impl NewTrait for Combo<i64, f64> {
    fn new(n: &NewStuffPointers) -> Self {
        Combo::default()
    }
}
impl NewTrait for Combo<Vec<i64>, Vec<f64>> {
    fn new(n: &NewStuffPointers) -> Self {
        Combo {
            combo: vec![false],
            i_delta: vec![0i64; n.len_guts],
            x_delta: vec![0.0f64; n.len_guts],
            i_mem: vec![0i64; n.len_guts],
            x_mem: vec![0.0f64; n.len_guts],
            ..Default::default()
        }
    }
}

impl NewTrait for Trnsp<i64, f64> {
    fn new(n: &NewStuffPointers) -> Self {
    }
}
impl NewTrait for Trnsp<Vec<i64>, Vec<f64>> {
    fn new(n: &NewStuffPointers) -> Self {
    }
}