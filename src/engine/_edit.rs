mod _dig_inputs;
mod _gut;
mod _holds;
mod _trnsp;
mod _vfc_sets;

use crate::*;
use better_default::Default;

#[derive(Default, Clone)]
pub(crate) struct NewEnginePartParams<_I: Int, _F: Flo> where f32: From<_F> {
    pub(crate) guts_len: usize,
    pub(crate) g_idx: usize,
    pub(crate) set_idx: usize,
    pub(crate) del_idx: usize,
    pub(crate) trnsp_idx: usize,
    pub(crate) c_btn_len: usize,
    _i: _I,
    _f: _F
}

impl<_I: Int, _F: Flo> NewEnginePartParams<_I, _F> where f32: From<_F> {
    pub(crate) fn new(engine: &Engine<_I, _F>) -> Self {
        NewEnginePartParams {
            guts_len: engine.guts.len(),
            c_btn_len: 1,
            ..Self::default()
        }
    }
}

pub(crate) trait NewTrait<_I: Int, _F: Flo>: Default where f32: From<_F> {
    fn new(n: &mut NewEnginePartParams<_I, _F>) -> Self;
}

impl<I: Int, F: Flo> NewTrait<I, F> for VFSet<I, F> where f32: From<F> {
    fn new(n: &mut NewEnginePartParams<I, F>) -> Self {
        VFSet {
            buttons: vec![VFBtn::new(n)],
            i_mem: vec![I::default(); n.guts_len],
            x_mem: vec![F::default(); n.guts_len],
            ..Default::default()
        }
    }
}

impl<I: Int, F: Flo> NewTrait<I, F> for VFBtn<I, F> where f32: From<F> {
    fn new(n: &mut NewEnginePartParams<I, F>) -> Self {
        VFBtn {
            i_delta: vec![I::default(); n.guts_len],
            x_delta: vec![F::default(); n.guts_len],
            i_mem: vec![I::default(); n.guts_len],
            x_mem: vec![F::default(); n.guts_len],
            ..Default::default()
        }
    }
}

impl<I: Int, F: Flo> NewTrait<I, F> for ComboSet<I, F> where f32: From<F> {
    fn new(n: &mut NewEnginePartParams<I, F>) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            combos: vec![Combo::new(n)],
            i_mem: vec![I::default(); n.guts_len],
            x_mem: vec![F::default(); n.guts_len],
            ..Default::default()
        }
    }
}

impl<I: Int, F: Flo> NewTrait<I, F> for Combo<I, F> where f32: From<F> {
    fn new(n: &mut NewEnginePartParams<I, F>) -> Self {
        Combo {
            combo: vec![false; n.c_btn_len],
            i_delta: vec![I::default(); n.guts_len],
            x_delta: vec![F::default(); n.guts_len],
            i_mem: vec![I::default(); n.guts_len],
            x_mem: vec![F::default(); n.guts_len],
            ..Default::default()
        }
    }
}

impl<I: Int, F: Flo> NewTrait<I, F> for Trnsp<I, F> where f32: From<F> {
    fn new(_n: &mut NewEnginePartParams<I, F>) -> Self {
        Self::default()
    }
}

impl<I: Int, F: Flo> NewTrait<I, F> for MulTrnsp<I, F> where f32: From<F> {
    fn new(n: &mut NewEnginePartParams<I, F>) -> Self {
        MulTrnsp {
            i_delta: vec![I::default(); n.guts_len],
            x_delta: vec![F::default(); n.guts_len],
            ..Default::default()
        }
    }
}
