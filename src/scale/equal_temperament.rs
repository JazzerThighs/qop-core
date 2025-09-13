use crate::scale::ListedScale;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ET_SCALES: Vec<ListedScale> = vec![
        ListedScale {
            name: String::from("Acoustic"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Adonai Malakh"),
            intervals: Some(vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Aeolian"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Aeolian b1"),
            intervals: Some(vec![3.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Aeolian Harmonic"),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Aeolian Major"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Aeolian Pentatonic"),
            intervals: Some(vec![2.0, 1.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ahava Rabba"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ajam Shiram"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ake-Bono"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Algerian"),
            origin: Some(String::from("Tunisia")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0, 12.0, 14.0, 15.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Algerian Octatonic"),
            origin: Some(String::from("Tunisia")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Alhijaz"),
            origin: Some(String::from("Arabia")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Altered Diminished"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Altered Lydian"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Altered (or Altered Dominant)"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Altered Pentatonic"),
            intervals: Some(vec![1.0, 4.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Altered Major Pentatonic"),
            intervals: Some(vec![2.0, 2.0, 1.0, 3.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ambassel"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ancient Chinese"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Anhemitonic Hexatonic"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Arabic"),
            origin: Some(String::from("Arabia")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ararai"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Arezzo Major Diatonic Hexachord"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ascending Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Augmented"),
            intervals: Some(vec![3.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Augmented Pentatonic"),
            intervals: Some(vec![3.0, 1.0, 3.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Augmented Pentatonic 2"),
            intervals: Some(vec![4.0, 2.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Avaha or Ahava Rabba"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bac"),
            origin: Some(String::from("Vietnam")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Banshikicho"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bartok"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bati"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Chromatic"),
            intervals: Some(vec![1.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Dorian"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Half-diminished"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Harmonic Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Locrian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Major"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Major Heptatonic"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Major Hexatonic"),
            intervals: Some(vec![2.0, 2.0, 3.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Melodic Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Minor"),
            intervals: Some(vec![2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Mixolydian"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop Natural Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Bebop (or Bebop Dominant)"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Belinese"),
            origin: Some(String::from("Bali")),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Dorian Hexatonic"),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Enneatonic"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Enneatonic 2"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Heptatonic"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Heptatonic 2"),
            intervals: Some(vec![3.0, 2.0, 1.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Leading Tone"),
            intervals: Some(vec![3.0, 2.0, 1.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Major"),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Minor"),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Minor Maj7"),
            intervals: Some(vec![3.0, 2.0, 1.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Minor Pentatonic"),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Mixed"),
            intervals: Some(vec![3.0, 1.0, 1.0, 1.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Modified"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Octatonic"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues (or Blues Hexatonic)"),
            intervals: Some(vec![3.0, 2.0, 1.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Blues Phrygian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Buzurg"),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Byzantine"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Center-Cluster PentaMirror"),
            intervals: Some(vec![3.0, 1.0, 1.0, 3.0, 4.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chad Gadyo"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 5.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chaio"),
            intervals: Some(vec![2.0, 3.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chin"),
            intervals: Some(vec![3.0, 3.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chinese"),
            origin: Some(String::from("China")),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chinese Eight-tone"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ching"),
            origin: Some(String::from("China")),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic"),
            intervals: Some(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Diatonic Dorian"),
            intervals: Some(vec![1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Dorian"),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Dorian Inverse"),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Hypodorian"),
            intervals: Some(vec![2.0, 1.0, 1.0, 3.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Hypolydian"),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Hypolydian Inverse"),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Hypophrygian Inverse"),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Lydian"),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Lydian Inverse"),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Mixolydian"),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Mixolydian 2"),
            intervals: Some(vec![1.0, 1.0, 2.0, 2.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Mixolydian Inverse"),
            intervals: Some(vec![2.0, 3.0, 1.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Permutated Diatonic Dorian"),
            intervals: Some(vec![1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Phrygian"),
            intervals: Some(vec![3.0, 1.0, 1.0, 3.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Chromatic Phrygian Inverse"),
            intervals: Some(vec![1.0, 1.0, 2.0, 3.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Cushak"),
            origin: Some(String::from("Armenia")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Damian Emmanuel"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dasrgah-e Mahur"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dastgah-e Rast Panjgah"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Diminished"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Diminished Half-tone"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Diminished Pentatonic"),
            intervals: Some(vec![3.0, 3.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dominant Pentatonic"),
            intervals: Some(vec![2.0, 2.0, 3.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian #4"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian Aeolian"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian b2"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian b2 b4"),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian b2 Maj7"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian b5"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian b9"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian b9 #11"),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorian Pentatonic"),
            intervals: Some(vec![2.0, 1.0, 4.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Dorico Flamenco"),
            origin: Some(String::from("Spain")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Double Harmonic"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Double Harmonic Minor"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Double Phrygian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 3.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Egyptian"),
            origin: Some(String::from("Egypt")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Enigmatic"),
            intervals_ascending: Some(vec![1.0, 3.0, 2.0, 2.0, 2.0, 1.0, 1.0]),
            notes_ascending: Some(vec![0.0, 1.0, 4.0, 6.0, 8.0, 10.0, 11.0]),
            intervals_descending: Some(vec![1.0, 3.0, 1.0, 3.0, 2.0, 1.0, 1.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Enigmatic Minor"),
            intervals: Some(vec![1.0, 2.0, 3.0, 2.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Enigmatic Mixed"),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Eskimo Hexatonic"),
            origin: Some(String::from("Alaska")),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Eskimo Hexatonic 2"),
            origin: Some(String::from("Alaska")),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Esplá"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ezel"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Flamenco"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Freygish or Fraigish"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Full Minor All Flats"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Geez"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Genus Chromaticum"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Genus Diatonicum"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Genus Diatonicum Veterum Correctum"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Genus Secundum"),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Genus Tertium"),
            intervals: Some(vec![3.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ghana Heptatonic"),
            origin: Some(String::from("Ghana")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ghana Pentatonic"),
            origin: Some(String::from("Ghana")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 5.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ghana Pentatonic 2"),
            origin: Some(String::from("Ghana")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gong"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Greek Arkaik"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 6.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 1"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 2"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 3"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 4"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 5"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 6"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 7"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gregorian 8"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gu"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gu Xian"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gypsy"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gypsy 2"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 4.0, 1.0, 2.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 6.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gypsy Hexatonic"),
            origin: Some(String::from("Hungarian")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gypsy Hexatonic Inverse"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gypsy Inverse"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Gypsy Minor"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Half Diminished"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hamel"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Han-Kumoi"),
            intervals: Some(vec![2.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Major"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Major 2"),
            intervals: Some(vec![2.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Major Inverse"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Minor #4"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Minor b5"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Minor Inverse"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Harmonic Neapolitan Minor"),
            origin: Some(String::from("Italy")),
            intervals: Some(vec![1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hawaiian"),
            origin: Some(String::from("Hawaii")),
            intervals: Some(vec![2.0, 1.0, 4.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hexatonic"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hijaz Kar"),
            origin: Some(String::from("Arabia")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hijaz Major"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 4.0, 1.0, 2.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 6.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hindi b2 b3 b7"),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hindu"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hirajoshi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hitzaz or Hijaz"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hitzazkiar (or Hijazskiar)"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Honkoshi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hon-Kumoi-Joshi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 1.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Houseini"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Houzam (or Huzam)"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Humayun"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Folk"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Gypsy"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Major"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Major Inverse"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Minor"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Minor b2"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hungarian Minor Inverse"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hybrid Pentatonic"),
            intervals: Some(vec![3.0, 2.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Hyojo"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ian Iwato"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ichilkotsucho (or Ishikosucho)"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("In"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Insen"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Insen Pentatonic"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Inverted Augmented"),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ioanian Augmented"),
            intervals: Some(vec![2.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ionian #2"),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ionian #5"),
            intervals: Some(vec![2.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ionian Augmented #2"),
            intervals: Some(vec![3.0, 1.0, 1.0, 3.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ionian Augmented b9"),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ionian b5"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ionian Pentatonic"),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Istrian"),
            origin: Some(String::from("Croatia")),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 1.0, 5.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Iwato"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 1.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Javanese"),
            origin: Some(String::from("Java")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Jazz Minor Inverse"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Jeths"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Jia Zhong"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Jiao"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Jin-Yu (or Quin-Yu)"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Jog"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("John Found's Mantra Of Will"),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kaffa"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kartzihiar"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kata-Kumoi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kiourdi"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kokin-Choshi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kubilai"),
            origin: Some(String::from("Mongolia")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kumoi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 4.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kung"),
            intervals: Some(vec![2.0, 2.0, 2.0, 3.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Kyemyonjo"),
            intervals: Some(vec![3.0, 2.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lai Noi"),
            origin: Some(String::from("Laos")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lai Po Sai"),
            origin: Some(String::from("Laos")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lai Soutsanaen"),
            origin: Some(String::from("Laos")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lai Yai"),
            origin: Some(String::from("Laos")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Leading Whole-Tone"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Leading Whole-Tone Inverse"),
            intervals: Some(vec![1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("LG Octatonic"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian #6"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian b4"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian bb3 bb7"),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian bb7"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian Dominant"),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian Maj7"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian Natural 2"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian Natural Maj6"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Locrian Pentatonic"),
            intervals: Some(vec![3.0, 1.0, 2.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian #2"),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian #2.0 #6"),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian #2.0 Hexatonic"),
            intervals: Some(vec![3.0, 1.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian #5"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian #6"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Augmented"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Augmented #2"),
            intervals: Some(vec![3.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Augmented #3"),
            intervals: Some(vec![2.0, 3.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Augmented Dominant"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian b3"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian b7"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Diminished"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Dominant"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Dominant b6"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Hexatonic"),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Minor"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Mixolydian"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Lydian Pentatonic"),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Madenda Modern"),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Magen Abot"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Augmented"),
            intervals: Some(vec![3.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Gypsy"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Inverse"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Locrian"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Minor"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Minor Mixed"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Mixolydian"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Pentatonic"),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Pentatonic b2"),
            intervals: Some(vec![1.0, 3.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Pentatonic b2 b5"),
            intervals: Some(vec![1.0, 3.0, 2.0, 3.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Pentatonic b3"),
            intervals: Some(vec![1.0, 2.0, 3.0, 3.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Pentatonic b6"),
            intervals: Some(vec![2.0, 2.0, 3.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major Pentatonic b7 #9"),
            intervals: Some(vec![3.0, 1.0, 3.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major-Dorian Mixed"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Major-Lydian Mixed"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Makam Lami"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Man Gong"),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Man Jue"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Athar Kurd"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Bayat-e-Esfahan"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Cargah"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Farahfaza"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Hedjaz"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Hicaz"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Hijaz"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Hijaz Kar"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Hijaz-Nahawand"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Hisar"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Humayun"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Huzzam"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Karcigar"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Kurd"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Nahawand"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Nahawand Murassah"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Nakriz"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Nawa Athar"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Saba Zamzam"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Shadd'araban"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Shahnaz (or Shahnaz Kurdi)"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Shawq Afza"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Shawq Awir"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Sultani Yakah"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Suzidil"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Tarzanuyn"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Ussak"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Zanjaran"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Maqam Zengule"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Bhairavi That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Bhavapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Cakravka"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Calanata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Carukesi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Citrambari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Dharmavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Dhatuvardhani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Dhavalambari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Dhenuka"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Divyamani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Gamanasrama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Ganamurti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Gangeyabhusani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Gaurimanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Gavambodhi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Gayakapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Hanumatodi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Harikamboji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Hatakambari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Hemavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Jalarnava"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Jhalavarli"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Jhankaradhvani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Jyotisvarupini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kamavardhani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kanakangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kantamani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kharaharapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kiravani"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kokilapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Kosalam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Latangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Manavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Mayamalavagowla"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Mecakalyani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Naganandini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Namanarayani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Nasikabhusani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Natabhairavi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Natakapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Navanitam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Nitimati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Pavani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Ragavardhani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Raghupriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Ramapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Rasikapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Ratnangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Rupavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Sadvidhmargini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Salaga"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Sanmukhapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Sarasangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Senavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Shankarabharanam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Shubhapanturavali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Sulini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Sumhendramadhyama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Suryakanta"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Suvarnangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Syamalangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Tenarupi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Vacaspati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Vagadhisvari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Vakulabharanam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Varunapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Venaspati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Visvambhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mela Yagapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Melodic Major"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Melodic Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Melodic Minor #4"),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Melog Selisir"),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 1st Mode"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 2nd Mode"),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 2nd Mode"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 2nd Mode Inverse"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 2nd Mode Truncated"),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 3rd Mode"),
            intervals: Some(vec![2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 6.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 3rd Mode Inverse"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 4th Mode"),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 4th Mode Inverse"),
            intervals: Some(vec![3.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 6.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 5th Mode"),
            intervals: Some(vec![1.0, 4.0, 1.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 5th Mode Inverse"),
            intervals: Some(vec![4.0, 1.0, 1.0, 4.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 6.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 6th Mode"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 6th Mode Inverse"),
            intervals: Some(vec![1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 7th Mode"),
            intervals: Some(vec![1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 5.0, 6.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen 7th Mode Inverse"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Messiaen Truncated 3rd Mode Inv."),
            intervals: Some(vec![3.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor 6th Added"),
            intervals: Some(vec![3.0, 2.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor b5"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor Gypsy Inverse"),
            origin: Some(String::from("Hungary")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor Hexatonic"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor Locrian"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor Pentatonic"),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor Pentatonic 7.0 b5"),
            intervals: Some(vec![3.0, 2.0, 1.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minor Pentatonic with Leading Tones"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Minyo"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mischung 1"),
            origin: Some(String::from("Germany")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mischung 2"),
            origin: Some(String::from("Germany")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mischung 3"),
            origin: Some(String::from("Germany")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mischung 5"),
            origin: Some(String::from("Germany")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mischung 6"),
            origin: Some(String::from("Germany")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Misheberekh"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian #1"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian #4"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian Augmented"),
            intervals: Some(vec![2.0, 2.0, 1.0, 3.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian Augmented Maj9"),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian b2 (or Mixolydian b9)"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian b5"),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian b6 (or Mixolydian b13)"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian Dorian"),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian Hexatonic"),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian Hexatonic 2"),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mixolydian Pentatonic"),
            intervals: Some(vec![4.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Miyakobushi"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Modus Conjunctus"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Mohammedan"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Moorish Phrygian"),
            origin: Some(String::from("Spain")),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Naka Zora"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Nam"),
            origin: Some(String::from("Vietnam")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Nando-Kyemyonjo"),
            origin: Some(String::from("Korea")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 5.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Natural Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Neapolitan Major (or Neapolitan)"),
            origin: Some(String::from("Italy")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Neapolitan Major b4"),
            origin: Some(String::from("Italy")),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Neapolitan Major b5"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Neapolitan Minor"),
            origin: Some(String::from("Italy")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Neseveri"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Niagari"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Niavent"),
            origin: Some(String::from("Egypt")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Niaventi Minor"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Nigriz"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Noh"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 3.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Nohkan"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 3.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Nonatonic 2"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Octatonic"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Olympos Enharmonic"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Oriental"),
            origin: Some(String::from("China")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Oriental 2"),
            origin: Some(String::from("China")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Oshikicho"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ousak"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Overtone"),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("P'Yongjo"),
            origin: Some(String::from("Korea")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("P'yongjo-kyemyonjo"),
            origin: Some(String::from("Korea")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Peiraiotikos"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pelo Degung Modern"),
            origin: Some(String::from("Bali")),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pelog"),
            origin: Some(String::from("Bali")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pelog Pentatonic"),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pentatonic Whole-Tone"),
            intervals: Some(vec![4.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Periaiotikos Minor"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Persian"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Peruvian Major"),
            origin: Some(String::from("Peru")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Peruvian Minor"),
            origin: Some(String::from("Peru")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Peruvian Major Pentatonic"),
            origin: Some(String::from("Peru")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Peruvian Minor Pentatonic"),
            origin: Some(String::from("Peru")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Petrushka chord"),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Aeolian b4"),
            intervals: Some(vec![1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Aeolian Mixed"),
            intervals: Some(vec![1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian b4"),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian b4 Maj7"),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Dominant"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Hexatonic"),
            intervals: Some(vec![3.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Locrian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Major"),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Mixolydian"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Natural 6"),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Phrygian Pentatonic"),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ping"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pireotikos"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pomeroy"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Prokofiev"),
            origin: Some(String::from("Russia")),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Prometheus"),
            intervals: Some(vec![2.0, 2.0, 2.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Prometheus Liszt"),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Prometheus Neapolitan"),
            origin: Some(String::from("Italy")),
            intervals: Some(vec![1.0, 3.0, 2.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Puravi b6"),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pyeong Jo"),
            origin: Some(String::from("Korea")),
            intervals: Some(vec![2.0, 3.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pygmy"),
            intervals: Some(vec![2.0, 1.0, 4.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Pyramid Hexatonic"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Qing Shang"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Quan Ming"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Quin-Yu (or Jin-Yu)"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Abheri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Abhogi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 4.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Adana"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ahir Bhairav"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ahira Lalita"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ahiri Todi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Aivarati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Alhaiya Bilaval"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Amarasenapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ambika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Amrtavarshini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Anandabhairavi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Andhali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Andolika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Arabhi ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Arabhi descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Arunajualita"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Asavari That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Audav Tukhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 3.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bahar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bahudari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bairagi (or Baira)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bairari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Balahamsa"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Barbara"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Basant"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bauli"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Begeshri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Begeshri 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhairav That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhairavi ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhairavi descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhairubahar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhankar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhanumati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhatiyar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhavani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhimpalasi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhinna Pancama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhinna Shadj"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhinnasadjam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhogachayanata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhopali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhunumanjari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhup"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhupalam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhupeshwari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bhusavati (or Bhusavali)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bibhas"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bihag"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bihagara"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bilahari ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bilahari descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bilashkhani Todi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bindumalini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Bowli"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![1.0, 3.0, 3.0, 1.0, 5.0]),
            notes_ascending: Some(vec![0.0, 1.0, 4.0, 7.0, 8.0]),
            intervals_descending: Some(vec![1.0, 3.0, 3.0, 1.0, 3.0, 1.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Brindabani (or Brindabani Serang)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Budhamanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 5.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Camara"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chakravakam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chandrajyoti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chandrakauns Kafi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chandrakauns Kiravani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chandrakauns Modern"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chaturangini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chaturangini 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chaya Todi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 2.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chaya Vati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chayanat"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chayanata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chinthamani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Chitthakarshini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 3.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Cudamani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Darbar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Desh"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Desh Malhar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Deshgaur"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 6.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Deshi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Deshi 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Deshi 3"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Deskar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Desya Khamas"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Desya Todi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devagandhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devakriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devamani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devamanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devarangini 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devaranjani (or  Devaranji)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![5.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Devarashtra"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhaanyasi ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhanyasi descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhauta Pancama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhavalangam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhavalashri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhipaka"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dhunibinnashadjam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dipak"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 5.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Disisimharavam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dumyaraga"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Dvigandharabushini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 3.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gamakakriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gamakasamantam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gambhiranata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ganasamavarali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ganavaridhi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gandharavam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gangatarangini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 1.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gaud Sarang"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gaula"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gaulipantu"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gauri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gauri Velavali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gaurikriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Geyahejjajji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ghandarva"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ghanta"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ghantana"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Girija"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Girvani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gopikacasantam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gopikatilaka"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gopriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gorakh"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gowla"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![1.0, 4.0, 2.0, 4.0, 1.0]),
            notes_ascending: Some(vec![0.0, 1.0, 5.0, 7.0, 11.0]),
            intervals_descending: Some(vec![1.0, 3.0, 1.0, 2.0, 4.0, 1.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Guhamanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gunkali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Gurjari Todi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamir Kalyani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamsadhvani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 4.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamsadhvani 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamsagiri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamsalata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamsanada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hamsanarayami"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hansanandi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hari Nata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Harikauns"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Harini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Haripriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hejjajji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hindol"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hindolam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Hindolita"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Huseni"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Indupriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jaganmohanam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jaganmohini ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jait Kalyan"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Janasammodini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Janjuti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jaunpuri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jayakauns"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 1.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jayamanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jeyasuddhamalavi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jhankara Bhramavi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jinavali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jingla"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jivantika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jivantini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jog"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jogiya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jotismatti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Jyoty"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kafi That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kaihavasi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kaishikiranjani (or Kaushiranjani)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalagada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalahamsa"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalakanthi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalakanthi 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalamurti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalingada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalyan That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalyana Vasantha"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalyani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kalyani Keseri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kamalamanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kamalamanohari 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kambhoji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kambodhi ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kambodhi descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kamud"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kanakambari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kanara"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kannadabangala"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kashyapi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kasiramakryia"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kaushikdhvani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kedar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kedaram"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes_ascending: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            intervals_descending: Some(vec![2.0, 2.0, 1.0, 2.0, 4.0, 1.0]),
            notes_descending: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Keradam ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Keseri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Khamach"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![4.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes_ascending: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            intervals_descending: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes_descending: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Khamaj"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Khamaj That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Khamaji Durga"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Khamas"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Khambhavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kharapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kiranavali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kirvani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kokil Pancham"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kokila"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kokilaravam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kshanika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kuksumakaram"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kumarapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 6.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kumudki"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 5.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kumurdaki"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 5.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kunakri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kunbhini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kuntala"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Kuntvarali (or Kuntalavarali)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![5.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Lalit"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Lalita"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Lalita Bhairav"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Lalita Panchami"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Lasaki"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Latantapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Latika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhava Manohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhmat Sarang"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhukauns"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhuranjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhuri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhuvanti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Madhyamavati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malahari"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![1.0, 4.0, 2.0, 1.0, 4.0]),
            notes_ascending: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0]),
            intervals_descending: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 4.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malahari ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malarani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malashri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malavastri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malayamarutam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malgunji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Malkauns"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mamata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manaranjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manaranjani 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manavi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 4.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mand"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mandari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manirangu"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Manoranjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Marga Hindola"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Marva"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Marwa Thaat"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Matha Kokila"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 5.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Megh (or Megh Malhar)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Megharamji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 6.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Megharanjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Miam Ki Malhar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mohanam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mohanangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mruganandana"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Mukhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Multani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Multani 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nabhomani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 5.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nagabharanam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nagagandhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nagaswaravali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nalinakanti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nandkauns"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Narmada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nasamani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nasikabhusani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Natabharanam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nattai"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes_ascending: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            intervals_descending: Some(vec![3.0, 2.0, 2.0, 4.0, 1.0]),
            notes_descending: Some(vec![0.0, 3.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nattaikurinji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Navamanohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nayaki"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nayaki Kanada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Neelangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Neroshta"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 5.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 11.0, 13.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nileshwari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 1.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nisada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Nishadi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga None"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Padi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Pahadi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Palasi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Pancama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Pantuvarali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Paraj"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Paras (or Pharas or Paraju)"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![4.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes_ascending: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            intervals_descending: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Partivaran"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Patdip"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Phenadyuti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Phenadyuti 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Pilu"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Pilu That"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Prabhati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Pratapa"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Priyadharshini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Puriya (or Puriya Kalyan)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Puriya 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Puriya Dhanashri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Purna Pancama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Purnanalita"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 5.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Puruhutika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![5.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Purvaholica"),
            origin: Some(String::from("India")),
            intervals: Some(vec![5.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Purvi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Purvi Thaat"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Purvikalyani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Putrika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 6.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Raga Pushpalithika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ragamalini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rageshri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 4.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ramakri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ramamahohari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ramamanohari 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ramdasi Malharq"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ramkali 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rangini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ranjiani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rasamanjari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rasamanjari 2"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rasavali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rasika Ranjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rasranjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ratipriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ratnakanthi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ravikriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Reejeshwari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Regeshri (or Rageshwari)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Reva (or Revagupti)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 1.0, 5.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ribhavari (or Revati)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rishabapriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Ritigaula"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rudra Pancama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Rukmangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 4.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sahera"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sailadesakshi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Salagavarali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 4.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Salanganata"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Samanta"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Samudhra Priya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 3.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sanjk Ka Hindol"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sankara (or Shankara)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Santanamanjari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 2.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sarasanana"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sarasvati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Saravati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Saugandhini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 5.0, 1.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 6.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Saugandhunu"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 5.0, 1.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 6.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Saurashtra"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sauviram"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Saveri ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Saveri descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Savethri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Savitri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Senagrani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shailaja"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 4.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shilangi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shobhavari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shree ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shree descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shreeranjani\n(or Shee Ranjani or Sriranjani)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shri Kalyan"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shubravarni"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shuddh Kalyan"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Shyamalam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Simharava"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Simhavahini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Simmendramadhyamam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sindhi-Bhairavi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sindhu Ramakriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sindhura"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sindhura Kafi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Siva Kambhoji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sohani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sohini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sohni"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 4.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sorati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sowrashtram"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Srutiranjani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sthavarajam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Bangala"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Mukhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 3.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Pancama"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Ramakriya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Saveri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Simantini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suddha Todi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suha Kanada"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suha Sughrai"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sunada Vinodini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Suposhini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Supradhipam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sur Malhar"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Surati"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Surya"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Sutradhari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Syamalam"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Takka"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Tanukirti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Tarangini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Tilang"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Tilang (or Bridabani Tilang)"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 1.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Tivravahini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Trimurti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 4.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Trishuli"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Udayaravicandrika"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Udhayaravi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vagedeeshwari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vaijayanti"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 4.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Valaji"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 3.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vamsavathi"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Varali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 3.0, 1.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 5.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Varamu"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 2.0, 4.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Varini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![3.0, 4.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vasanta"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![4.0, 1.0, 4.0, 2.0, 1.0]),
            notes_ascending: Some(vec![0.0, 4.0, 5.0, 9.0, 11.0]),
            intervals_descending: Some(vec![1.0, 3.0, 1.0, 4.0, 2.0, 1.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 5.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vasantha"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vativasanta"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vegavahini"),
            origin: Some(String::from("India")),
            intervals_ascending: Some(vec![4.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes_ascending: Some(vec![0.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            intervals_descending: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes_descending: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vegavahini descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Velavali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vibhas"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 3.0, 3.0, 1.0, 5.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vijayanagari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vijayasri"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vijayavasanta"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vilasini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Viravasantham"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 3.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vivardhini"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Viyogavarali"),
            origin: Some(String::from("India")),
            intervals: Some(vec![1.0, 2.0, 2.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Vutari"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Yaduka Kambodi descending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Yadukua Kambodhi ascending"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Yaman Kalyan"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Yamuna Kalyani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 2.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Zilaf"),
            origin: Some(String::from("India")),
            intervals: Some(vec![4.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Raga Zilla"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ragta Narayani"),
            origin: Some(String::from("India")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Rahawi"),
            origin: Some(String::from("Iraq")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Rast"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ravel"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Relative Blues"),
            intervals: Some(vec![2.0, 1.0, 1.0, 3.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ritusen"),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ritzu"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 2.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ritzu Gagaku"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Rock ‘n Roll"),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Romanian Bacovia"),
            origin: Some(String::from("Romania")),
            intervals: Some(vec![4.0, 1.0, 3.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Romanian Major"),
            origin: Some(String::from("Romania")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Rui Bin"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ryo"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ryosen"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ryukyu"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![4.0, 1.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 4.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sabach"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 1.0, 3.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sabach Maj7"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 1.0, 3.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sakura"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Scottish Hexatonic"),
            origin: Some(String::from("Scotland")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Scottish Pentatonic"),
            origin: Some(String::from("Scotland")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Scriabin"),
            intervals: Some(vec![1.0, 3.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Segiah"),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Se"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Semilocrian"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Semilocrian b4"),
            intervals: Some(vec![2.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sengiach (or Sengah)"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![3.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Shang"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Shang-Diao"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sho"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sho #2"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 4.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Shostakovich"),
            origin: Some(String::from("Russia")),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Soft Ascend"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 4.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Soft Descend"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Souzinak"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Spanish Heptatonic"),
            origin: Some(String::from("Spain")),
            intervals: Some(vec![3.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Spanish Octatonic"),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Spanish (or Spanish Gypsy)"),
            origin: Some(String::from("Spain")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Spanish Phrygian"),
            origin: Some(String::from("Spain")),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Sultani Yakah"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Superlocrian"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Superlocrian #6"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Superlocrian bb3"),
            intervals: Some(vec![1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Superlocrian bb6 bb7"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Superlocrian Maj7"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Suspended Pentatonic"),
            intervals: Some(vec![2.0, 3.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Symmetrical Decatonic"),
            intervals: Some(vec![1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Symmetrical Nonatonic"),
            intervals: Some(vec![1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 6.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Synthetic Mixture #5"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Syrian Pentatonic"),
            origin: Some(String::from("Syria")),
            intervals: Some(vec![1.0, 3.0, 1.0, 3.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tabahaniotiko"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Taishikicho"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Takemitzu Tree 1"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 3.0, 2.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Takemitzu Tree 2"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 3.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tcherepnin"),
            origin: Some(String::from("Russia")),
            intervals: Some(vec![1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tcherepnin Major Pentatonic"),
            origin: Some(String::from("Russia")),
            intervals: Some(vec![2.0, 3.0, 2.0, 4.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tezeta Major"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tezeta Minor"),
            origin: Some(String::from("Ethiopia")),
            intervals: Some(vec![1.0, 2.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Todi b7"),
            intervals: Some(vec![1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tritone"),
            intervals: Some(vec![1.0, 3.0, 2.0, 1.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 6.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tsinganikos"),
            origin: Some(String::from("Greece")),
            intervals: Some(vec![1.0, 3.0, 1.0, 1.0, 3.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 6.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Tunisian"),
            origin: Some(String::from("Tunisia")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Two-semitone Tritone"),
            intervals: Some(vec![1.0, 1.0, 4.0, 1.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 6.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ujo"),
            origin: Some(String::from("Korea")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ukrainian Minor"),
            origin: Some(String::from("Ukraina")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ukranian Dorian"),
            origin: Some(String::from("Ukraina")),
            intervals: Some(vec![2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 6.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ultralocrian"),
            intervals: Some(vec![1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ultralocrian bb3"),
            intervals: Some(vec![1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 2.0, 4.0, 6.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Ultraphrygian"),
            intervals: Some(vec![1.0, 2.0, 1.0, 3.0, 1.0, 1.0, 3.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 4.0, 7.0, 8.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Utility Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Van Der Host"),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Whole-Tone"),
            intervals: Some(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Whole-Tone Diminished"),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Xin"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 2.0, 1.0, 2.0, 2.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 5.0, 7.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yi Ze"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 3.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yishtabach"),
            origin: Some(String::from("Jewish")),
            intervals: Some(vec![1.0, 2.0, 2.0, 1.0, 2.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 6.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yo"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yona Nuki Major"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 2.0, 3.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 4.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yona Nuki Minor"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 1.0, 4.0, 1.0, 4.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 7.0, 8.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yosen"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yu"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 2.0, 1.0, 2.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 9.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Yu 2"),
            origin: Some(String::from("China")),
            intervals: Some(vec![3.0, 2.0, 2.0, 3.0, 2.0]),
            notes: Some(vec![0.0, 3.0, 5.0, 7.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zangula"),
            origin: Some(String::from("Nigeria")),
            intervals: Some(vec![2.0, 1.0, 2.0, 1.0, 3.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 6.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zhalibny Minor"),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 3.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zheng"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zhi"),
            origin: Some(String::from("China")),
            intervals: Some(vec![2.0, 3.0, 2.0, 2.0, 3.0]),
            notes: Some(vec![0.0, 2.0, 5.0, 7.0, 9.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zilof"),
            origin: Some(String::from("Spain")),
            intervals: Some(vec![1.0, 3.0, 1.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 4.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zirafkend"),
            origin: Some(String::from("Arabia")),
            intervals: Some(vec![2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0]),
            notes: Some(vec![0.0, 2.0, 3.0, 5.0, 7.0, 8.0, 9.0, 11.0]),
            ..Default::default()
        },
        ListedScale {
            name: String::from("Zokuso"),
            origin: Some(String::from("Japan")),
            intervals: Some(vec![1.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0]),
            notes: Some(vec![0.0, 1.0, 3.0, 5.0, 7.0, 8.0, 10.0]),
            ..Default::default()
        }
    ];
}
