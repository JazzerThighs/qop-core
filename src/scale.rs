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
            scale_type: self.scale_type.clone(),
        }
    }
}



impl TemperamentType {
    fn name(&self, f: f64) -> String {
        match self {
            TemperamentType::EqualTemperament(o, d) => format!("{f} hz {d}-Tone Equal Temperament Scale"),
            TemperamentType::PrimeLimit(p) => format!("{f} hz {p}-Limit Intonation Scale"),
            TemperamentType::Arbitrary => format!("{f} hz Arbitrary Scale"),
            
        }
    }

    fn intervals(&self) -> Vec<f64> {
        todo!()
    }
}

impl Temperament<Edit> {
    pub(crate) fn new(n: Option<&Temperament>) -> Temperament<Edit> {
        let mut new_scale: Temperament<Edit> = if n.is_some() { 
            n.unwrap().clone()
        } else { 
            Temperament::default() 
        };
        match n.scale_type {
            TemperamentType::EqualTemperament(divisions_value, octave_value) => {
                if divisions_value > 0.0 {
                    n.tuning_hz * 2.0f64.powf((n.reference_note as f64) / (divisions_value as f64));
                }
            },
            _ => {
                match n.scale_type {
                    TemperamentType::Arbitrary => {},
                    _ => n.intervals = n.scale_type.intervals()
                }
                todo!() // populate scale with interval'd notes
            }
        };
        new_scale
    }
}
