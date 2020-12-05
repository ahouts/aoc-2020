use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SEATS_PER_ROW: usize = 8;

fn find_seat(pass: &[u8]) -> (usize, usize) {
    let row_id = &pass[..7];
    let mut row = 0;
    for c in row_id.iter().copied() {
        row = row << 1;
        match c {
            b'F' => (),
            b'B' => row += 1,
            _ => panic!("unknown row descriptor {}", c),
        }
    }

    let seat_id = &pass[7..];
    let mut seat = 0;
    for c in seat_id.iter().copied() {
        seat = seat << 1;
        match c {
            b'L' => (),
            b'R' => seat += 1,
            _ => panic!("unknown seat descriptor {}", c),
        }
    }

    (row, seat)
}

fn seat_id(row: usize, col: usize) -> usize {
    row * SEATS_PER_ROW + col
}

fn get_seats() -> Result<Vec<(usize, usize)>, std::io::Error> {
    BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|r| Ok(find_seat(r?.as_bytes())))
        .collect::<Result<Vec<_>, std::io::Error>>()
}

fn part1(seats: &[(usize, usize)]) -> usize {
    seats
        .iter()
        .copied()
        .map(|(row, seat)| seat_id(row, seat))
        .max()
        .unwrap_or(0)
}

fn part2(seats: &[(usize, usize)]) -> usize {
    let mut filled_seats = [false; 8 * 128];
    seats
        .iter()
        .copied()
        .map(|(row, seat)| seat_id(row, seat))
        .for_each(|seat| filled_seats[seat] = true);
    filled_seats
        .iter()
        .cloned()
        .zip(filled_seats.iter().cloned().enumerate().skip(1))
        .zip(filled_seats.iter().cloned().skip(2))
        .map(|((a, (i, b)), c)| ((a, b, c), i))
        .filter(|((a, b, c), _)| *a && !*b && *c)
        .map(|(_, i)| i)
        .next()
        .unwrap_or(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let seats = get_seats()?;
    println!("part 1: {}", part1(seats.as_ref()));
    println!("part 2: {}", part2(seats.as_ref()));

    Ok(())
}
