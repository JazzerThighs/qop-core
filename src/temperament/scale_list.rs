pub mod twelve_tet;
pub mod unequal_temperament;

#[derive(Default)]

pub struct ETScale {
    pub name: String,
    pub origin: String,
    pub notes: Vec<usize>,
    pub notes_ascending: Vec<usize>,
    pub notes_descending: Vec<usize>,
}

#[derive(Default)]

pub struct UETScale {
    pub name: String,
    pub origin: String,
    pub notes: Vec<f64>,
    pub notes_ascending: Vec<f64>,
    pub notes_descending: Vec<f64>,
}