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