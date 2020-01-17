# rust-ray-tracer

Toy ray tracer implementation using Rust from the book `The Ray Tracer Challenge: A Test-Driven Guide to Your First 3D Renderer` by Jamis Buck. Currently implemented features up to chapter 9.

![raytraced image](/example.png)

To run as a native application:

```
cargo run --release
```

Output will be in the `./renders` directory



To compile to WebAssembly and run in a web browser:


```
cd ray_tracer_wasm
npm install
npm start


npm run build

```
