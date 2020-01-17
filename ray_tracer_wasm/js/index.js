let CANVAS_WIDTH = 350;
let CANVAS_HEIGHT = 250;

// let CANVAS_WIDTH = 200;
// let CANVAS_HEIGHT = 100;


// Implementation using Web Workers
// window.navigator.hardwareConcurrency will be undefined in Safari
// so default to 1 and essentially run single threaded there
const numCores = window.navigator.hardwareConcurrency || 1;
console.log(`Using ${numCores} web workers`);

const canvas = document.getElementById('raytracer-canvas');
canvas.height = CANVAS_HEIGHT;
canvas.width = CANVAS_WIDTH;
const ctx = canvas.getContext('2d');

function workerMessageEventHandler(e) {
    window.requestAnimationFrame(() => {
        ctx.fillStyle = `rgb(${e.data.r}, ${e.data.g}, ${e.data.b})`;
        ctx.fillRect(e.data.x, e.data.y, 1, 1);
    });
}

// Create a worker for each logical processor available to run threads on the user's computer.
const workers = [];
for (let i = 0; i < numCores; i++) {
    const worker = new Worker("./worker.js");
    worker.addEventListener('message', workerMessageEventHandler);
    workers[i] = worker;
}

const button = document.getElementById("raytracer-button");
button.addEventListener('click', () => {
    button.disabled = true;
    // Send messages to each worker round robin, which for this example is "good enough".
    // In a production system you might want to distribute the work less naively.
    let currentWorker = 0;
    for (let y = 0; y < CANVAS_HEIGHT; y++) {
        for (let x = 0; x < CANVAS_WIDTH; x++) {
            if (currentWorker === workers.length) {
                currentWorker = 0;
            }
            const worker = workers[currentWorker];

            worker.postMessage({x: x, y: y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT});
            currentWorker++;
        }
    }

});


// Alternative implementation without using Web Workers
// import("../pkg/index.js").then(wasm => {
//     const canvas = document.getElementById('raytracer-canvas');
//     canvas.height = CANVAS_HEIGHT;
//     canvas.width = CANVAS_WIDTH;

//     const context = canvas.getContext('2d');

//     const button = document.getElementById("raytracer-button");
//     button.addEventListener('click', () => {
//         button.disabled = true;
//         wasm.draw_single_threaded(context, CANVAS_WIDTH, CANVAS_HEIGHT);
//         button.disabled = false;
//     });

// }).catch(console.error);
