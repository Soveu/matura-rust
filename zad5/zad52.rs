use std::fs::File;
use std::collections::BTreeSet;
use std::io::Read;

fn read_all_file(filename: &str) -> String {
    let mut buffer = Vec::new();
    let mut file = File::open(filename).expect("failed to open file");
    file.read_to_end(&mut buffer).expect("failed to read file");

    /* Safety: the file is guaranteed to be a valid ascii file */
    return unsafe { String::from_utf8_unchecked(buffer) };
}

fn main() {
    let langfile = read_all_file("../dane/jezyki.txt");
    let mut all_langs: BTreeSet<&str> = BTreeSet::new();

    for line in langfile.lines().skip(1) {
        let n = line.find('\t').expect("Could not find tab in line");
        let lang = &line[..n];
        all_langs.insert(lang);
    }

    let usersfile = read_all_file("../dane/uzytkownicy.txt");
    let mut official_langs: BTreeSet<&str> = BTreeSet::new();

    for line in usersfile.lines().skip(1) {
        let mut fields = line.splitn(4, '\t');
        let lang = fields.by_ref().nth(1).expect("Invalid file format"); /* 2nd field */
        let official = fields.by_ref().nth(1).expect("Invalid file format"); /* 4th field */
        
        debug_assert!(official == "tak" || official == "nie");

        if official == "tak" {
            official_langs.insert(lang);
        }
    }

    println!("{}", all_langs.difference(&official_langs).count());
}

