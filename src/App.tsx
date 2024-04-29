import { onMount } from "solid-js";
import { main } from "./webgpu/main";

export const App = () => {
  let canvas: HTMLCanvasElement;
  onMount(async () => {
    main(canvas);
  });

  // @ts-expect-error
  return <canvas ref={canvas} width="512" height="512" />;
};
