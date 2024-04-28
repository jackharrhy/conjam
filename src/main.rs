use comfy::*;

simple_game!("conjam", GameState, config, setup, update);

static SCREEN_WIDTH: u32 = 512;
static SCREEN_HEIGHT: u32 = 512;

static CELL_SIZE: f32 = 1.0;
static GRID_SIZE: i32 = 256;

fn config(config: GameConfig) -> GameConfig {
    let reso = ResolutionConfig::Physical(SCREEN_WIDTH, SCREEN_HEIGHT);

    GameConfig {
        resolution: reso,
        ..config
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Air,
    Sand,
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Cell>,
    size: i32,
}

fn x_y_to_index(grid: &Grid, x: i32, y: i32) -> Option<usize> {
    if x < 0 || x >= grid.size || y < 0 || y >= grid.size {
        None
    } else {
        Some((y * grid.size + x) as usize)
    }
}

fn get(grid: &Grid, x: i32, y: i32) -> Option<&Cell> {
    match x_y_to_index(grid, x, y) {
        Some(index) => grid.cells.get(index),
        None => None,
    }
}

fn set(grid: &mut Grid, x: i32, y: i32, value: Cell) {
    match x_y_to_index(grid, x, y) {
        Some(index) => grid.cells[index] = value,
        None => {}
    }
}

fn draw_point(grid: &mut Grid, x: i32, y: i32, size: i32) {
    for dx in -size..=size {
        for dy in -size..=size {
            set(grid, x + dx, y + dy, Cell::Sand);
        }
    }
}

fn step(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new(grid.size);

    for x in 0..grid.size {
        for y in 0..grid.size {
            let cell = get(grid, x, y).unwrap();
            match cell {
                Cell::Air => {}
                Cell::Sand => {
                    if y == 0 {
                        set(&mut new_grid, x, y, Cell::Sand);
                        continue;
                    }

                    let bellow = get(grid, x, y - 1);

                    if bellow == Some(&Cell::Air) {
                        set(&mut new_grid, x, y - 1, Cell::Sand);
                    } else {
                        set(&mut new_grid, x, y, Cell::Sand);
                    }
                }
            }
        }
    }

    new_grid
}

fn fill_grid_randomly(grid: &mut Grid, choices: [Cell; 2]) {
    let mut rng = rand::thread_rng();

    let cells: Vec<Cell> = (0..(grid.size * grid.size))
        .map(|_| choices.choose(&mut rng).unwrap())
        .cloned()
        .collect();

    grid.cells = cells;
}

impl Grid {
    fn new(size: i32) -> Self {
        let cells = vec![Cell::Air; (size * size) as usize];
        Self { cells, size }
    }
}

pub struct GameState {
    grid: Grid,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        let mut grid = Grid::new(GRID_SIZE);

        let choices = [Cell::Air, Cell::Sand];
        fill_grid_randomly(&mut grid, choices);

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
        if let &Cell::Sand = c {
            let y = ((index as f32) / size).floor() + CELL_SIZE / 2.0;
            let x = ((index as f32) % size) + CELL_SIZE / 2.0;

            draw_rect(vec2(x, y), splat(CELL_SIZE), ORANGE_RED, 0);
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        let mouse = mouse_world().floor();
        draw_point(&mut state.grid, mouse.x as i32, mouse.y as i32, 4);
    }

    state.grid = step(&state.grid);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sets_cells_correctly() {
        use crate::{get, set, Cell, Grid};

        let mut grid = Grid::new(16);

        set(&mut grid, 0, 0, Cell::Sand);
        let val = get(&grid, 0, 0);

        assert_eq!(val, Some(&Cell::Sand));
    }
}
