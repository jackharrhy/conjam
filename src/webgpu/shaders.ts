import { WORKGROUP_SIZE } from "./consts";

export const cellShader = /* WGSL */ `
struct VertexInput {
  @location(0) pos: vec2f,
  @builtin(instance_index) instance: u32,
};

struct VertexOutput {
  @builtin(position) pos: vec4f,
  @location(0) cell: vec2f,
};

@group(0) @binding(0) var<uniform> grid: vec2f;
@group(0) @binding(1) var<storage> cellState: array<u32>;

@vertex
fn vertexMain(@location(0) pos: vec2f,
              @builtin(instance_index) instance: u32) -> VertexOutput {
  let i = f32(instance);
  let cell = vec2f(i % grid.x, floor(i / grid.x));
  let state = f32(cellState[instance]);

  let cellOffset = cell / grid * 2;
  let gridPos = (pos*state+1) / grid - 1 + cellOffset;

  var output: VertexOutput;
  output.pos = vec4f(gridPos, 0, 1);
  output.cell = cell;
  return output;
}

@fragment
fn fragmentMain(input: VertexOutput) -> @location(0) vec4f {
  return vec4f(0.93, 0.62, 0.38, 0);
}
`;

export const simulationShader = /* WGSL */ `
@group(0) @binding(0) var<uniform> grid: vec2f;

@group(0) @binding(1) var<storage> cellStateIn: array<u32>;
@group(0) @binding(2) var<storage, read_write> cellStateOut: array<u32>;

fn cellIndex(cell: vec2u) -> u32 {
  return (cell.y % u32(grid.y)) * u32(grid.x) +
         (cell.x % u32(grid.x));
}

fn cellValue(x: u32, y: u32) -> u32 {
  return cellStateIn[cellIndex(vec2(x, y))];
}

fn setSand(i: u32) {
  cellStateOut[i] = 1;
}

fn setAir(i: u32) {
  cellStateOut[i] = 0;
}

fn isBottom(y: u32) -> bool {
  return y == 0;
}

fn isTop(y: u32) -> bool {
  return f32(y + 1) == grid.y;
}

@compute @workgroup_size(${WORKGROUP_SIZE}, ${WORKGROUP_SIZE})
fn simulation(@builtin(global_invocation_id) cell: vec3u) {
  let i = cellIndex(cell.xy);
  let value = cellStateIn[i];

  if value == 1 {
    // sand

    // bottom of the world, stay sand
    if isBottom(cell.y) {
      setSand(i);
      return;
    }

    // gravity
    let bottomAir = cellValue(cell.x, cell.y - 1) == 0;
    if bottomAir {
      setAir(i);
      return;
    }

    let topAir = cellValue(cell.x, cell.y + 1) == 0;

    // left diag
    let leftAir = cellValue(cell.x - 1, cell.y) == 0;
    let bottomLeftAir = cellValue(cell.x - 1, cell.y - 1) == 0;

    if topAir && leftAir && bottomLeftAir {
      setAir(i);
      return;
    }

    // right diag
    let rightAir = cellValue(cell.x + 1, cell.y) == 0;
    let bottomRightAir = cellValue(cell.x + 1, cell.y - 1) == 0;

    if topAir && rightAir && bottomRightAir {
      setAir(i);
      return;
    }

    // stay same
    setSand(i);
  } else {
    // top of the world, don't process logic
    if isTop(cell.y) {
      setAir(i);
      return;
    }

    // gravity
    let topSand = cellValue(cell.x, cell.y + 1) == 1;
    if topSand {
      setSand(i);
      return;
    }

    // left diag, as in sand moving left
    let rightSand = cellValue(cell.x + 1, cell.y) == 1;
    let topRightSand = cellValue(cell.x + 1, cell.y + 1) == 1;
    let topRightTopAir = cellValue(cell.x + 1, cell.y + 2) == 0;

    if rightSand && topRightSand && topRightTopAir {
      setSand(i);
      return;
    }

    // right diag, as in sand moving right
    let leftSand = cellValue(cell.x - 1, cell.y) == 1;
    let topLeftSand = cellValue(cell.x - 1, cell.y + 1) == 1;
    let topLeftTopAir = cellValue(cell.x - 1, cell.y + 2) == 0;

    if leftSand && topLeftSand && topLeftTopAir {
      setSand(i);
      return;
    }

    // stay same
    setAir(i);
  }
}
`;
