use comfy::*;

simple_game!("conjam", GameState, config, setup, update);

static SCREEN_WIDTH: u32 = 512;
static SCREEN_HEIGHT: u32 = 512;

static GRID_SIZE: f32 = 32.0;

fn config(config: GameConfig) -> GameConfig {
    let reso = ResolutionConfig::Physical(SCREEN_WIDTH, SCREEN_HEIGHT);

    GameConfig {
        resolution: reso,
        ..config
    }
}

pub struct GameState {
    grid: Vec<u8>,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        let mut grid = vec![0; (GRID_SIZE * GRID_SIZE) as usize];

        grid[0] = 1;
        grid[128] = 1;

        Self {
            grid,
        }
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    let mut camera = main_camera_mut();

    camera.center = vec2(GRID_SIZE / 2.0, GRID_SIZE / 2.0);
    camera.zoom = GRID_SIZE + 1.0;
}

static CELL_SIZE: f32 = 0.75;

fn update(state: &mut GameState, _c: &mut EngineContext) {
    for (index, c) in state.grid.iter().enumerate() {
        let y = ((index as f32) / GRID_SIZE).floor() + CELL_SIZE / 2.0;
        let x = ((index as f32) % GRID_SIZE) + CELL_SIZE / 2.0;
        let color = if *c == 1 { GREEN } else { RED };

        draw_rect(vec2(x, y), splat(CELL_SIZE), color, 0);
    }

    /*
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            draw_rect(vec2(x as f32, y as f32), splat(1.0), RED, 0);
        }
    }
    */
}

