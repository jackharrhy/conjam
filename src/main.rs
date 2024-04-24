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

        Self {
            grid,
        }
    }
}

static CELL_SIZE: f32 = 1.0;

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    let mut camera = main_camera_mut();

    let center = GRID_SIZE / 2.0;
    camera.center = vec2(center, center);
    camera.zoom = GRID_SIZE;
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    draw_rect(splat(GRID_SIZE / 2.0), splat(GRID_SIZE), ORANGE, 0);

    for (index, c) in state.grid.iter().enumerate() {
        let y = ((index as f32) / GRID_SIZE).floor() + CELL_SIZE / 2.0;
        let x = ((index as f32) % GRID_SIZE) + CELL_SIZE / 2.0;
        let color = if *c == 1 { GREEN } else { RED };

        draw_rect(vec2(x, y), splat(CELL_SIZE), color, 0);
    }
}

