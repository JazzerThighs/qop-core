mod _edit;
mod _play;

use crate::{engine::_edit::*, *};
use better_default::Default;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};
use winit::keyboard::KeyCode;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub struct Engine<Mode = Edit> {
        pub(crate) _mode: PhantomData<Mode>,
        pub name: String,
        pub description: String,
        pub(crate) dig_inputs: Vec<KeyCode>,
        pub(crate) analog_inputs: Vec<(usize, usize)>,
        pub(crate) i_delta_mode: bool,
        pub(crate) x_delta_mode: bool,
        pub(crate) max_pressed: usize,
        pub(crate) min_pressed: usize,
        pub(crate) radio_mode: bool,
        pub(crate) holds_mode: bool,
        pub(crate) holds:
            pub(crate) struct HoldBtns {
                pub(crate) sustain:
                    pub(crate) struct HoldTog {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool
                    },
                pub(crate) inv_sustain: HoldTog,
                pub(crate) sostenuto: HoldTog,
                pub(crate) inv_sostenuto: HoldTog,
            },
        pub(crate) trnsp_mode: bool,
        pub(crate) trnsp_all: Vec<
            pub(crate) struct MulTrnsp {
                pub(crate) triggers: Vec<usize>,
                pub(crate) i_delta: Vec<i32>,
                pub(crate) x_delta: Vec<f64>,
            }
        >,
        pub(crate) analog_mode: bool,
        pub(crate) analog_all: Vec<
            pub(crate) struct MulAnalogMod {
                pots: Vec<usize>,
                i_mem: Vec<i32>,
                x_mem: Vec<f64>,
                pot_input_nodes: Vec<
                    pub(crate) struct MulAnalogModNode {
                        pot_value: usize,
                        i_delta: Vec<i32>,
                        x_delta: Vec<f64>
                    }
                >,
            }
        >,
        #[default(vec![Gut::default()])]
        pub(crate) guts: Vec<
            pub(crate) struct Gut {
                pub name: String,
                pub description: String,
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
                pub(crate) i_mem: i32,
                pub(crate) x_mem: f64,
                pub(crate) index_out: usize,
                pub(crate) extra_out: f64,
                pub(crate) trnsp_one: Vec<
                    pub(crate) struct Trnsp {
                        pub(crate) triggers: Vec<usize>,
                        pub(crate) i_delta: i32,
                        pub(crate) x_delta: f64,
                    }
                >,
                pub(crate) analog_one: Vec<
                    pub(crate) struct AnalogMod {
                        pots: Vec<usize>,
                        i_mem: i32,
                        x_mem: f64,
                        pot_input_nodes: Vec<
                            pub(crate) struct AnalogModNode {
                                pot_value: usize,
                                i_delta: i32,
                                x_delta: f64
                            }
                        >,
                    }
                >,
            }
        >,
        pub(crate) v_multi: Vec<
            pub(crate) struct VFSet {
                pub name: String,
                pub description: String,
                #[default(vec![false])]
                pub(crate) pressed: Vec<bool>,
                pub(crate) i_mem: Vec<i32>,
                pub(crate) x_mem: Vec<f64>,
                pub(crate) max_pressed: usize,
                pub(crate) min_pressed: usize,
                pub(crate) radio_mode: bool,
                pub(crate) holds: HoldBtns,
                pub(crate) trnsp_all: Vec<MulTrnsp>,
                pub(crate) analog_all: Vec<MulAnalogMod>,
                pub(crate) buttons: Vec<
                    pub(crate) struct VFBtn {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) i_delta: Vec<i32>,
                        pub(crate) x_delta: Vec<f64>,
                        pub(crate) i_mem: Vec<i32>,
                        pub(crate) x_mem: Vec<f64>,
                        pub(crate) trnsp_one: Vec<MulTrnsp>,
                        pub(crate) analog_one: Vec<MulAnalogMod>,
                    }
                >,
            }
        >,
        pub(crate) f_multi: Vec<VFSet>,
        pub(crate) c_multi: Vec<
            pub(crate) struct ComboSet {
                pub name: String,
                pub description: String,
                #[default(vec![false])]
                pub(crate) pressed: Vec<bool>,
                pub(crate) i_mem: Vec<i32>,
                pub(crate) x_mem: Vec<f64>,
                pub(crate) max_pressed: usize,
                pub(crate) min_pressed: usize,
                pub(crate) radio_mode: bool,
                pub(crate) holds: HoldBtns,
                pub(crate) trnsp_all: Vec<MulTrnsp>,
                pub(crate) analog_all: Vec<MulAnalogMod>,
                #[default(vec![ComboTog::default()])]
                pub(crate) buttons: Vec<
                    pub(crate) struct ComboTog {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                    }
                >,
                pub(crate) combos: Vec<
                    pub(crate) struct Combo {
                        pub name: String,
                        pub description: String,
                        pub(crate) combo: Vec<bool>,
                        pub(crate) i_delta: Vec<i32>,
                        pub(crate) x_delta: Vec<f64>,
                        pub(crate) i_mem: Vec<i32>,
                        pub(crate) x_mem: Vec<f64>,
                        pub(crate) trnsp_one: Vec<MulTrnsp>,
                        pub(crate) analog_one: Vec<MulAnalogMod>,
                    }
                >,
            }
        >,
    }
}

