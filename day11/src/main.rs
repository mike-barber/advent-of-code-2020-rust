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
    rows: usize,
    cols: usize,
}
impl SeatMap {
    fn parse_from_strings(source: Vec<String>) -> Result<Self, String> {
        let cols = source[0].chars().count();
        let rows = source.len();
        let mut map = SeatMap {
            places: vec![Place::Floor; rows * cols],
            rows,
            cols,
        };

        for (row, line) in source.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                let elem = map.get_mut(row, col);
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
            let line: String = (0..self.cols).map(|c| self.get(r, c).char()).collect();
            println!("{}", line);
        }
        println!("---");
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Place {
        let idx = self.addr(row, col);
        &mut self.places[idx]
    }

    fn get(&self, row: usize, col: usize) -> &Place {
        let idx = self.addr(row, col);
        &self.places[idx]
    }

    fn addr(&self, row: usize, col: usize) -> usize {
        col + row * self.cols
    }

    fn count_adjacent(&self, row: usize, col: usize, what: &Place) -> usize {
        let r0 = row.checked_sub(1).unwrap_or(0);
        let r1 = (row + 1).min(self.rows - 1);
        let c0 = col.checked_sub(1).unwrap_or(0);
        let c1 = (col + 1).min(self.cols - 1);
        let mut count = 0;
        for r in r0..=r1 {
            for c in c0..=c1 {
                // ignore self
                if r==row && c == col {
                    continue;
                }
                if self.get(r, c) == what {
                    count += 1
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
                let new_place = match self.get(r, c) {
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
                *map.get_mut(r, c) = new_place;
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
