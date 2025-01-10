mod _dig_inputs;
mod _gut;
mod _holds;
mod _trnsp;
mod _vfc_sets;

use crate::*;
use better_default::Default;

#[derive(Default, Clone)]
pub(crate) struct NewEnginePartParams {
    pub(crate) guts_len: usize,
    pub(crate) g_idx: usize,
    pub(crate) set_idx: usize,
    pub(crate) del_idx: usize,
    pub(crate) trnsp_idx: usize,
    pub(crate) c_btn_len: usize,
}

impl NewEnginePartParams {
    pub(crate) fn new(engine: &Engine) -> Self {
        NewEnginePartParams {
            guts_len: engine.guts.len(),
            c_btn_len: 1,
            ..Default::default()
        }
    }
}

pub(crate) trait NewTrait: Default {
    fn new(n: &mut NewEnginePartParams) -> Self;
}

impl NewTrait for VFSet {
    fn new(n: &mut NewEnginePartParams) -> Self {
        VFSet {
            buttons: vec![VFBtn::new(n)],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for VFBtn {
    fn new(n: &mut NewEnginePartParams) -> Self {
        VFBtn {
            i_delta: vec![0i64; n.guts_len],
            x_delta: vec![0.0f64; n.guts_len],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for ComboSet {
    fn new(n: &mut NewEnginePartParams) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            combos: vec![Combo::new(n)],
            i_mem: vec![0i64; n.guts_len],
            x_mem: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for Combo {
    fn new(n: &mut NewEnginePartParams) -> Self {
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
    fn new(_n: &mut NewEnginePartParams) -> Self {
        Trnsp::default()
    }
}
impl NewTrait for Trnsp<Vec<i64>, Vec<f64>> {
    fn new(n: &mut NewEnginePartParams) -> Self {
        Trnsp {
            i_delta: vec![0i64; n.guts_len],
            x_delta: vec![0.0f64; n.guts_len],
            ..Default::default()
        }
    }
}
