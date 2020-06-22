use std::fs::File;
use std::io::Read;

fn read_all_file(filename: &str) -> String {
    let mut buffer = Vec::new();
    let mut file = File::open(filename).expect("failed to open file");
    file.read_to_end(&mut buffer).expect("failed to read file");

    /* Safety: the file is guaranteed to be a valid ascii file */
    return unsafe { String::from_utf8_unchecked(buffer) };
}

fn pair_from_line(s: &str) -> (usize, &str) {
    let mut it = s.split(' ');

    let number = it.by_ref().next().expect("invalid file format");
    let s = it.by_ref().next().expect("invalid file format");

    let number = number.parse().expect("invalid number format");

    return (number, s);
}

fn cmp(a: &(usize, &str), b: &(usize, &str)) -> std::cmp::Ordering {
    if a.0 == b.0 {
        return a.1.cmp(&b.1);
    }

    return a.0.cmp(&b.0);
}

fn main() {
    let input = read_all_file("../dane/pary.txt");

    let max = input.lines()
        .map(pair_from_line)
        .max_by(cmp)
        .expect("cant find max");

    println!("{} {}", max.0, max.1);
}

