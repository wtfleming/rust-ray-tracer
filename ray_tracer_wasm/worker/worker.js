let wasmPromise = import("../pkg/index.js");

onmessage = async function(e) {
    // Ensures the WebAssembly code has finished loading before doing anything
    let wasm = await wasmPromise;

    let { width, height, x, y } = e.data;
    let color = wasm.color_at_pixel(width, height, x, y);
    postMessage({x, y, r: color.r, g: color.g, b: color.b});
};
