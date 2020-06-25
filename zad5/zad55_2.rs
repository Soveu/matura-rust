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

struct Language<'a> {
    name:   &'a str,
    users:  u64,
    official: bool,
}

struct Country<'a> {
    population: u64,
    languages: Vec<Language<'a>>,
}

impl<'a> Country<'a> {
    fn with_population(p: u64) -> Self {
        Self {
            population: p,
            languages:  Vec::new(),
        }
    }
}

fn main() {
    let countryfile = read_all_file("../dane/panstwa.txt");
    let mut countries: HashMap<&str, Country> = HashMap::new();

    for line in countryfile.lines().skip(1) {
        let mut fields = line.splitn(3, '\t');
        let country = fields.by_ref().next().expect("Invalid file format");
        let population = fields.by_ref().nth(1).expect("Invalid file format");

        let population = parse_millions(population).expect("Invalid number format");

        countries.insert(country, Country::with_population(population));
    }

    let usersfile = read_all_file("../dane/uzytkownicy.txt");
 
    for line in usersfile.lines().skip(1) {
        let mut fields = line.splitn(4, '\t');
        let country  = fields.by_ref().next().expect("Invalid file format");
        let language = fields.by_ref().next().expect("Invalid file format");
        let users    = fields.by_ref().next().expect("Invalid file format");
        let official = fields.by_ref().next().expect("Invalid file format");

        let users = parse_millions(users).expect("Invalid number format");

        let langinfo = Language {
            name:       language,
            users:      users,
            official:   official == "tak",
        };

        let entry = countries.get_mut(country).unwrap();
        entry.languages.push(langinfo);
    }

    let iter = countries.into_iter().flat_map(|(k, v)| {
        let pop = v.population;
        v.languages.into_iter()
            .filter(move |lang| !lang.official && lang.users * 10 >= pop * 3)
            .map(move |lang| (k, lang.name, lang.users * 100 / pop))
    });

    for (country, langname, percent) in iter {
        println!("{} {} {}%", country, langname, percent);
    }
}

