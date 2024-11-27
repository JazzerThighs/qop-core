use nestify::nest;
use winit::keyboard::KeyCode;
use crate::qopedit::QopEdit;

nest! {
    #[repr(C)]*
    #[derive(Debug)]*
    pub struct QopPlay {
        key_codes: Vec<pub(crate) struct DigitalInputs {
            input: KeyCode,
            pressed: bool
        }>,
        plucks: Vec<pub(crate) struct Gut{
            gut_triggers: pub(crate) struct BtnTog{
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
            },
            pub(crate) index_out: usize,
            pub(crate) extra_out: f64,
            pub(crate) trnsp_gut: Vec<pub(crate) struct TrnspGut {
                pub(crate) triggers: Vec<usize>,
                pub(crate) i_delta: i64,
                pub(crate) x_delta: f64,
            }>,
            pub(crate) tp_i_mem: i64,
            pub(crate) tp_x_mem: f64,
        }>,
        pub(crate) gut_holds: pub(crate) struct HoldBtns {
            pub(crate) sustain: BtnTog,
            pub(crate) inv_sustain: BtnTog,
            pub(crate) sostenuto: BtnTog,
            pub(crate) inv_sostenuto: BtnTog,
        },
        pub(crate) valve_sets: Vec<pub(crate) struct IndvSet {
            pub(crate) buttons: Vec<pub(crate) struct DeltaTog {
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
                pub(crate) i_deltas: Vec<i64>,
                pub(crate) x_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<pub(crate) struct TrnspSet {
                    triggers: Vec<usize>,
                    i_deltas: Vec<i64>,
                    x_deltas: Vec<f64>,
                }>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
            }>,
            pub(crate) trnsp_all: Vec<TrnspSet>,
            pub(crate) tp_i_mem: Vec<i64>,
            pub(crate) tp_x_mem: Vec<f64>,
            pub(crate) holds: HoldBtns,
            pub(crate) max_pressed: Option<u8>,
            pub(crate) min_pressed: Option<u8>,
        }>,
        pub(crate) fret_sets: Vec<IndvSet>,
        pub(crate) radio_sets: Vec<IndvSet>,
        pub(crate) combo_sets: Vec<pub(crate) struct ComboSet {
            pub(crate) buttons: Vec<BtnTog>,
            pub(crate) combos: Vec<pub(crate) struct Combo {
                pub(crate) combo: Vec<bool>,
                pub(crate) i_deltas: Vec<i64>,
                pub(crate) x_deltas: Vec<f64>,
                pub(crate) trnsp_one: Vec<TrnspSet>,
                pub(crate) tp_i_mem: Vec<i64>,
                pub(crate) tp_x_mem: Vec<f64>,
            }>,
            pub(crate) holds: HoldBtns,
            pub(crate) trnsp_all: Vec<TrnspSet>,
            pub(crate) tp_i_mem: Vec<i64>,
            pub(crate) tp_x_mem: Vec<f64>,
        }>,
    }
}

impl From<&QopEdit> for QopPlay {
    fn from(qe: &QopEdit) -> Self {
        Self {
            key_codes: qe.key_codes.clone(),
            plucks: qe.guts.clone(),
            plk_holds: qe.gut_holds.clone(),
            valve_sets: qe.valve_sets.clone(),
            fret_sets: qe.fret_sets.clone(),
            radio_sets: qe.radio_sets.clone(),
            combo_sets: qe.combo_sets.clone(),
        }
    }
}