impl Engine<Edit> {
    pub fn new_saturated() -> Engine<Edit> {
        let mut engine: Engine<Edit> = Engine {
            dig_inputs: vec![KeyCode::KeyA],
            analog_inputs: vec![(0, 1)],
            ..Default::default()
        };
        let mut n = NewEnginePartParams::new(&engine);
        engine.trnsp_all = vec![MulTrnsp::new(&mut n)];
        engine.analog_all = vec![MulAnalogMod::new(&mut n)];
        
        engine.guts[0].trnsp_one = vec![Trnsp::new(&mut n)];
        engine.guts[0].analog_one = vec![AnalogMod::new(&mut n)];
        
        engine.v_multi = vec![VFSet::new(&mut n)];
        engine.v_multi[0].trnsp_all = vec![MulTrnsp::new(&mut n)];
        engine.v_multi[0].analog_all = vec![MulAnalogMod::new(&mut n)];
        engine.v_multi[0].buttons[0].trnsp_one = vec![MulTrnsp::new(&mut n)];
        engine.v_multi[0].buttons[0].analog_one = vec![MulAnalogMod::new(&mut n)];

        engine.f_multi = vec![VFSet::new(&mut n)];
        engine.f_multi[0].trnsp_all = vec![MulTrnsp::new(&mut n)];
        engine.f_multi[0].analog_all = vec![MulAnalogMod::new(&mut n)];
        engine.f_multi[0].buttons[0].trnsp_one = vec![MulTrnsp::new(&mut n)];
        engine.f_multi[0].buttons[0].analog_one = vec![MulAnalogMod::new(&mut n)];

        engine.c_multi = vec![ComboSet::new(&mut n)];
        engine.c_multi[0].trnsp_all = vec![MulTrnsp::new(&mut n)];
        engine.c_multi[0].analog_all = vec![MulAnalogMod::new(&mut n)];
        engine.c_multi[0].combos[0].trnsp_one = vec![MulTrnsp::new(&mut n)];
        engine.c_multi[0].combos[0].analog_one = vec![MulAnalogMod::new(&mut n)];

        let op = |key_idx_vec: &mut Vec<usize>| -> Result<(), String> {key_idx_vec.push(0); Ok(())};
        engine.dig_inputs_global_vec_manip(op);
        engine.ana_inputs_global_vec_manip(op);

        engine
    }
    
    pub fn to_play(&mut self) -> Result<Engine<Play>, String> {
        self.check_gut_vec_lengths()?;
        self.check_digitalref_invariants()?;

        Ok(Engine {
            _mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            dig_inputs: self.dig_inputs.clone(),
            analog_inputs: self.analog_inputs.clone(),
            i_delta_mode: self.i_delta_mode.clone(),
            x_delta_mode: self.x_delta_mode.clone(),
            max_pressed: self.max_pressed.clone(),
            min_pressed: self.min_pressed.clone(),
            radio_mode: self.radio_mode.clone(),
            holds_mode: self.holds_mode.clone(),
            holds: self.holds.clone(),
            trnsp_mode: self.trnsp_mode.clone(),
            trnsp_all: self.trnsp_all.clone(),
            analog_mode: self.analog_mode.clone(),
            analog_all: self.analog_all.clone(),
            guts: self.guts.clone(),
            v_multi: self.v_multi.clone(),
            f_multi: self.f_multi.clone(),
            c_multi: self.c_multi.clone(),
        })
    }
}
