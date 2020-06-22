use std::fs::File;
use std::io::Read;

fn read_all_file(filename: &str) -> String {
    let mut buffer = Vec::new();
    let mut file = File::open(filename).expect("failed to open file");
    file.read_to_end(&mut buffer).expect("failed to read file");

    /* Safety: the file is guaranteed to be a valid ascii file */
    return unsafe { String::from_utf8_unchecked(buffer) };
}

fn string_from_line(s: &str) -> &str {
    s.split(' ').nth(1).expect("invalid file format")
}

fn longest_common_piece(s: &str) -> (char, usize) {
    let mut maxc = '.';
    let mut maxn = 0;

    let mut c = '.';
    let mut n = 0;

    for x in s.chars() {
        if x != c {
            if n > maxn {
                maxn = n;
                maxc = c;
            }
            c = x;
            n = 0;
        }

        n += 1;
    }

    if n > maxn {
        maxn = n;
        maxc = c;
    }

    return (maxc, maxn);
}

fn main() {
    let input = read_all_file("../dane/pary.txt");

    let iter = input.lines()
        .map(string_from_line);

    for s in iter {
        let (c, n) = longest_common_piece(s);
        for x in std::iter::repeat(c).take(n) {
            print!("{}", x);
        }
        println!(" {}", n);
    }
}

