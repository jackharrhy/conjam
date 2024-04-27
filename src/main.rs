use comfy::*;

simple_game!("conjam", GameState, config, setup, update);

static SCREEN_WIDTH: u32 = 512;
static SCREEN_HEIGHT: u32 = 512;

static CELL_SIZE: f32 = 1.0;

fn config(config: GameConfig) -> GameConfig {
    let reso = ResolutionConfig::Physical(SCREEN_WIDTH, SCREEN_HEIGHT);

    GameConfig {
        resolution: reso,
        ..config
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Dead,
    Alive(Color),
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Cell>,
    size: i32,
}

fn x_y_to_index(grid: &Grid, x: i32, y: i32) -> usize {
    let x = x.rem_euclid(grid.size);
    let y = y.rem_euclid(grid.size);

    (y * grid.size + x) as usize
}

fn get(grid: &Grid, x: i32, y: i32) -> Option<&Cell> {
    let index = x_y_to_index(grid, x, y);
    grid.cells.get(index)
}

fn set(grid: &mut Grid, x: i32, y: i32, value: Cell) {
    let index = x_y_to_index(grid, x, y);
    grid.cells[index] = value;
}

fn get_alive_neighbor_count(grid: &Grid, x: i32, y: i32) -> u8 {
    let mut count = 0;
    for xd in -1..=1 {
        for yd in -1..=1 {
            if xd == 0 && yd == 0 {
                continue;
            }

            if let Some(Cell::Alive(_c)) = get(grid, x + xd, y + yd) {
                count += 1;
            }
        }
    }

    return count;
}

fn step(grid: &Grid) -> Grid {
    let mut new_grid = grid.clone();

    for x in 0..grid.size {
        for y in 0..grid.size {
            let count = get_alive_neighbor_count(grid, x, y);

            match get(grid, x, y) {
                Some(&Cell::Alive(_c)) => {
                    if count < 2 || count > 3 {
                        set(&mut new_grid, x, y, Cell::Dead);
                    }
                }
                Some(&Cell::Dead) => {
                    if count == 3 {
                        set(&mut new_grid, x, y, Cell::Alive(PINK));
                    }
                }
                None => panic!("there should never be a missing cell!"),
            }
        }
    }

    new_grid
}

impl Grid {
    fn new(size: i32) -> Self {
        let cells = vec![Cell::Dead; (size * size) as usize];
        Self { cells, size }
    }
}

pub struct GameState {
    grid: Grid,
}

fn setup_glider(grid: &mut Grid, x: i32, y: i32) {
    set(grid, x + 0, y + 0, Cell::Alive(PINK));
    set(grid, x + 1, y + 0, Cell::Alive(PINK));
    set(grid, x + 2, y + 0, Cell::Alive(PINK));
    set(grid, x + 2, y + 1, Cell::Alive(PINK));
    set(grid, x + 1, y + 2, Cell::Alive(PINK));
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        let mut grid = Grid::new(16);

        setup_glider(&mut grid, 1, 1);
        setup_glider(&mut grid, 6, 1);
        setup_glider(&mut grid, 11, 1);

        Self { grid }
    }
}

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    let mut camera = main_camera_mut();

    let center = (state.grid.size as f32) / 2.0;
    camera.center = vec2(center, center);
    camera.zoom = state.grid.size as f32;
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    let cells = &state.grid.cells;
    let size = state.grid.size as f32;
    draw_rect(splat(size / 2.0), splat(size), BLACK, 0);

    for (index, c) in cells.iter().enumerate() {
        if let &Cell::Alive(color) = c {
            let y = ((index as f32) / size).floor() + CELL_SIZE / 2.0;
            let x = ((index as f32) % size) + CELL_SIZE / 2.0;

            draw_rect(vec2(x, y), splat(CELL_SIZE), color, 0);
        }
    }

    if cooldowns().can_use("step-simulation", 0.1) {
        state.grid = step(&state.grid);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sets_cells_correctly() {
        use crate::{get, set, Cell, Grid};
        use comfy::PINK;

        let mut grid = Grid::new(16);

        set(&mut grid, 0, 0, Cell::Alive(PINK));
        let val = get(&grid, 0, 0);

        assert_eq!(val, Some(&Cell::Alive(PINK)));
    }
}
