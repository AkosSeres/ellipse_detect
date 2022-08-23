const rust_imports = import('./pkg');

rust_imports
    .then(m => {
        console.log('Rust import successful');
    })
    .catch(console.error);

function setCanvasSize() {
    let canvas = document.getElementById('canvas');
    canvas.style.width = '100%';
    canvas.style.height = '100%';
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;

    let ctx = canvas.getContext('2d');
    ctx.fillStyle = 'grey';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

window.addEventListener('resize', setCanvasSize);
window.addEventListener('load', setCanvasSize);

let offsetX = 0;
let offsetY = 0;
let scale = 1;

let mouseIsDown = false;

document.getElementById('canvas').addEventListener('mousedown', e => {
    mouseIsDown = true;
});
document.addEventListener('mouseup', e => {
    mouseIsDown = false;
});
document.addEventListener('mousemove', e => {
    if (mouseIsDown) {
        offsetX += e.movementX;
        offsetY += e.movementY;
    }
});
document.getElementById('canvas').addEventListener('wheel', e => {
    let canvas = document.getElementById('canvas');
    let mutliplier = 1 / Math.exp(e.deltaY / 100);
    let minScale = canvas.offsetWidth / image.width / 10;
    let maxScale = canvas.offsetWidth / image.width * 50;
    if (scale * mutliplier > maxScale) {
        mutliplier = maxScale / scale;
    } else if (scale * mutliplier < minScale) {
        mutliplier = minScale / scale;
    }

    offsetX += canvas.offsetWidth / 2 * (1 - mutliplier);
    offsetY += canvas.offsetHeight / 2 * (1 - mutliplier);
    offsetX *= mutliplier;
    offsetY *= mutliplier;
    scale *= mutliplier;
});

let ellipses = [];
let image = new Image();

window.addEventListener('dragover', (e) => e.preventDefault());
window.addEventListener('drop', async (e) => {
    e.preventDefault();
    if (e.dataTransfer.files.length > 0) {
        let textPromises = [];
        let images = [];
        for (let i = 0; i < e.dataTransfer.files.length; i++) {
            let file = e.dataTransfer.files[i];
            if (file.type == 'application/json') {
                textPromises.push(file.text());
            }
            else if (file.type == 'image/png' || file.type == 'image/jpeg' || file.type == 'image/bmp') {
                images.push(file);
            }
        }
        ellipses = (await Promise.all(textPromises)).flatMap(text => JSON.parse(text));
        if (images.length > 0) {
            image = new Image();
            image.src = URL.createObjectURL(images[0]);
        }

        offsetX = 0;
        offsetY = 0;
        scale = 1;
    }
});

const drawFunc = () => {
    let canvas = document.getElementById('canvas');
    /** @type{CanvasRenderingContext2D} */
    let ctx = canvas.getContext('2d');
    ctx.fillStyle = 'grey';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.translate(offsetX, offsetY);
    ctx.scale(scale, scale);

    ctx.drawImage(image, 0, 0);

    ctx.strokeStyle = 'red';
    ellipses.forEach(d => {
        ctx.beginPath();
        ctx.ellipse(d.x, d.y, d.a, d.b, d.theta, 0, 2 * Math.PI);
        ctx.stroke();
    });

    ctx.resetTransform();
};

window.addEventListener('load', () => {
    function update() {
        drawFunc();
        requestAnimationFrame(update);
    }
    requestAnimationFrame(update);
});