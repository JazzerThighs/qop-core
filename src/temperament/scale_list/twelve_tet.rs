use crate::temperament::scale_list::ETScale;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TWELVE_TET_SCALES: Vec<ETScale> = vec![
        ETScale {
            name: String::from("Acoustic"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Adonai Malakh"),
            notes: vec![0, 1, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Aeolian"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Aeolian b1"),
            notes: vec![0, 3, 4, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Aeolian Harmonic"),
            notes: vec![0, 3, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Aeolian Major"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Aeolian Pentatonic"),
            notes: vec![0, 2, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ahava Rabba"),
            origin: String::from("Jewish"),
            notes: vec![0, 1, 3, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ajam Shiram"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ake-Bono"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Algerian"),
            origin: String::from("Tunisia"),
            notes: vec![0, 2, 3, 6, 7, 8, 11, 12, 14, 15],
            ..Default::default()
        },
        ETScale {
            name: String::from("Algerian Octatonic"),
            origin: String::from("Tunisia"),
            notes: vec![0, 2, 3, 5, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Alhijaz"),
            origin: String::from("Arabia"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Altered Diminished"),
            notes: vec![0, 2, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Altered Lydian"),
            notes: vec![0, 2, 4, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Altered (or Altered Dominant)"),
            notes: vec![0, 1, 3, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Altered Pentatonic"),
            notes: vec![0, 1, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Altered Major Pentatonic"),
            notes: vec![0, 2, 4, 5, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ambassel"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ancient Chinese"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Anhemitonic Hexatonic"),
            notes: vec![0, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Arabic"),
            origin: String::from("Arabia"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ararai"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Arezzo Major Diatonic Hexachord"),
            notes: vec![0, 2, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ascending Minor"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Augmented"),
            notes: vec![0, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Augmented Pentatonic"),
            notes: vec![0, 3, 4, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Augmented Pentatonic 2"),
            notes: vec![0, 4, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Avaha or Ahava Rabba"),
            origin: String::from("Jewish"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bac"),
            origin: String::from("Vietnam"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Banshikicho"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 4, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bartok"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bati"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Chromatic"),
            notes: vec![0, 1, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Dorian"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Half-diminished"),
            notes: vec![0, 1, 3, 5, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Harmonic Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Locrian"),
            notes: vec![0, 1, 3, 5, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Major"),
            notes: vec![0, 2, 4, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Major Heptatonic"),
            notes: vec![0, 2, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Major Hexatonic"),
            notes: vec![0, 2, 4, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Melodic Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Minor"),
            notes: vec![0, 2, 3, 4, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Mixolydian"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop Natural Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Bebop (or Bebop Dominant)"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Belinese"),
            origin: String::from("Bali"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Dorian Hexatonic"),
            notes: vec![0, 1, 3, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Enneatonic"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Enneatonic 2"),
            notes: vec![0, 2, 3, 4, 5, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Heptatonic"),
            notes: vec![0, 2, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Heptatonic 2"),
            notes: vec![0, 3, 5, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Leading Tone"),
            notes: vec![0, 3, 5, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Major"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Minor"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Minor Maj7"),
            notes: vec![0, 3, 5, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Minor Pentatonic"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Mixed"),
            notes: vec![0, 3, 4, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Modified"),
            notes: vec![0, 2, 3, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Octatonic"),
            notes: vec![0, 2, 3, 5, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues (or Blues Hexatonic)"),
            notes: vec![0, 3, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Blues Phrygian"),
            notes: vec![0, 1, 3, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Buzurg"),
            notes: vec![0, 1, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Byzantine"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Center-Cluster PentaMirror"),
            notes: vec![0, 3, 4, 5, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chad Gadyo"),
            origin: String::from("Jewish"),
            notes: vec![0, 2, 3, 5, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chaio"),
            notes: vec![0, 2, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chin"),
            notes: vec![0, 3, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chinese"),
            origin: String::from("China"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chinese Eight-tone"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ching"),
            origin: String::from("China"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic"),
            notes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Diatonic Dorian"),
            notes: vec![0, 1, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Dorian"),
            notes: vec![0, 1, 2, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Dorian Inverse"),
            notes: vec![0, 3, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Hypodorian"),
            notes: vec![0, 2, 3, 4, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Hypolydian"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Hypolydian Inverse"),
            notes: vec![0, 1, 4, 5, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Hypophrygian Inverse"),
            notes: vec![0, 1, 2, 5, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Lydian"),
            notes: vec![0, 1, 4, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Lydian Inverse"),
            notes: vec![0, 1, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Mixolydian"),
            notes: vec![0, 1, 2, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Mixolydian 2"),
            notes: vec![0, 1, 2, 4, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Mixolydian Inverse"),
            notes: vec![0, 2, 5, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Permutated Diatonic Dorian"),
            notes: vec![0, 1, 2, 4, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Phrygian"),
            notes: vec![0, 3, 4, 5, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Chromatic Phrygian Inverse"),
            notes: vec![0, 1, 2, 4, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Cushak"),
            origin: String::from("Armenia"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Damian Emmanuel"),
            notes: vec![0, 2, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dasrgah-e Mahur"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dastgah-e Rast Panjgah"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Diminished"),
            notes: vec![0, 2, 3, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Diminished Half-tone"),
            notes: vec![0, 1, 3, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Diminished Pentatonic"),
            notes: vec![0, 3, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dominant Pentatonic"),
            notes: vec![0, 2, 4, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian #4"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian Aeolian"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian b2"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian b2 b4"),
            notes: vec![0, 1, 3, 4, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian b2 Maj7"),
            notes: vec![0, 1, 3, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian b5"),
            notes: vec![0, 2, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian b9"),
            notes: vec![0, 1, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian b9 #11"),
            notes: vec![0, 1, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorian Pentatonic"),
            notes: vec![0, 2, 3, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Dorico Flamenco"),
            origin: String::from("Spain"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Double Harmonic"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Double Harmonic Minor"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Double Phrygian"),
            notes: vec![0, 1, 3, 5, 6, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Egyptian"),
            origin: String::from("Egypt"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Enigmatic"),
            notes_ascending: vec![0, 1, 4, 6, 8, 10, 11],
            notes_descending: vec![0, 1, 4, 5, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Enigmatic Minor"),
            notes: vec![0, 1, 3, 6, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Enigmatic Mixed"),
            notes: vec![0, 1, 4, 5, 6, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Eskimo Hexatonic"),
            origin: String::from("Alaska"),
            notes: vec![0, 2, 4, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Eskimo Hexatonic 2"),
            origin: String::from("Alaska"),
            notes: vec![0, 2, 4, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Esplá"),
            notes: vec![0, 1, 3, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ezel"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Flamenco"),
            notes: vec![0, 1, 3, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Freygish or Fraigish"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Full Minor All Flats"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Geez"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Genus Chromaticum"),
            notes: vec![0, 1, 3, 4, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Genus Diatonicum"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Genus Diatonicum Veterum Correctum"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Genus Secundum"),
            notes: vec![0, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Genus Tertium"),
            notes: vec![0, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ghana Heptatonic"),
            origin: String::from("Ghana"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ghana Pentatonic"),
            origin: String::from("Ghana"),
            notes: vec![0, 2, 3, 5, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ghana Pentatonic 2"),
            origin: String::from("Ghana"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gong"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Greek Arkaik"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 5, 6],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 1"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 2"),
            notes: vec![0, 2, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 3"),
            notes: vec![0, 1, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 4"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 5"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 6"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 7"),
            notes: vec![0, 2, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gregorian 8"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gu"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gu Xian"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gypsy"),
            origin: String::from("Hungary"),
            notes: vec![0, 2, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gypsy 2"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 5, 6, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gypsy Hexatonic"),
            origin: String::from("Hungarian"),
            notes: vec![0, 1, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gypsy Hexatonic Inverse"),
            origin: String::from("Hungary"),
            notes: vec![0, 3, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gypsy Inverse"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Gypsy Minor"),
            origin: String::from("Hungary"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Half Diminished"),
            notes: vec![0, 2, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hamel"),
            notes: vec![0, 1, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Han-Kumoi"),
            notes: vec![0, 2, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Major"),
            notes: vec![0, 2, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Major 2"),
            notes: vec![0, 2, 4, 5, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Major Inverse"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Minor #4"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Minor b5"),
            notes: vec![0, 2, 3, 5, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Minor Inverse"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Harmonic Neapolitan Minor"),
            origin: String::from("Italy"),
            notes: vec![0, 1, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hawaiian"),
            origin: String::from("Hawaii"),
            notes: vec![0, 2, 3, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hexatonic"),
            notes: vec![0, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hijaz Kar"),
            origin: String::from("Arabia"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hijaz Major"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 5, 6, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hindi b2 b3 b7"),
            notes: vec![0, 1, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hindu"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hirajoshi"),
            origin: String::from("Japan"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hitzaz or Hijaz"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hitzazkiar (or Hijazskiar)"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Honkoshi"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 3, 5, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hon-Kumoi-Joshi"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Houseini"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 4, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Houzam (or Huzam)"),
            origin: String::from("Greece"),
            notes: vec![0, 3, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Humayun"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Folk"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Gypsy"),
            origin: String::from("Hungary"),
            notes: vec![0, 2, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Major"),
            origin: String::from("Hungary"),
            notes: vec![0, 3, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Major Inverse"),
            origin: String::from("Hungary"),
            notes: vec![0, 2, 3, 5, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Minor"),
            origin: String::from("Hungary"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Minor b2"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hungarian Minor Inverse"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hybrid Pentatonic"),
            notes: vec![0, 3, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Hyojo"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ian Iwato"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ichilkotsucho (or Ishikosucho)"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("In"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Insen"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Insen Pentatonic"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Inverted Augmented"),
            notes: vec![0, 1, 4, 5, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ioanian Augmented"),
            notes: vec![0, 2, 4, 5, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ionian #2"),
            notes: vec![0, 3, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ionian #5"),
            notes: vec![0, 2, 4, 5, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ionian Augmented #2"),
            notes: vec![0, 3, 4, 5, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ionian Augmented b9"),
            notes: vec![0, 1, 4, 5, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ionian b5"),
            notes: vec![0, 2, 4, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ionian Pentatonic"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Istrian"),
            origin: String::from("Croatia"),
            notes: vec![0, 1, 3, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Iwato"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Javanese"),
            origin: String::from("Java"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Jazz Minor Inverse"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Jeths"),
            notes: vec![0, 2, 3, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Jia Zhong"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Jiao"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Jin-Yu (or Quin-Yu)"),
            origin: String::from("China"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Jog"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("John Found's Mantra Of Will"),
            notes: vec![0, 1, 4, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kaffa"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kartzihiar"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kata-Kumoi"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kiourdi"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 5, 6, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kokin-Choshi"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kubilai"),
            origin: String::from("Mongolia"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kumoi"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kung"),
            notes: vec![0, 2, 4, 6, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Kyemyonjo"),
            notes: vec![0, 3, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lai Noi"),
            origin: String::from("Laos"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lai Po Sai"),
            origin: String::from("Laos"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lai Soutsanaen"),
            origin: String::from("Laos"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lai Yai"),
            origin: String::from("Laos"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Leading Whole-Tone"),
            notes: vec![0, 2, 4, 6, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Leading Whole-Tone Inverse"),
            notes: vec![0, 1, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("LG Octatonic"),
            notes: vec![0, 1, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian"),
            notes: vec![0, 1, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian #6"),
            notes: vec![0, 1, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian b4"),
            notes: vec![0, 1, 3, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian bb3 bb7"),
            notes: vec![0, 1, 2, 5, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian bb7"),
            notes: vec![0, 1, 3, 5, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian Dominant"),
            notes: vec![0, 1, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian Maj7"),
            notes: vec![0, 1, 3, 5, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian Natural 2"),
            notes: vec![0, 2, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian Natural Maj6"),
            notes: vec![0, 1, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Locrian Pentatonic"),
            notes: vec![0, 3, 4, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian"),
            notes: vec![0, 2, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian #2"),
            notes: vec![0, 3, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian #2 #6"),
            notes: vec![0, 3, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian #2 Hexatonic"),
            notes: vec![0, 3, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian #5"),
            notes: vec![0, 2, 4, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian #6"),
            notes: vec![0, 2, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Augmented"),
            notes: vec![0, 2, 4, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Augmented #2"),
            notes: vec![0, 3, 4, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Augmented #3"),
            notes: vec![0, 2, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Augmented Dominant"),
            notes: vec![0, 2, 4, 6, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian b3"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian b7"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Diminished"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Dominant"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Dominant b6"),
            notes: vec![0, 2, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Hexatonic"),
            notes: vec![0, 2, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Minor"),
            notes: vec![0, 2, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Mixolydian"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Lydian Pentatonic"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Madenda Modern"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Magen Abot"),
            origin: String::from("Jewish"),
            notes: vec![0, 1, 3, 4, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Augmented"),
            notes: vec![0, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Gypsy"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Inverse"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Locrian"),
            notes: vec![0, 2, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Minor"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Minor Mixed"),
            notes: vec![0, 2, 3, 4, 5, 7, 8, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Mixolydian"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Pentatonic"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Pentatonic b2"),
            notes: vec![0, 1, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Pentatonic b2 b5"),
            notes: vec![0, 1, 4, 6, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Pentatonic b3"),
            notes: vec![0, 1, 3, 6, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Pentatonic b6"),
            notes: vec![0, 2, 4, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major Pentatonic b7 #9"),
            notes: vec![0, 3, 4, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major-Dorian Mixed"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Major-Lydian Mixed"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Makam Lami"),
            origin: String::from("Jewish"),
            notes: vec![0, 1, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Man Gong"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Man Jue"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Athar Kurd"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Bayat-e-Esfahan"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Cargah"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Farahfaza"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Hedjaz"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Hicaz"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Hijaz"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Hijaz Kar"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Hijaz-Nahawand"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Hisar"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Humayun"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Huzzam"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 4, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Karcigar"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Kurd"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Nahawand"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Nahawand Murassah"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Nakriz"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Nawa Athar"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Saba Zamzam"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 4, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Shadd'araban"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Shahnaz (or Shahnaz Kurdi)"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Shawq Afza"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Shawq Awir"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Sultani Yakah"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Suzidil"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Tarzanuyn"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Ussak"),
            origin: String::from("Iraq"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Zanjaran"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Maqam Zengule"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Bhairavi That"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Bhavapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Cakravka"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Calanata"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Carukesi"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Citrambari"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Dharmavati"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Dhatuvardhani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Dhavalambari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Dhenuka"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Divyamani"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Gamanasrama"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Ganamurti"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Gangeyabhusani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Gaurimanohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Gavambodhi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Gayakapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Hanumatodi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Harikamboji"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Hatakambari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Hemavati"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Jalarnava"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Jhalavarli"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Jhankaradhvani"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Jyotisvarupini"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kamavardhani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kanakangi"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kantamani"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kharaharapriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kiravani"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kokilapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Kosalam"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Latangi"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Manavati"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Mayamalavagowla"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Mecakalyani"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Naganandini"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Namanarayani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Nasikabhusani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Natabhairavi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Natakapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Navanitam"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Nitimati"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Pavani"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Ragavardhani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Raghupriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Ramapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Rasikapriya"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Ratnangi"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Rupavati"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Sadvidhmargini"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Salaga"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Sanmukhapriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Sarasangi"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Senavati"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Shankarabharanam"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Shubhapanturavali"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Sulini"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Sumhendramadhyama"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Suryakanta"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Suvarnangi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Syamalangi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Tenarupi"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Vacaspati"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Vagadhisvari"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Vakulabharanam"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Varunapriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Venaspati"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Visvambhari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mela Yagapriya"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Melodic Major"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Melodic Minor"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Melodic Minor #4"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Melog Selisir"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 1st Mode"),
            notes: vec![0, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 2nd Mode"),
            notes: vec![0, 1, 4, 5, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 2nd Mode"),
            notes: vec![0, 1, 3, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 2nd Mode Inverse"),
            notes: vec![0, 2, 3, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 2nd Mode Truncated"),
            notes: vec![0, 1, 3, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 3rd Mode"),
            notes: vec![0, 2, 3, 4, 6, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 3rd Mode Inverse"),
            notes: vec![0, 1, 3, 4, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 4th Mode"),
            notes: vec![0, 1, 2, 5, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 4th Mode Inverse"),
            notes: vec![0, 3, 4, 5, 6, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 5th Mode"),
            notes: vec![0, 1, 5, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 5th Mode Inverse"),
            notes: vec![0, 4, 5, 6, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 6th Mode"),
            notes: vec![0, 2, 4, 5, 6, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 6th Mode Inverse"),
            notes: vec![0, 1, 2, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 7th Mode"),
            notes: vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen 7th Mode Inverse"),
            notes: vec![0, 2, 3, 4, 5, 6, 8, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Messiaen Truncated 3rd Mode Inv."),
            notes: vec![0, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor 6th Added"),
            notes: vec![0, 3, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor b5"),
            notes: vec![0, 2, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor Gypsy Inverse"),
            origin: String::from("Hungary"),
            notes: vec![0, 1, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor Hexatonic"),
            notes: vec![0, 2, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor Locrian"),
            notes: vec![0, 2, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor Pentatonic"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor Pentatonic 7 b5"),
            notes: vec![0, 3, 5, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minor Pentatonic with Leading Tones"),
            notes: vec![0, 2, 3, 4, 5, 6, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Minyo"),
            origin: String::from("Japan"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mischung 1"),
            origin: String::from("Germany"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mischung 2"),
            origin: String::from("Germany"),
            notes: vec![0, 2, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mischung 3"),
            origin: String::from("Germany"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mischung 5"),
            origin: String::from("Germany"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mischung 6"),
            origin: String::from("Germany"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Misheberekh"),
            origin: String::from("Jewish"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian #1"),
            notes: vec![0, 1, 3, 4, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian #4"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian Augmented"),
            notes: vec![0, 2, 4, 5, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian Augmented Maj9"),
            notes: vec![0, 1, 4, 5, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian b2 (or Mixolydian b9)"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian b5"),
            notes: vec![0, 2, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian b6 (or Mixolydian b13)"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian Dorian"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian Hexatonic"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian Hexatonic 2"),
            notes: vec![0, 2, 4, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mixolydian Pentatonic"),
            notes: vec![0, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Miyakobushi"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Modus Conjunctus"),
            notes: vec![0, 2, 3, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Mohammedan"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Moorish Phrygian"),
            origin: String::from("Spain"),
            notes: vec![0, 1, 3, 4, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Naka Zora"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Nam"),
            origin: String::from("Vietnam"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Nando-Kyemyonjo"),
            origin: String::from("Korea"),
            notes: vec![0, 2, 3, 5, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Natural Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Neapolitan Major (or Neapolitan)"),
            origin: String::from("Italy"),
            notes: vec![0, 1, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Neapolitan Major b4"),
            origin: String::from("Italy"),
            notes: vec![0, 1, 3, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Neapolitan Major b5"),
            notes: vec![0, 1, 3, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Neapolitan Minor"),
            origin: String::from("Italy"),
            notes: vec![0, 1, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Neseveri"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 3, 6, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Niagari"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Niavent"),
            origin: String::from("Egypt"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Niaventi Minor"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Nigriz"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Noh"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Nohkan"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Nonatonic 2"),
            notes: vec![0, 1, 3, 4, 5, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Octatonic"),
            notes: vec![0, 2, 3, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Olympos Enharmonic"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Oriental"),
            origin: String::from("China"),
            notes: vec![0, 1, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Oriental 2"),
            origin: String::from("China"),
            notes: vec![0, 1, 4, 5, 6, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Oshikicho"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ousak"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Overtone"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("P'Yongjo"),
            origin: String::from("Korea"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("P'yongjo-kyemyonjo"),
            origin: String::from("Korea"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Peiraiotikos"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pelo Degung Modern"),
            origin: String::from("Bali"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pelog"),
            origin: String::from("Bali"),
            notes: vec![0, 2, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pelog Pentatonic"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pentatonic Whole-Tone"),
            notes: vec![0, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Periaiotikos Minor"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Persian"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Peruvian Major"),
            origin: String::from("Peru"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Peruvian Minor"),
            origin: String::from("Peru"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Peruvian Major Pentatonic"),
            origin: String::from("Peru"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Peruvian Minor Pentatonic"),
            origin: String::from("Peru"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Petrushka chord"),
            notes: vec![0, 1, 4, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Aeolian b4"),
            notes: vec![0, 1, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Aeolian Mixed"),
            notes: vec![0, 1, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian b4"),
            notes: vec![0, 1, 3, 4, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian b4 Maj7"),
            notes: vec![0, 1, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Dominant"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Hexatonic"),
            notes: vec![0, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Locrian"),
            notes: vec![0, 1, 3, 5, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Major"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Mixolydian"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Natural 6"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Phrygian Pentatonic"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ping"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pireotikos"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pomeroy"),
            notes: vec![0, 1, 3, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Prokofiev"),
            origin: String::from("Russia"),
            notes: vec![0, 1, 3, 5, 6, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Prometheus"),
            notes: vec![0, 2, 4, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Prometheus Liszt"),
            notes: vec![0, 1, 4, 5, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Prometheus Neapolitan"),
            origin: String::from("Italy"),
            notes: vec![0, 1, 4, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Puravi b6"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pyeong Jo"),
            origin: String::from("Korea"),
            notes: vec![0, 2, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pygmy"),
            notes: vec![0, 2, 3, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Pyramid Hexatonic"),
            notes: vec![0, 2, 3, 5, 6, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Qing Shang"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Quan Ming"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Quin-Yu (or Jin-Yu)"),
            origin: String::from("China"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Abheri"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Abhogi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Adana"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ahir Bhairav"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ahira Lalita"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ahiri Todi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Aivarati"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Alhaiya Bilaval"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Amarasenapriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ambika"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Amrtavarshini"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Anandabhairavi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Andhali"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Andolika"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Arabhi ascending"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Arabhi descending"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Arunajualita"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Asavari That"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Audav Tukhari"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bahar"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bahudari"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bairagi (or Baira)"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bairari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Balahamsa"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Barbara"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Basant"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bauli"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Begeshri"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Begeshri 2"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhairav That"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhairavi ascending"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhairavi descending"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhairubahar"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhankar"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhanumati"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhatiyar"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhavani"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhimpalasi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhinna Pancama"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhinna Shadj"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhinnasadjam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhogachayanata"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhopali"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhunumanjari"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhup"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhupalam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhupeshwari"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bhusavati (or Bhusavali)"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bibhas"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bihag"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bihagara"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bilahari ascending"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bilahari descending"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bilashkhani Todi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bindumalini"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Bowli"),
            origin: String::from("India"),
            notes_ascending: vec![0, 1, 4, 7, 8],
            notes_descending: vec![0, 1, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Brindabani (or Brindabani Serang)"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Budhamanohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Camara"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chakravakam"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chandrajyoti"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chandrakauns Kafi"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chandrakauns Kiravani"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chandrakauns Modern"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chaturangini"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chaturangini 2"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chaya Todi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chaya Vati"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chayanat"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chayanata"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chinthamani"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Chitthakarshini"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Cudamani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Darbar"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Desh"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Desh Malhar"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Deshgaur"),
            origin: String::from("India"),
            notes: vec![0, 1, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Deshi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Deshi 2"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Deshi 3"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Deskar"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Desya Khamas"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Desya Todi"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devagandhari"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devakriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devamani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devamanohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devarangini 2"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devaranjani (or  Devaranji)"),
            origin: String::from("India"),
            notes: vec![0, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Devarashtra"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhaanyasi ascending"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhani"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhanyasi descending"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhauta Pancama"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhavalangam"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhavalashri"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhipaka"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dhunibinnashadjam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dipak"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Disisimharavam"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dumyaraga"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Dvigandharabushini"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gamakakriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gamakasamantam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gambhiranata"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ganasamavarali"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ganavaridhi"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gandharavam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gangatarangini"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gaud Sarang"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gaula"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gaulipantu"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gauri"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gauri Velavali"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gaurikriya"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Geyahejjajji"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ghandarva"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ghanta"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ghantana"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Girija"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Girvani"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gopikacasantam"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gopikatilaka"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gopriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gorakh"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gowla"),
            origin: String::from("India"),
            notes_ascending: vec![0, 1, 5, 7, 11],
            notes_descending: vec![0, 1, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Guhamanohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gunkali"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Gurjari Todi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamir Kalyani"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamsadhvani"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamsadhvani 2"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamsagiri"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamsalata"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamsanada"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hamsanarayami"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hansanandi"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hari Nata"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Harikauns"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Harini"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Haripriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hejjajji"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hindol"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hindolam"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Hindolita"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Huseni"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Indupriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jaganmohanam"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jaganmohini ascending"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jait Kalyan"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Janasammodini"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Janjuti"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jaunpuri"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jayakauns"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jayamanohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jeyasuddhamalavi"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jhankara Bhramavi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jinavali"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jingla"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jivantika"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jivantini"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jog"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jogiya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jotismatti"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Jyoty"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kafi That"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kaihavasi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kaishikiranjani (or Kaushiranjani)"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalagada"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalahamsa"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalakanthi"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalakanthi 2"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalamurti"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalavati"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalingada"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalyan That"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalyana Vasantha"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalyani"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kalyani Keseri"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kamalamanohari"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kamalamanohari 2"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kambhoji"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kambodhi ascending"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kambodhi descending"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kamud"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kanakambari"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kanara"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kannadabangala"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kashyapi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kasiramakryia"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kaushikdhvani"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kedar"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kedaram"),
            origin: String::from("India"),
            notes_ascending: vec![0, 4, 5, 7, 11],
            notes_descending: vec![0, 2, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Keradam ascending"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Keseri"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Khamach"),
            origin: String::from("India"),
            notes_ascending: vec![0, 4, 5, 7, 9, 10, 11],
            notes_descending: vec![0, 2, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Khamaj"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Khamaj That"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Khamaji Durga"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Khamas"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Khambhavati"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kharapriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kiranavali"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kirvani"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kokil Pancham"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kokila"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kokilaravam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kshanika"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kuksumakaram"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kumarapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kumudki"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kumurdaki"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kunakri"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kunbhini"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kuntala"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Kuntvarali (or Kuntalavarali)"),
            origin: String::from("India"),
            notes: vec![0, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Lalit"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Lalita"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Lalita Bhairav"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Lalita Panchami"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Lasaki"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Latantapriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Latika"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhava Manohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhmat Sarang"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhukauns"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhuranjani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhuri"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhuvanti"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Madhyamavati"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malahari"),
            origin: String::from("India"),
            notes_ascending: vec![0, 1, 5, 7, 8],
            notes_descending: vec![0, 1, 4, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malahari ascending"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malarani"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malashri"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malavastri"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malayamarutam"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malgunji"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malini"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Malkauns"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mamata"),
            origin: String::from("India"),
            notes: vec![0, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manaranjani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manaranjani 2"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manavi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mand"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mandari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manirangu"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manji"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manohari"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Manoranjani"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Marga Hindola"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Marva"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Marwa Thaat"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Matha Kokila"),
            origin: String::from("India"),
            notes: vec![0, 2, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Megh (or Megh Malhar)"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Megharamji"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Megharanjani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Miam Ki Malhar"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mohanam"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mohanangi"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mruganandana"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Mukhari"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Multani"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Multani 2"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nabhomani"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nagabharanam"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nagagandhari"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nagaswaravali"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nalinakanti"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nandkauns"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Narmada"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nasamani"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nasikabhusani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nata"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Natabharanam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nattai"),
            origin: String::from("India"),
            notes_ascending: vec![0, 3, 4, 5, 7, 10, 11],
            notes_descending: vec![0, 3, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nattaikurinji"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Navamanohari"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nayaki"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nayaki Kanada"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Neelangi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Neroshta"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 11, 13],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nileshwari"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nisada"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Nishadi"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga None"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Padi"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Pahadi"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Palasi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Pancama"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Pantuvarali"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Paraj"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Paras (or Pharas or Paraju)"),
            origin: String::from("India"),
            notes_ascending: vec![0, 4, 5, 7, 8, 11],
            notes_descending: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Partivaran"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Patdip"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Phenadyuti"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Phenadyuti 2"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Pilu"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Pilu That"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Prabhati"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Pratapa"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Priyadharshini"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Puriya (or Puriya Kalyan)"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Puriya 2"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Puriya Dhanashri"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Purna Pancama"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Purnanalita"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Puruhutika"),
            origin: String::from("India"),
            notes: vec![0, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Purvaholica"),
            origin: String::from("India"),
            notes: vec![0, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Purvi"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Purvi Thaat"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Purvikalyani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Putrika"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Raga Pushpalithika"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ragamalini"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rageshri"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ramakri"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ramamahohari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ramamanohari 2"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ramdasi Malharq"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ramkali 2"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rangini"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ranjiani"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rasamanjari"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rasamanjari 2"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rasavali"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rasika Ranjani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rasranjani"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ratipriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ratnakanthi"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ravikriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Reejeshwari"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Regeshri (or Rageshwari)"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Reva (or Revagupti)"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ribhavari (or Revati)"),
            origin: String::from("India"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rishabapriya"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Ritigaula"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rudra Pancama"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Rukmangi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sahera"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sailadesakshi"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Salagavarali"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Salanganata"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Samanta"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Samudhra Priya"),
            origin: String::from("India"),
            notes: vec![0, 3, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sanjk Ka Hindol"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sankara (or Shankara)"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Santanamanjari"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sarasanana"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sarasvati"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Saravati"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Saugandhini"),
            origin: String::from("India"),
            notes: vec![0, 1, 6, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Saugandhunu"),
            origin: String::from("India"),
            notes: vec![0, 1, 6, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Saurashtra"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sauviram"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Saveri ascending"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Saveri descending"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Savethri"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Savitri"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Senagrani"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shailaja"),
            origin: String::from("India"),
            notes: vec![0, 3, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shilangi"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shobhavari"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shree ascending"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shree descending"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shreeranjani\n(or Shee Ranjani or Sriranjani)"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shri"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shri Kalyan"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shubravarni"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shuddh Kalyan"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Shyamalam"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Simharava"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Simhavahini"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Simmendramadhyamam"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sindhi-Bhairavi"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 3, 4, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sindhu Ramakriya"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sindhura"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sindhura Kafi"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Siva Kambhoji"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sohani"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sohini"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sohni"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sorati"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sowrashtram"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sri"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Srutiranjani"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sthavarajam"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Bangala"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Mukhari"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Pancama"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Ramakriya"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 6, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Saveri"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Simantini"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suddha Todi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suha Kanada"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suha Sughrai"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sunada Vinodini"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Suposhini"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Supradhipam"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sur Malhar"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Surati"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Surya"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Sutradhari"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Syamalam"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Takka"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Tanukirti"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Tarangini"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Tilang"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Tilang (or Bridabani Tilang)"),
            origin: String::from("India"),
            notes: vec![0, 4, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Tivravahini"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Trimurti"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Trishuli"),
            origin: String::from("India"),
            notes: vec![0, 3, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Udayaravicandrika"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Udhayaravi"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vagedeeshwari"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vaijayanti"),
            origin: String::from("India"),
            notes: vec![0, 2, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Valaji"),
            origin: String::from("India"),
            notes: vec![0, 4, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vamsavathi"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Varali"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 5, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Varamu"),
            origin: String::from("India"),
            notes: vec![0, 3, 5, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Varini"),
            origin: String::from("India"),
            notes: vec![0, 3, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vasanta"),
            origin: String::from("India"),
            notes_ascending: vec![0, 4, 5, 9, 11],
            notes_descending: vec![0, 1, 4, 5, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vasantha"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vativasanta"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vegavahini"),
            origin: String::from("India"),
            notes_ascending: vec![0, 4, 5, 7, 9, 10],
            notes_descending: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vegavahini descending"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Velavali"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vibhas"),
            origin: String::from("India"),
            notes: vec![0, 1, 4, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vijayanagari"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vijayasri"),
            origin: String::from("India"),
            notes: vec![0, 1, 2, 6, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vijayavasanta"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vilasini"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Viravasantham"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 5, 7, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vivardhini"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Viyogavarali"),
            origin: String::from("India"),
            notes: vec![0, 1, 3, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Vutari"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Yaduka Kambodi descending"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Yadukua Kambodhi ascending"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Yaman Kalyan"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Yamuna Kalyani"),
            origin: String::from("India"),
            notes: vec![0, 2, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Zilaf"),
            origin: String::from("India"),
            notes: vec![0, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Raga Zilla"),
            origin: String::from("India"),
            notes: vec![0, 2, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ragta Narayani"),
            origin: String::from("India"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Rahawi"),
            origin: String::from("Iraq"),
            notes: vec![0, 1, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Rast"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 4, 5, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ravel"),
            notes: vec![0, 1, 3, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Relative Blues"),
            notes: vec![0, 2, 3, 4, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ritusen"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ritzu"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ritzu Gagaku"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Rock 'n Roll"),
            notes: vec![0, 3, 4, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Romanian Bacovia"),
            origin: String::from("Romania"),
            notes: vec![0, 4, 5, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Romanian Major"),
            origin: String::from("Romania"),
            notes: vec![0, 1, 4, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Rui Bin"),
            origin: String::from("China"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ryo"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ryosen"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ryukyu"),
            origin: String::from("Japan"),
            notes: vec![0, 4, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sabach"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 4, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sabach Maj7"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 4, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sakura"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Scottish Hexatonic"),
            origin: String::from("Scotland"),
            notes: vec![0, 2, 4, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Scottish Pentatonic"),
            origin: String::from("Scotland"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Scriabin"),
            notes: vec![0, 1, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Segiah"),
            notes: vec![0, 2, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Se"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Semilocrian"),
            notes: vec![0, 2, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Semilocrian b4"),
            notes: vec![0, 2, 3, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sengiach (or Sengah)"),
            origin: String::from("Greece"),
            notes: vec![0, 3, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Shang"),
            origin: String::from("China"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Shang-Diao"),
            origin: String::from("China"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sho"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sho #2"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 3, 4, 6, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Shostakovich"),
            origin: String::from("Russia"),
            notes: vec![0, 1, 3, 4, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Soft Ascend"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Soft Descend"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 4, 6, 7],
            ..Default::default()
        },
        ETScale {
            name: String::from("Souzinak"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Spanish Heptatonic"),
            origin: String::from("Spain"),
            notes: vec![0, 3, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Spanish Octatonic"),
            notes: vec![0, 1, 3, 4, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Spanish (or Spanish Gypsy)"),
            origin: String::from("Spain"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Spanish Phrygian"),
            origin: String::from("Spain"),
            notes: vec![0, 1, 3, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Sultani Yakah"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Superlocrian"),
            notes: vec![0, 1, 3, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Superlocrian #6"),
            notes: vec![0, 1, 3, 4, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Superlocrian bb3"),
            notes: vec![0, 1, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Superlocrian bb6 bb7"),
            notes: vec![0, 1, 3, 4, 6, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Superlocrian Maj7"),
            notes: vec![0, 1, 3, 4, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Suspended Pentatonic"),
            notes: vec![0, 2, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Symmetrical Decatonic"),
            notes: vec![0, 1, 2, 4, 5, 6, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Symmetrical Nonatonic"),
            notes: vec![0, 1, 2, 4, 6, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Synthetic Mixture #5"),
            notes: vec![0, 2, 4, 6, 8, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Syrian Pentatonic"),
            origin: String::from("Syria"),
            notes: vec![0, 1, 4, 5, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tabahaniotiko"),
            origin: String::from("Greece"),
            notes: vec![0, 2, 4, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Taishikicho"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 4, 5, 6, 7, 9, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Takemitzu Tree 1"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 6, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Takemitzu Tree 2"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tcherepnin"),
            origin: String::from("Russia"),
            notes: vec![0, 1, 3, 4, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tcherepnin Major Pentatonic"),
            origin: String::from("Russia"),
            notes: vec![0, 2, 5, 7, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tezeta Major"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tezeta Minor"),
            origin: String::from("Ethiopia"),
            notes: vec![0, 1, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Todi b7"),
            notes: vec![0, 1, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tritone"),
            notes: vec![0, 1, 4, 6, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tsinganikos"),
            origin: String::from("Greece"),
            notes: vec![0, 1, 4, 5, 6, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Tunisian"),
            origin: String::from("Tunisia"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Two-semitone Tritone"),
            notes: vec![0, 1, 2, 6, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ujo"),
            origin: String::from("Korea"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ukrainian Minor"),
            origin: String::from("Ukraina"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ukranian Dorian"),
            origin: String::from("Ukraina"),
            notes: vec![0, 2, 3, 6, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ultralocrian"),
            notes: vec![0, 1, 3, 4, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ultralocrian bb3"),
            notes: vec![0, 1, 2, 4, 6, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Ultraphrygian"),
            notes: vec![0, 1, 3, 4, 7, 8, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Utility Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 10, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Van Der Host"),
            notes: vec![0, 1, 3, 5, 6, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Whole-Tone"),
            notes: vec![0, 2, 4, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Whole-Tone Diminished"),
            notes: vec![0, 2, 3, 5, 6, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Xin"),
            origin: String::from("China"),
            notes: vec![0, 2, 4, 5, 7, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yi Ze"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yishtabach"),
            origin: String::from("Jewish"),
            notes: vec![0, 1, 3, 5, 6, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yo"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yona Nuki Major"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 4, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yona Nuki Minor"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 3, 7, 8],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yosen"),
            origin: String::from("Japan"),
            notes: vec![0, 2, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yu"),
            origin: String::from("China"),
            notes: vec![0, 2, 3, 5, 7, 9, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Yu 2"),
            origin: String::from("China"),
            notes: vec![0, 3, 5, 7, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zangula"),
            origin: String::from("Nigeria"),
            notes: vec![0, 2, 3, 5, 6, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zhalibny Minor"),
            notes: vec![0, 2, 3, 5, 7, 8, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zheng"),
            origin: String::from("China"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zhi"),
            origin: String::from("China"),
            notes: vec![0, 2, 5, 7, 9],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zilof"),
            origin: String::from("Spain"),
            notes: vec![0, 1, 4, 5, 7, 8, 10],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zirafkend"),
            origin: String::from("Arabia"),
            notes: vec![0, 2, 3, 5, 7, 8, 9, 11],
            ..Default::default()
        },
        ETScale {
            name: String::from("Zokuso"),
            origin: String::from("Japan"),
            notes: vec![0, 1, 3, 5, 7, 8, 10],
            ..Default::default()
        }
    ];
}
