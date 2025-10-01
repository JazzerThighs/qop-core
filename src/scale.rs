pub mod intervals;
pub mod scale_list;

use crate::*;
use duplicate::duplicate_item;

impl Temperament<Edit> {
    pub fn to_play(&self) -> Temperament<Play> {
        Temperament {
            _mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            reference_note: self.reference_note.clone(),
            tuning_hz: self.tuning_hz.clone(),
            note_class_set: self.note_class_set.clone(),
            octave_scalar_factor: self.octave_scalar_factor.clone(),
            intervals: self.intervals.clone(),
            octave_label: self.octave_label.clone(),
            temperament_type: self.temperament_type.clone(),
        }
    }
}

impl TemperamentType {
    fn name(&self, f: f64) -> String {
        match self {
            TemperamentType::EqualTemperament(d, _o) => format!("{f} hz {d}-Tone Equal Temperament Scale"),
            TemperamentType::PrimeLimit(p) => format!("{f} hz {p}-Limit Intonation Scale"),
            TemperamentType::Arbitrary => format!("{f} hz Arbitrary Scale"), 
        }
    }

    fn intervals(&self) -> Option<Vec<f64>> {
        match self {
            TemperamentType::EqualTemperament(_d, _o) => todo!(),
            TemperamentType::PrimeLimit(_p) => todo!(),
            TemperamentType::Arbitrary => None,
        }
    }
}

impl Temperament<Edit> {
    pub(crate) fn new(
        name: Option<String>,
        description: Option<String>,
        temperament_type: Option<TemperamentType>,
        reference_note: Option<usize>,
        tuning_hz: Option<f64>,
        octave_label: Option<i64>,
        octave_scalar_factor: Option<f64>,
        note_class_set: Option<Vec<String>>,
        intervals: Option<Vec<f64>>,
    ) -> Temperament<Edit> {
        let mut n: Temperament<Edit> = Temperament {
            _mode: PhantomData,
            name: name.unwrap_or_default(),
            description: description.unwrap_or_default(),
            temperament_type: temperament_type.unwrap_or_default(),
            reference_note: reference_note.unwrap_or_default(),
            tuning_hz: tuning_hz.unwrap_or_default(),
            octave_label: octave_label.unwrap_or_default(),
            octave_scalar_factor: octave_scalar_factor.unwrap_or_default(),
            note_class_set: note_class_set.unwrap_or_default(),
            intervals: intervals.unwrap_or_default(),
        };
        
        // based on what is present in the input, build the other params from that
        // check for list of parameters in order of precedence

        if let Some(interval_list) = n.temperament_type.intervals() {
            n.intervals = interval_list;
        }

        n
    }
}
