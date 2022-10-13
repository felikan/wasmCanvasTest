import * as wasm from "./pkg/index_bg.wasm";
import('./pkg')
  .catch(console.error);

document.getElementById("button").addEventListener("click", wasm.update)