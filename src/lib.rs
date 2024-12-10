#![allow(dead_code)]

mod qopedit;
mod qopplay;
mod scale;

use std::marker::PhantomData;
use winit::keyboard::KeyCode;
use better_default::Default;
use serde::{Deserialize, Serialize};
use nestify::nest;

#[derive(Debug, Clone)]
pub(crate) struct Edit;
pub(crate) struct Play;

#[derive(Default, Clone)]
pub(crate) struct NewStuffPointers {
    // Engine Parts
    pub(crate) guts_len: usize,
    pub(crate) c_btn_len: usize,

    // Scale Parts
    new_reference_note: usize,
    new_tuning_hz: f64,
    new_octave_divisions: usize,
    new_note_class_set: Vec<String>,
    octave: usize,
    note_amount: usize,
}

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    #[derive(Default)]*
    pub struct Qop {
        pub(crate) engine: 
            pub(crate) struct Engine<Mode = Edit> {
                pub(crate) _engine_mode: PhantomData<Mode>,
                pub name: String,
                pub description: String,
                pub(crate) dig_inputs:  Vec<KeyCode>,
                pub(crate) gut_max_pressed: usize,
                pub(crate) gut_min_pressed: usize,
                pub(crate) gut_radio_mode: bool,
                pub(crate) gut_holds:
                    pub(crate) struct HoldBtns {
                        pub(crate) sustain: 
                            pub(crate) struct HoldTog {
                                pub name: String,
                                pub description: String,
                                togs: Vec<usize>,
                                pressed: bool
                            },
                        pub(crate) inv_sustain: HoldTog,
                        pub(crate) sostenuto: HoldTog,
                        pub(crate) inv_sostenuto: HoldTog,
                    },
                #[default(vec![Gut::default()])]
                pub(crate) guts: Vec<
                    pub(crate) struct Gut {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool,
                        pub(crate) index_out: usize,
                        pub(crate) extra_out: f64,
                        pub(crate) i_mem: i64,
                        pub(crate) x_mem: f64,
                        pub(crate) trnsp_gut: Vec<
                            pub(crate) struct Trnsp<T: Default, U: Default> {
                                pub(crate) triggers: Vec<usize>,
                                pub(crate) i_delta: T,
                                pub(crate) x_delta: U,
                            } ||<i64, f64>
                        >,
                        pub(crate) v_one: Vec<
                            pub(crate) struct VFSet<T: Default, U: Default> {
                                pub name: String,
                                pub description: String,
                                pub(crate) buttons: Vec<
                                    pub(crate) struct VFBtn<T: Default, U: Default> {
                                        pub name: String,
                                        pub description: String,
                                        pub(crate) togs: Vec<usize>,
                                        pub(crate) i_delta: T,
                                        pub(crate) x_delta: U,
                                        pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                        pub(crate) i_mem: T,
                                        pub(crate) x_mem: U,
                                    } ||<T, U>
                                >,
                                #[default(vec![false])]
                                pub(crate) pressed: Vec<bool>,
                                pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                                pub(crate) i_mem: T,
                                pub(crate) x_mem: U,
                                pub(crate) holds: HoldBtns,
                                pub(crate) max_pressed: usize,
                                pub(crate) min_pressed: usize,
                                pub(crate) radio_mode: bool,
                            } ||<i64, f64>
                        >,
                        pub(crate) f_one: Vec<VFSet<i64, f64>>,
                        pub(crate) c_one: Vec<
                            pub(crate) struct ComboSet<T: Default, U: Default> {
                                pub name: String,
                                pub description: String,
                                #[default(vec![ComboTog::default()])]
                                pub(crate) buttons: Vec<
                                    pub(crate) struct ComboTog {
                                        pub name: String,
                                        pub description: String,
                                        togs: Vec<usize>,
                                    }
                                >,
                                #[default(vec![false])]
                                pub(crate) pressed: Vec<bool>,
                                pub(crate) combos: Vec<
                                    pub(crate) struct Combo<T: Default, U: Default> {
                                        pub name: String,
                                        pub description: String,
                                        pub(crate) combo: Vec<bool>,
                                        pub(crate) i_delta: T,
                                        pub(crate) x_delta: U,
                                        pub(crate) trnsp_one: Vec<Trnsp<T, U>>,
                                        pub(crate) i_mem: T,
                                        pub(crate) x_mem: U,
                                    } ||<T, U>
                                >,
                                pub(crate) holds: HoldBtns,
                                pub(crate) trnsp_all: Vec<Trnsp<T, U>>,
                                pub(crate) i_mem: T,
                                pub(crate) x_mem: U,
                                pub(crate) max_pressed: usize,
                                pub(crate) min_pressed: usize,
                                pub(crate) radio_mode: bool,
                            }||<i64, f64>
                        >
                    }
                >,
                pub(crate) v_multi: Vec<VFSet<Vec<i64>, Vec<f64>>>,
                pub(crate) f_multi: Vec<VFSet<Vec<i64>, Vec<f64>>>,
                pub(crate) c_multi: Vec<ComboSet<Vec<i64>, Vec<f64>>>,
            },
        pub(crate) scale:
            pub(crate) struct Scale {
                pub name: String,
                pub description: String,
                pub(crate) scale_type: 
                    pub(crate) enum ScaleType {
                        Arbitrary,
                        #[default]
                        EqualTemperament,
                        JustIntonation,
                        Pythagorean5Limit,
                        Werckmeister,
                        Kirnberger,
                        Maqam,
                        Ndebele,
                        Gagaku,
                        Pelog,
                        Slendro,
                        Hijaz,
                        ShonaMbira,
                        BohlenPierce,
                    },
                #[default(69)]
                pub(crate) reference_note: usize,
                #[default(440.0f64)]
                pub(crate) tuning_hz: f64,
                #[default(12)]
                pub(crate) octave_divisions: usize,
                #[default(["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"].iter().map(|i: &&str| i.to_string()).collect())]
                pub(crate) note_class_set: Vec<String>,
                pub(crate) notes: Vec<
                    pub(crate) struct Note {
                        pub name: String,
                        pub description: String,
                        pub frequency: f64,
                        pub color: String,
                    }
                >,
            }
    }
}
