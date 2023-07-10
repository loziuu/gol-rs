const OFFSETS: [Offset; 8] = [
    Offset::new(1, 0),
    Offset::new(0, 1),
    Offset::new(1, 1),
    Offset::new(-1, 0),
    Offset::new(0, -1),
    Offset::new(-1, -1),
    Offset::new(1, -1),
    Offset::new(-1, 1),
];

pub const N_GRID: usize = 100;

struct Offset {
    x: i32,
    y: i32,
}

impl Offset {
    const fn new(x: i32, y: i32) -> Offset {
        Offset { x, y }
    }
}

pub struct Life {
    grid: [[bool; N_GRID]; N_GRID],
}

pub struct Cords {
    x: usize,
    y: usize,
}

impl Cords {
    pub fn new(x: usize, y: usize) -> Cords {
        Cords { x, y }
    }
}

impl Life {
    pub fn new(alive: Vec<Cords>) -> Life {
        let mut grid = [[false; N_GRID]; N_GRID];
        for cord in alive {
            grid[cord.x][cord.y] = true;
        }
        Life { grid }
    }

    pub fn current_generation(&self) -> &[[bool; N_GRID]; N_GRID] {
        return &self.grid;
    }

    pub fn next_generation(&mut self) {
        let old = self.grid.clone();
        for x in 0..N_GRID {
            for y in 0..N_GRID {
                verify_cell(&old, &mut self.grid, x, y);
            }
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.grid[x][y] = !self.grid[x][y];
    }
}

fn verify_cell(old: &[[bool;  N_GRID]; N_GRID], grid: &mut [[bool; N_GRID]; N_GRID], x: usize, y: usize) {
    let mut number_of_living_neighbors = 0;
    for offset in OFFSETS {
        let n_x = sum(x, offset.x);
        let n_y = sum(y, offset.y);

        if n_x >= N_GRID as i32 || n_y >= N_GRID as i32 || n_x < 0 || n_y < 0 { 
            continue;
        }

        if old[n_x as usize][n_y as usize] {
            number_of_living_neighbors += 1;
        }
    }
    if number_of_living_neighbors == 3 {
        grid[x][y] = true;
    } else if number_of_living_neighbors == 2 {
    } else {
        grid[x][y] = false;
    }
}

fn sum(a: usize, b: i32) -> i32 {
    a as i32 + b 
}
