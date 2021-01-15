use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Place {
    Floor,
    Vacant,
    Occupied,
}
impl Place {
    fn char(&self) -> char {
        match self {
            Place::Occupied => '#',
            Place::Vacant => 'L',
            Place::Floor => '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SeatMap {
    places: Vec<Place>,
    rows: i32,
    cols: i32,
}
impl SeatMap {
    fn parse_from_strings(source: Vec<String>) -> Result<Self, String> {
        let cols = source[0].chars().count();
        let rows = source.len();
        let mut map = SeatMap {
            places: vec![Place::Floor; rows * cols],
            rows: rows as i32,
            cols: cols as i32,
        };

        for (row, line) in source.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                let elem = map.get_mut(row as i32, col as i32).unwrap();
                *elem = match char {
                    'L' => Place::Vacant,
                    '#' => Place::Occupied,
                    '.' => Place::Floor,
                    _ => return Err("Unexpected character".to_string()),
                }
            }
        }
        Ok(map)
    }

    fn print(&self) {
        for r in 0..self.rows {
            let line: String = (0..self.cols)
                .map(|c| self.get(r, c).unwrap().char())
                .collect();
            println!("{}", line);
        }
        println!("---");
    }

    fn get_mut(&mut self, row: i32, col: i32) -> Option<&mut Place> {
        let idx = self.addr(row, col)?;
        Some(&mut self.places[idx])
    }

    fn get(&self, row: i32, col: i32) -> Option<&Place> {
        let idx = self.addr(row, col)?;
        Some(&self.places[idx])
    }

    fn addr(&self, row: i32, col: i32) -> Option<usize> {
        if row < 0 || col < 0 {
            return None;
        }
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(col as usize + row as usize * self.cols as usize)
    }

    fn count_adjacent(&self, row: i32, col: i32, what: &Place) -> usize {
        let r0 = row - 1;
        let r1 = row + 1;
        let c0 = col - 1;
        let c1 = col + 1;
        let mut count = 0;
        for r in r0..=r1 {
            for c in c0..=c1 {
                // ignore self
                if r == row  && c == col {
                    continue;
                }
                // invalid addresses are None
                if let Some(place) = self.get(r, c) {
                    if place == what {
                        count += 1 
                    }
                }
            }
        }
        count
    }

    fn count(&self, what: &Place) -> usize {
        self.places.iter().filter(|&p| p == what).count()
    }

    fn evolve(&self) -> Self {
        let mut map = self.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                let new_place = match self.get(r, c).unwrap() {
                    Place::Floor => Place::Floor,
                    Place::Vacant => {
                        let count_occupied = self.count_adjacent(r, c, &Place::Occupied);
                        if count_occupied == 0 {
                            Place::Occupied
                        } else {
                            Place::Vacant
                        }
                    }
                    Place::Occupied => {
                        let count_occupied = self.count_adjacent(r, c, &Place::Occupied);
                        if count_occupied >= 4 {
                            Place::Vacant
                        } else {
                            Place::Occupied
                        }
                    }
                };
                *map.get_mut(r, c).unwrap() = new_place;
            }
        }
        map
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffered = BufReader::new(File::open("day11/input.txt")?);
    let lines = buffered.lines().map(|l| l.unwrap()).collect();

    let seat_map = SeatMap::parse_from_strings(lines)?;
    seat_map.print();

    let mut map = seat_map.clone();
    loop {
        let new_map = map.evolve();
        new_map.print();
        if new_map == map {
            println!(
                "Complete with {} places occupied",
                new_map.count(&Place::Occupied)
            );
            break;
        }
        map = new_map;
    }

    Ok(())
}
