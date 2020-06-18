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

fn main() {
    let input = read_all_file("../dane/jezyki.txt");
    let mut map: HashMap<&str, u32> = HashMap::new();

    for line in input.lines().skip(1) {
        let n = line.find('\t').expect("Could not find tab in line");
        let family = &line[n+1..];
        let entry = map.entry(family).or_insert(0);
        *entry += 1;
    }

    let mut vec: Vec<(&str, u32)> = map.into_iter().collect();
    vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    for (family, langnum) in vec {
        println!("{} {}", family, langnum);
    }
}

