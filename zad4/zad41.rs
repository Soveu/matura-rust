use std::fs::File;
use std::io::Read;

fn read_all_file(filename: &str) -> String {
    let mut buffer = Vec::new();
    let mut file = File::open(filename).expect("failed to open file");
    file.read_to_end(&mut buffer).expect("failed to read file");

    /* Safety: the file is guaranteed to be a valid ascii file */
    return unsafe { String::from_utf8_unchecked(buffer) };
}

fn get_primes(n: usize) -> Vec<bool> {
    let mut is_prime = Vec::new();
    is_prime.resize(n, true);
    is_prime[0] = false;
    is_prime[1] = false;

    for number in 2..n {
        if !is_prime[number] { continue; }

        for nonprime in (2*number .. n).step_by(number) {
            is_prime[nonprime] = false;
        }
    }

    return is_prime;
}

fn goldbach(x: usize, is_prime: &[bool]) -> (usize, usize, usize) {
    let primes = is_prime.into_iter().enumerate()
        .filter(|(_n, &b)| b)
        .map(|(n, _b)| n);

    for prime in primes {
        if is_prime[x - prime] { return (x, prime, x-prime); }
    }

    unreachable!();
}

fn number_from_line(s: &str) -> usize {
    let number = s.split(' ').next().expect("invalid file format");
    return number.parse().expect("invalid number format");
}

fn main() {
    let input = read_all_file("../dane/pary.txt");
    let primes = get_primes(111); // numbers are guaranteed to be under 100

    let iter = input.lines()
        .map(number_from_line)
        .filter(|n| n%2 == 0)
        .map(|n| goldbach(n, &primes));

    for (a, b, c) in iter {
        println!("{} {} {}", a, b, c);
    }
}

