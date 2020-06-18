#![allow(non_upper_case_globals)]

use std::fs::File;
use std::collections::HashMap;
use std::io::Read;

fn read_all_file(filename: &str) -> String {
    let mut buffer = Vec::new();
    let mut file = File::open(filename).expect("failed to open file");
    file.read_to_end(&mut buffer).expect("failed to read file");

    /* Safety: the file is guaranteed to be a valid ascii file */
    return unsafe { String::from_utf8_unchecked(buffer) };
}

const Europe: u16          = 1 << 0;
const Asia: u16            = 1 << 1;
const Africa: u16          = 1 << 2;
const NorthAmerica: u16    = 1 << 3;
const SouthAmerica: u16    = 1 << 4;
const Australia: u16       = 1 << 5;

fn continent_from_str(s: &str) -> Option<u16> {
    match s {
        "Europa"             => Some(Europe),
        "Azja"               => Some(Asia),
        "Afryka"             => Some(Africa),
        "Ameryka Polnocna"   => Some(NorthAmerica),
        "Ameryka Poludniowa" => Some(SouthAmerica),
        "Australia"          => Some(Australia),
        _ => None,
    }
}

fn main() {
    let countryfile = read_all_file("../dane/panstwa.txt");
    let mut country_to_continent: HashMap<&str, u16> = HashMap::new();

    for line in countryfile.lines().skip(1) {
        let mut fields = line.splitn(3, '\t');
        let country = fields.by_ref().next().expect("Invalid file format");
        let continent = fields.by_ref().next().expect("Invalid file format");
        let continent = continent_from_str(continent).expect("Does this continent exist?");

        country_to_continent.insert(country, continent);
    }

    let usersfile = read_all_file("../dane/uzytkownicy.txt");
    let mut langs_on_continents: HashMap<&str, u16> = HashMap::new();
 
    for line in usersfile.lines().skip(1) {
        let mut fields = line.splitn(4, '\t');
        let country = fields.by_ref().next().expect("Invalid file format");
        let language = fields.by_ref().next().expect("Invalid file format");

        let entry = langs_on_continents.entry(language).or_insert(0);
        let continent = country_to_continent[country];
        *entry |= continent;
    }

    langs_on_continents.into_iter()
        .filter(|(_lang, continents)| continents.count_ones() >= 4)
        .map(|(lang, _continents)| lang)
        .for_each(|lang| println!("{}", lang));
}

