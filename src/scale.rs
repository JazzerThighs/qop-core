pub mod intervals;

use crate::*;
use better_default::Default;
use nestify::nest;
use duplicate::duplicate_item;

nest! {
    #[derive(Default)]*
    pub(crate) struct NewScaleParams {
        name: Option<String>,
        description: Option<String>,
        scale_type: 
            pub enum ScaleType {
                #[default(0: 12usize)]
                EqualTemperament(usize),
                PrimeLimit(usize),
                Arbitrary,
            },
        #[default(69usize)]  reference_note: usize,
        #[default(440.0f64)] tuning_hz: f64,
                             interval_ratios: Option<Vec<f64>>,
        #[default(-2i64)]    octave_label: i64,
        #[default(128usize)] note_amount: usize,
        #[default(["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"].iter().map(|i: &&str| i.to_string()).collect())]
                             note_class_set: Vec<String>,
    }
}

impl ScaleType {
    fn name(&self, f: f64) -> String {
        match self {
            ScaleType::EqualTemperament(d) => format!("{f} hz {d}-Tone Equal Temperament Scale"),
            ScaleType::PrimeLimit(p) => format!("{f} hz {p}-Limit Intonation Scale"),
            ScaleType::Arbitrary => format!("{f} hz Arbitrary Scale"),
            
        }
    }

    fn intervals(&self) -> Option<Vec<f64>> {
        Some(vec![])
    }
}

impl Scale<Edit> {
    pub(crate) fn new(mut n: NewScaleParams) -> Scale<Edit> {
        let mut new_scale: Scale<Edit> = Scale {
            _scale_mode: PhantomData,
            name: if let Some(some_name) = n.name.clone() {
                some_name
            } else {
                n.scale_type.name(n.tuning_hz)
            },
            description: if let Some(some_description) = n.description.clone() {
                some_description
            } else {
                String::default()
            },
            reference_note: n.reference_note,
            tuning_hz: n.tuning_hz,
            note_class_set: n.note_class_set.clone(),
            notes: vec![Note::default(); n.note_amount],
        };
        match n.scale_type {
            ScaleType::EqualTemperament(d) => {
                if d != 0 {
                    let mut note_class_idx: usize = 0;
                    for i in 0..n.note_amount {
                        let distance_from_ref: i64 = i as i64 - n.reference_note as i64;
                        new_scale.notes[i].frequency =
                            n.tuning_hz * 2.0f64.powf((distance_from_ref as f64) / (d as f64));
                        new_scale.notes[i].note_num = i;
            
                        if !n.note_class_set.is_empty() {
                            new_scale.notes[i].name =
                                format!("{}{}", n.note_class_set[note_class_idx], n.octave_label);
                            note_class_idx += 1;
                            if note_class_idx == n.note_class_set.len() {
                                note_class_idx = 0;
                                n.octave_label += 1;
                            }
                        }
                    }
                }
            },
            _ => {
                match n.scale_type {
                    ScaleType::Arbitrary => {},
                    _ => n.interval_ratios = n.scale_type.intervals()
                }
                todo!() // populate scale with interval'd notes
            }
        };
        new_scale
    }

    pub(crate) fn refresh_note_nums(&mut self) {
        for (i, note) in self.notes.iter_mut().enumerate() {
            note.note_num = i;
        }
    }
    pub(crate) fn swap_notes(&mut self, n1: usize, n2: usize) {
        if n1 < self.notes.len() && n2 < self.notes.len() {
            self.notes.swap(n1, n2);
            self.refresh_note_nums();
        }
    }

    pub fn to_play(&self) -> Scale<Play> {
        Scale {
            _scale_mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            reference_note: self.reference_note.clone(),
            tuning_hz: self.tuning_hz.clone(),
            note_class_set: self.note_class_set.clone(),
            notes: self.notes.clone(),
        }
    }
}

#[duplicate_item(
    change_note_param         new_note_param    param_type note_param;
    [change_note_name]        [new_name]        [String]   [name];
    [change_note_description] [new_description] [String]   [description];
    [change_note_color]       [new_color]       [String]   [color];
    [change_note_frequency]   [new_frequency]   [f64]      [frequency];
)]
impl Scale<Edit> {
    pub(crate) fn change_note_param(&mut self, n_idx: usize, new_note_param: param_type) {
        if n_idx < self.notes.len() {
            self.notes[n_idx].note_param = new_note_param;
        }
    }
}

#[duplicate_item( 
    insertremove_note insertremove_idx;
    [insert_note]     [insert(n_idx, Note::default())];
    [remove_note]     [remove(n_idx)];
)]
impl Scale<Edit> {
    pub fn insertremove_note(&mut self, n_idx: usize) {
        if n_idx <= self.notes.len() {
            self.notes.insertremove_idx;
        }
        self.refresh_note_nums();             
    }
}
