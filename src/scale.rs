use crate::*;
use better_default::Default;
use duplicate::duplicate_item;

#[rustfmt::skip]
#[derive(Default)]
pub(crate) struct NewScaleParams {
                         name: Option<String>,
                         description: Option<String>,
                         scale_type: ScaleType,
    #[default(69usize)]  reference_note: usize,
    #[default(440.0f64)] tuning_hz: f64,
    #[default(12usize)]  octave_divisions: usize,
    #[default(-2i64)]    octave_label: i64,
    #[default(128usize)] note_amount: usize,
    #[default(["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"].iter().map(|i: &&str| i.to_string()).collect())]
                         note_class_set: Vec<String>,
}

impl Scale<Edit> {
    pub(crate) fn new(mut n: NewScaleParams) -> Scale<Edit> {
        let mut new_scale: Scale = Scale {
            name: if let Some(some_name) = n.name {
                some_name
            } else {
                String::default()
            },
            description: if let Some(some_description) = n.description {
                some_description
            } else {
                String::default()
            },
            scale_type: n.scale_type.clone(),
            reference_note: n.reference_note,
            tuning_hz: n.tuning_hz,
            octave_divisions: n.octave_divisions,
            note_class_set: n.note_class_set.clone(),
            notes: vec![Note::default(); n.note_amount],
            ..Default::default()
        };
        let mut new_scale_name: String = String::default();
        match n.scale_type {
            ScaleType::EqualTemperament => {
                if n.octave_divisions != 0 {
                    let mut note_class_idx: usize = 0;
                    for i in 0..n.note_amount {
                        let distance_from_ref: i64 = i as i64 - n.reference_note as i64;
                        new_scale.notes[i].frequency = n.tuning_hz
                            * 2.0f64.powf((distance_from_ref as f64) / (n.octave_divisions as f64));
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

                    new_scale_name = format!(
                        "{}hz {}-Tone Equal Temperament Scale",
                        n.tuning_hz, n.octave_divisions
                    );
                }
            }
            ScaleType::Arbitrary => new_scale.name = String::from("Arbitrary Scale"),
            ScaleType::JustIntonation => todo!(),
            ScaleType::Pythagorean5Limit => todo!(),
            ScaleType::Werckmeister => todo!(),
            ScaleType::Kirnberger => todo!(),
            ScaleType::Maqam => todo!(),
            ScaleType::Ndebele => todo!(),
            ScaleType::Gagaku => todo!(),
            ScaleType::Pelog => todo!(),
            ScaleType::Slendro => todo!(),
            ScaleType::Hijaz => todo!(),
            ScaleType::ShonaMbira => todo!(),
            ScaleType::BohlenPierce => todo!(),
        }
        if new_scale.name.is_empty() {
            new_scale.name = new_scale_name;
        }
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
            scale_type: self.scale_type.clone(),
            reference_note: self.reference_note.clone(),
            tuning_hz: self.tuning_hz.clone(),
            octave_divisions: self.octave_divisions.clone(),
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

impl Scale<Edit> {
    pub fn insert_note(&mut self, note_idx: usize) {
        if note_idx <= self.notes.len() {
            self.notes.insert(note_idx, Note::default());
        }
        self.refresh_note_nums();             
    }
}
