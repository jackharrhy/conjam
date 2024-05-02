import { onMount } from "solid-js";
import { initializeWebGPU, setupCanvas } from "./webgpu/main";

export const App = () => {
  let canvas: HTMLCanvasElement;
  onMount(async () => {
    initializeWebGPU(() => {
      setupCanvas(canvas);
    });
  });

  // @ts-expect-error
  return <canvas ref={canvas} />;
};
