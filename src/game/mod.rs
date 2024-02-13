use std::time::Instant;

const OFFSETS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

pub const N_GRID: usize = 100;

pub struct Life {
    grid: [[bool; N_GRID]; N_GRID],
    alive: Vec<(usize, usize)>,
}

pub struct Cords {
    x: usize,
    y: usize,
}

impl Life {
    pub fn new(alive: Vec<Cords>) -> Life {
        let mut grid = [[false; N_GRID]; N_GRID];
        for cord in alive {
            grid[cord.x][cord.y] = true;
        }
        Life {
            grid,
            alive: Vec::new(),
        }
    }

    pub fn current_generation(&self) -> &[[bool; N_GRID]; N_GRID] {
        return &self.grid;
    }

    pub fn next_generation(&mut self) {
        let mut visited = vec![vec![false; N_GRID]; N_GRID];

        let mut x_cords: Vec<usize> = Vec::with_capacity(self.alive.len() * 9);
        let mut y_cords: Vec<usize> = Vec::with_capacity(self.alive.len() * 9);


        for cell in &self.alive {
            x_cords.push(cell.0);
            y_cords.push(cell.1);
            for offset in OFFSETS {
                let dx = sum(cell.0, offset.0);
                let dy = sum(cell.1, offset.1);
                if dx >= 0
                    && dy >= 0
                    && dx < N_GRID as i32
                    && dy < N_GRID as i32
                    && !visited[dx as usize][dy as usize]
                {
                    visited[dx as usize][dy as usize] = true;
                    x_cords.push(dx as usize);
                    y_cords.push(dy as usize);
                }
            }
        }

        let start = Instant::now();
        let checker = GridChecker::new(&x_cords, &y_cords);
        let alive = checker.check(&self.grid);
        println!("Grid checker: {:?}", start.elapsed());


        let mut visited = vec![vec![false; N_GRID]; N_GRID];
        let mut next = Vec::with_capacity(alive.len());
        for cell in alive {
            if visited[cell.x][cell.y] {
                continue;
            }
            visited[cell.x][cell.y] = true;
            self.grid[cell.x][cell.y] = cell.val;
            if self.grid[cell.x][cell.y] {
                next.push((cell.x, cell.y));
            }
        }
        self.alive = next;
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.grid[x][y] = !self.grid[x][y];
        self.alive.push((x, y));
    }
}

fn sum(a: usize, b: i32) -> i32 {
    a as i32 + b
}

#[derive(Debug)]
struct SetCell {
    x: usize,
    y: usize,
    val: bool,
}

struct GridChecker<'a> {
    x: &'a [usize],
    y: &'a [usize],
}

impl<'a> GridChecker<'a> {

    fn new(x: &'a [usize], y: &'a [usize]) -> Self {
        GridChecker { x, y }
    }

    pub fn check(&self, grid: &[[bool; N_GRID]; N_GRID]) -> Vec<SetCell> {
        (0..self.x.len())
            .map(|i| self.verify_cell(self.x[i], self.y[i], grid))
            .collect()
    }

    fn verify_cell(&self, x: usize, y: usize, grid: &[[bool; N_GRID]; N_GRID]) -> SetCell {
        let mut number_of_living_neighbors = 0;
        for offset in OFFSETS {
            let n_x = sum(x, offset.0);
            let n_y = sum(y, offset.1);

            if n_x >= N_GRID as i32 || n_y >= N_GRID as i32 || n_x < 0 || n_y < 0 {
                continue;
            }

            if grid[n_x as usize][n_y as usize] {
                number_of_living_neighbors += 1;
            }
        }
        if number_of_living_neighbors == 3 {
            SetCell { x, y, val: true }
        } else if number_of_living_neighbors == 2 {
            SetCell {
                x,
                y,
                val: grid[x][y],
            }
        } else {
            SetCell { x, y, val: false }
        }
    }


}
