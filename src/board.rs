//use rand;

use std::collections::{HashMap, HashSet};


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
}

pub struct LifeBoard {
    alive: HashSet<Cell>,
    neighbors: HashMap<Cell, u8>,
}


impl Cell {
    fn new(x: i64, y: i64) -> Self {
        Cell {
            x: x,
            y: y,
        }
    }
}

impl LifeBoard {
    pub fn new() -> Self {
        LifeBoard {
            alive: HashSet::new(),
            neighbors: HashMap::new(),
        }
    }

    fn get_neighbors(&self, x: i64, y: i64) -> [Cell; 8] {
        let mut neighbors = [Cell::new(0, 0); 8];
        let mut next_neighbor = 0;

        for j in (y - 1)..(y + 2) {
            for i in (x - 1)..(x + 2) {
                if x != i || y != j {
                    neighbors[next_neighbor] = Cell {
                        x: i,
                        y: j,
                    };
                    next_neighbor += 1;
                }
            }
        }
        neighbors
    }

    fn update_neighbors(&mut self, x: i64, y: i64, value: bool) {
        let neighbors = self.get_neighbors(x, y);
        let amount: i8 = if value { 1 } else { -1 };
        for cell in &neighbors {
            let neighbor_count = self.neighbors.get(&cell).cloned().unwrap_or(0);
            let new_neighbor_count = (neighbor_count as i8 + amount) as u8;

            if new_neighbor_count != 0 {
                self.neighbors.insert(cell.clone(), new_neighbor_count);
            } else {
                self.neighbors.remove(&cell);
            }
        }
    }

    //fn get_internal(&self, x: i64, y: i64) -> InternalCell {
    //    let index = (y * self.width) + x;
    //    self.inner[index]
    //}

    pub fn get(&self, x: i64, y: i64) -> bool {
        self.alive.contains(&Cell::new(x, y))
    }

    pub fn get_neighbor_count(&self, x: i64, y: i64) -> u8 {
        self.neighbors.get(&Cell::new(x, y)).cloned().unwrap_or(0)
    }

    pub fn set(&mut self, x: i64, y: i64, value: bool) {
        // Get current value, if changing, update neighbors
        if value {
            if self.alive.insert(Cell::new(x, y)) {
                self.update_neighbors(x, y, value);
            }
        } else {
            if self.alive.remove(&Cell::new(x, y)) {
                self.update_neighbors(x, y, value);
            }
        }
    }

    pub fn clear(&mut self) {
        self.alive.clear();
        self.neighbors.clear();
    }

    //pub fn randomize(&mut self) {
    //    for i in 0..self.inner.len() {
    //        let x = i % self.width;
    //        let y = i / self.width;
    //        self.set(x, y, rand::random());
    //    }
    //}

    pub fn step(&mut self) {
        let mut killed = Vec::new();
        let mut spawned = Vec::new();

        for cell in &self.alive {
            let neighbor_count = self.neighbors.get(cell).cloned().unwrap_or(0);
            if neighbor_count < 2 || neighbor_count > 3 {
                killed.push(cell.clone());
            }
        }

        for (cell, &neighbor_count) in &self.neighbors {
            if neighbor_count == 3 && !self.alive.contains(cell) {
                spawned.push(cell.clone());
            }
        }

        for cell in killed {
            self.set(cell.x, cell.y, false);
        }
        for cell in spawned {
            self.set(cell.x, cell.y, true);
        }
    }

