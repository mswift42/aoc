use std::fs;
use std::str;
use std::io::{BufReader, BufRead, Lines};

fn main() {
    println!("Hello, world!");
    let sizes = read_file("1ainput.txt");
    println!("{:}", sizes);
}

fn fuel_required(mass: usize) -> usize {
    let divided = mass / 3;
    if divided > 2 {
        divided - 2
    } else {
        0
    }
}

fn more_fuel_required(mass: usize) -> usize {
    let fr = fuel_required(mass);
    if fr == 0 {
        fr
    } else {
         fr + more_fuel_required(fr)
    }
}

fn parse_string(inp: &str) -> usize {
    inp.parse().unwrap()
}

fn read_file(filename: &str) -> usize {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let masses= reader.lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<usize>())
        .filter_map(Result::ok)
        .map(|i| more_fuel_required(i))
        .sum();
    masses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn test_more_fuel_required() {
        assert_eq!(more_fuel_required(14), 2);
        assert_eq!(more_fuel_required(1969), 966 );
        assert_eq!(more_fuel_required(100756), 50346);
    }
}
