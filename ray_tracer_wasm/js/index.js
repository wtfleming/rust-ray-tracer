import("../pkg/index.js").then(wasm => {
    let CANVAS_WIDTH = 100;
    let CANVAS_HEIGHT = 50;

    // let CANVAS_WIDTH = 200;
    // let CANVAS_HEIGHT = 100;

    // let CANVAS_WIDTH = 700;
    // let CANVAS_HEIGHT = 500;


    const canvas = document.getElementById('raytracer-canvas');
    canvas.height = CANVAS_HEIGHT;
    canvas.width = CANVAS_WIDTH;

    const context = canvas.getContext('2d');

    const button = document.getElementById("raytracer-button");
    button.addEventListener('click', () => {
        button.disabled = true;
        wasm.draw(context, CANVAS_WIDTH, CANVAS_HEIGHT);
        button.disabled = false;
    });
}).catch(console.error);