    pub fn iter_live_cells<'a>(&'a self) -> impl Iterator<Item=&'a Cell> + 'a {
        self.alive.iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn set_get() {
        let mut board = LifeBoard::new();

        board.set(1, 1, true);

        assert_eq!(board.get(1, 1), true);
    }

    #[test]
    fn step_one_cell() {
        let mut board = LifeBoard::new();

        board.set(1, 1, true);

        assert_eq!(board.get(1, 1), true);

        board.step();

        assert_eq!(board.get(1, 1), false);
    }

    #[test]
    fn get_neighbors_0_0() {
        let board = LifeBoard::new();

        let neighbors = board.get_neighbors(0, 0);

        assert!(neighbors.contains(&Cell::new(-1, -1)));
        assert!(neighbors.contains(&Cell::new(-1, 0)));
        assert!(neighbors.contains(&Cell::new(-1, 1)));
        assert!(neighbors.contains(&Cell::new(0, -1)));
        assert!(neighbors.contains(&Cell::new(0, 1)));
        assert!(neighbors.contains(&Cell::new(1, -1)));
        assert!(neighbors.contains(&Cell::new(1, 0)));
        assert!(neighbors.contains(&Cell::new(1, 1)));

        assert!(!neighbors.contains(&Cell::new(0, 0)));
    }

    #[test]
    fn update_neighbors_increment() {
        let mut board = LifeBoard::new();

        board.set(1, 1, true);

        assert_eq!(board.get_neighbor_count(0, 0), 1);
        assert_eq!(board.get_neighbor_count(1, 0), 1);
        assert_eq!(board.get_neighbor_count(2, 0), 1);
        assert_eq!(board.get_neighbor_count(0, 1), 1);
        assert_eq!(board.get_neighbor_count(1, 1), 0);
        assert_eq!(board.get_neighbor_count(2, 1), 1);
        assert_eq!(board.get_neighbor_count(0, 2), 1);
        assert_eq!(board.get_neighbor_count(1, 2), 1);
        assert_eq!(board.get_neighbor_count(2, 2), 1);
    }

    #[test]
    fn update_neighbors_increment_decrement() {
        let mut board = LifeBoard::new();

        board.set(1, 1, true);

        assert_eq!(board.get_neighbor_count(0, 0), 1);
        assert_eq!(board.get_neighbor_count(1, 0), 1);
        assert_eq!(board.get_neighbor_count(2, 0), 1);
        assert_eq!(board.get_neighbor_count(0, 1), 1);
        assert_eq!(board.get_neighbor_count(1, 1), 0);
        assert_eq!(board.get_neighbor_count(2, 1), 1);
        assert_eq!(board.get_neighbor_count(0, 2), 1);
        assert_eq!(board.get_neighbor_count(1, 2), 1);
        assert_eq!(board.get_neighbor_count(2, 2), 1);

        board.set(1, 1, false);

        assert_eq!(board.get_neighbor_count(0, 0), 0);
        assert_eq!(board.get_neighbor_count(1, 0), 0);
        assert_eq!(board.get_neighbor_count(2, 0), 0);
        assert_eq!(board.get_neighbor_count(0, 1), 0);
        assert_eq!(board.get_neighbor_count(1, 1), 0);
        assert_eq!(board.get_neighbor_count(2, 1), 0);
        assert_eq!(board.get_neighbor_count(0, 2), 0);
        assert_eq!(board.get_neighbor_count(1, 2), 0);
        assert_eq!(board.get_neighbor_count(2, 2), 0);
    }

    #[test]
    fn iter_live_cells() {
        let mut board = LifeBoard::new();

        board.set(1, 1, true);

        let mut board_iter = board.iter_live_cells();

        assert_eq!(board_iter.next(), Some(&Cell {
            x: 1,
            y: 1,
        }));
    }

    #[bench]
    fn bench_empty_step_once(b: &mut Bencher) {
        let mut board = LifeBoard::new();

        b.iter(|| board.step());
    }

    #[bench]
    fn bench_glider_step_100(b: &mut Bencher) {
        let mut board = LifeBoard::new();
        board.set(0, 0, true);
        board.set(1, 0, true);
        board.set(2, 0, true);
        board.set(2, 1, true);
        board.set(1, 2, true);

        b.iter(|| {
            for _ in 0..100 {
                board.step();
            }
        });
    }

    #[bench]
    fn bench_acorn_step_100(b: &mut Bencher) {
        let mut board = LifeBoard::new();
        board.set(0, 0, true);
        board.set(1, 0, true);
        board.set(1, 2, true);
        board.set(3, 1, true);
        board.set(4, 0, true);
        board.set(5, 0, true);
        board.set(6, 0, true);

        b.iter(|| {
            for _ in 0..100 {
                board.step();
            }
        });
    }

    #[bench]
    fn bench_100x100_dense_step_once(b: &mut Bencher) {
        let mut board = LifeBoard::new();

        for j in 0..100 {
            for i in 0..100 {
                board.set(i, j, true);
            }
        }

        b.iter(|| {
            board.step();
        });
    }

    // TODO: Glider gun benchmark?
}
