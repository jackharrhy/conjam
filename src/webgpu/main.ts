import {
  CELL_VERTS,
  GRID_SIZE,
  UPDATE_INTERVAL,
  WORKGROUP_SIZE,
} from "./consts";
import { cellShader, simulationShader } from "./shaders";

const cellStateArray = new Uint32Array(GRID_SIZE * GRID_SIZE);

let device: GPUDevice | null = null;

export const initializeWebGPU = async (cb: () => void) => {
  if (!("gpu" in navigator)) {
    console.error("User agent doesn’t support WebGPU.");
    return false;
  }

  const gpuAdapter = await navigator.gpu.requestAdapter();

  if (!gpuAdapter) {
    console.error("No WebGPU adapters found.");
    return false;
  }

  device = await gpuAdapter.requestDevice();

  device.lost.then((info) => {
    console.error(`WebGPU device was lost: ${info.message}`);

    device = null;

    if (info.reason != "destroyed") {
      initializeWebGPU(cb);
    }
  });

  cb();

  return true;
};

export const setupCanvas = async (canvas: HTMLCanvasElement) => {
  if (device === null) {
    throw new Error("GPUDevice not initialized");
  }

  canvas.width = GRID_SIZE;
  canvas.height = GRID_SIZE;

  const context = canvas.getContext("webgpu")!;

  const canvasFormat = navigator.gpu.getPreferredCanvasFormat();
  context.configure({
    device: device,
    format: canvasFormat,
  });

  const vertexBuffer = device.createBuffer({
    label: "Cell vertices",
    size: CELL_VERTS.byteLength,
    usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
  });

  device.queue.writeBuffer(vertexBuffer, 0, CELL_VERTS);

  const cellStateStorage = [
    device.createBuffer({
      label: "Cell State A",
      size: cellStateArray.byteLength,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    }),
    device.createBuffer({
      label: "Cell State B",
      size: cellStateArray.byteLength,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    }),
  ];

  for (let i = 0; i < cellStateArray.length; ++i) {
    cellStateArray[i] = Math.random() > 0.6 ? 1 : 0;
  }

  cellStateArray[GRID_SIZE * 0 + 3] = 1;
  cellStateArray[GRID_SIZE * 0 + 4] = 1;
  cellStateArray[GRID_SIZE * 0 + 5] = 1;

  cellStateArray[GRID_SIZE * 1 + 4] = 1;
  cellStateArray[GRID_SIZE * 2 + 4] = 1;
  cellStateArray[GRID_SIZE * 3 + 4] = 1;
  cellStateArray[GRID_SIZE * 4 + 4] = 1;
  cellStateArray[GRID_SIZE * 5 + 4] = 1;
  cellStateArray[GRID_SIZE * 6 + 4] = 1;

  device.queue.writeBuffer(cellStateStorage[0], 0, cellStateArray);
  device.queue.writeBuffer(cellStateStorage[1], 0, cellStateArray);

  const vertexBufferLayout: GPUVertexBufferLayout = {
    arrayStride: 8,
    attributes: [
      {
        format: "float32x2",
        offset: 0,
        shaderLocation: 0,
      },
    ],
  };

  const gridSizeUniformArray = new Float32Array([GRID_SIZE, GRID_SIZE]);
  const gridSizeUniformBuffer = device.createBuffer({
    label: "Grid Uniforms",
    size: gridSizeUniformArray.byteLength,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  device.queue.writeBuffer(gridSizeUniformBuffer, 0, gridSizeUniformArray);

  const cellShaderModule = device.createShaderModule({
    label: "Cell shader",
    code: cellShader,
  });

  const simulationShaderModule = device.createShaderModule({
    label: "Simulation shader",
    code: simulationShader,
  });

  const bindGroupLayout = device.createBindGroupLayout({
    label: "Cell Bind Group Layout",
    entries: [
      {
        binding: 0,
        visibility:
          GPUShaderStage.VERTEX |
          GPUShaderStage.FRAGMENT |
          GPUShaderStage.COMPUTE,
        buffer: {},
      },
      {
        binding: 1,
        visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE,
        buffer: { type: "read-only-storage" },
      },
      {
        binding: 2,
        visibility: GPUShaderStage.COMPUTE,
        buffer: { type: "storage" },
      },
    ],
  });

  const bindGroups = [
    device.createBindGroup({
      label: "Cell renderer bind group A",
      layout: bindGroupLayout,
      entries: [
        {
          binding: 0,
          resource: { buffer: gridSizeUniformBuffer },
        },
        {
          binding: 1,
          resource: { buffer: cellStateStorage[0] },
        },
        {
          binding: 2,
          resource: { buffer: cellStateStorage[1] },
        },
      ],
    }),
    device.createBindGroup({
      label: "Cell renderer bind group B",
      layout: bindGroupLayout,

      entries: [
        {
          binding: 0,
          resource: { buffer: gridSizeUniformBuffer },
        },
        {
          binding: 1,
          resource: { buffer: cellStateStorage[1] },
        },
        {
          binding: 2,
          resource: { buffer: cellStateStorage[0] },
        },
      ],
    }),
  ];

  const pipelineLayout = device.createPipelineLayout({
    label: "Cell Pipeline Layout",
    bindGroupLayouts: [bindGroupLayout],
  });

  const cellPipeline = device.createRenderPipeline({
    label: "Cell pipeline",
    layout: pipelineLayout,
    vertex: {
      module: cellShaderModule,
      entryPoint: "vertexMain",
      buffers: [vertexBufferLayout],
    },
    fragment: {
      module: cellShaderModule,
      entryPoint: "fragmentMain",
      targets: [
        {
          format: canvasFormat,
        },
      ],
    },
  });

  const simulationPipeline = device.createComputePipeline({
    label: "Simulation pipeline",
    layout: pipelineLayout,
    compute: {
      module: simulationShaderModule,
      entryPoint: "simulation",
    },
  });

  const workgroupCount = Math.ceil(GRID_SIZE / WORKGROUP_SIZE);

  let intervalId: number;
  let step = 0;
  function updateGrid() {
    if (device === null) {
      clearInterval(intervalId);
      return;
    }

    const encoder = device.createCommandEncoder();
    const computePass = encoder.beginComputePass();

    const currentBindGroup = bindGroups[step % 2];

    computePass.setPipeline(simulationPipeline);
    computePass.setBindGroup(0, currentBindGroup);

    computePass.dispatchWorkgroups(workgroupCount, workgroupCount);

    computePass.end();

    step++;

    const pass = encoder.beginRenderPass({
      colorAttachments: [
        {
          view: context.getCurrentTexture().createView(),
          loadOp: "clear",
          clearValue: { r: 1, g: 1, b: 1, a: 1.0 },
          storeOp: "store",
        },
      ],
    });

    pass.setPipeline(cellPipeline);

    pass.setBindGroup(0, currentBindGroup);
    pass.setVertexBuffer(0, vertexBuffer);
    pass.draw(CELL_VERTS.length / 2, GRID_SIZE * GRID_SIZE);

    pass.end();

    device.queue.submit([encoder.finish()]);
  }

  intervalId = setInterval(updateGrid, UPDATE_INTERVAL);
  updateGrid();
};
