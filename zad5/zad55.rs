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

/* parse for example "16,4" into 16_400_000 */
fn parse_millions(s: &str) -> Option<u64> {
    let n = s.find(',')?;
    let (millions, rest) = s.split_at(n);
    let rest = &rest[1..];

    let millions: u64 = match millions.parse() {
        Ok(x) => x,
        Err(_) => return None,
    };
    debug_assert!(rest.len() == 1);
    let rest: u64 = match rest.parse() {
        Ok(x) => x,
        Err(_) => return None,
    };

    return Some(millions * 1000000 + rest * 100000);
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

    let langfile = read_all_file("../dane/jezyki.txt");
    let mut lang_to_family: HashMap<&str, &str> = HashMap::new();

    for line in langfile.lines().skip(1) {
        let mut fields = line.splitn(2, '\t');
        let lang   = fields.by_ref().next().expect("Invalid file format");
        let family = fields.by_ref().next().expect("Invalid file format");
        lang_to_family.insert(lang, family);
    }

    let usersfile = read_all_file("../dane/uzytkownicy.txt");
    let mut langusers: HashMap<&str, u64> = HashMap::new();
 
    for line in usersfile.lines().skip(1) {
        let mut fields = line.splitn(4, '\t');
        let country  = fields.by_ref().next().expect("Invalid file format");
        let language = fields.by_ref().next().expect("Invalid file format");
        let users    = fields.by_ref().next().expect("Invalid file format");
        
        let continent = country_to_continent[country];
        if continent != NorthAmerica && continent != SouthAmerica {
            continue;
        }
        if lang_to_family[language] == "indoeuropejska" {
            continue;
        }

        let entry = langusers.entry(language).or_insert(0);
        *entry += parse_millions(users).expect("invalid number format");
    }

    let mut vec: Vec<(&str, u64)> = langusers.into_iter().collect();
    vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    for (lang, users) in vec.into_iter().take(6) {
        let family = lang_to_family[lang];
        println!("{} {} {}", lang, family, users);
    }
}

