
pub fn solve(arg: &Vec<String>) -> u16 {
    arg.iter()
        .map(|s| parse_number(s.as_str()) )
        .map(seat_number)
        .max()
        .unwrap()
}

pub fn solve_advanced(arg: &Vec<String>) -> u16 {
    let mut seats: Vec<u16> = arg.iter()
        .map(|s| parse_number(s.as_str()))
        .map(seat_number)
        .collect();
    seats.sort_unstable();

    let (p, _) = seats.iter().clone().zip(seats[1..].iter().clone()).find(|&(p, n)| {
        n - p > 1
    }).unwrap();
    p + 1
}

#[derive(Eq, PartialEq, Debug)]
struct Seat {
    row: u16,
    column: u16
}

fn seat_number(seat: Seat) -> u16 {
    seat.row * 8 + seat.column
}

fn parse_number(encoded: &str) -> Seat {
    let row_encoded = &encoded[0..7];
    let column_encoded = &encoded[7..10];

    let row = row_encoded
        .chars()
        .rev()
        .enumerate()
        .fold(0 as u16, |res, (ind, c)| -> u16 {
            let to_add: u16 = 1 << ind;
            if c == 'F' {
                res
            } else {
                res + to_add
            }
        });

    let column = column_encoded
        .chars()
        .rev()
        .enumerate()
        .fold(0 as u16, |res, (ind, c)| -> u16 {
           let to_add: u16 = 1 << ind;
            if c == 'L' {
                res
            } else {
                res + to_add
            }
        });

    Seat {
        row,
        column
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::{parse_number, Seat};

    #[test]
    fn check_parsing() {
        let number_1 = "BFFFBBFRRR";
        let number_2 = "FFFBBBFRRR";
        let number_3 = "BBFFBBFRLL";

        assert_eq!(parse_number(number_1), Seat { row: 70, column: 7});
        assert_eq!(parse_number(number_2), Seat { row: 14, column: 7});
        assert_eq!(parse_number(number_3), Seat { row: 102, column: 4});
    }
}