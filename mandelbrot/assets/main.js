const ctx = document.getElementsByTagName("canvas")[0].getContext("2d");
ctx.canvas.width = window.innerWidth;
ctx.canvas.height = window.innerWidth*0.8;
ctx.canvas.addEventListener("resize", () => {
    console.log("resize");
    ctx.canvas.width = window.innerWidth;
    ctx.canvas.height = window.innerWidth*0.8;
});


/**
 * @type {HTMLInputElement}
 */
const scale_inp = document.getElementById("scale");

/**
 * @type {HTMLInputElement}
 */
const offset_x_inp = document.getElementById("offset_x");

/**
 * @type {HTMLInputElement}
 */
const offset_y_inp = document.getElementById("offset_y");

/**
 * @type {HTMLInputElement}
 */
const generate_btn = document.getElementById("generate");

generate_btn.onclick = e => {
    e.preventDefault();
    const scale = Number.parseFloat(scale_inp.value)
    const offset_x = Number.parseFloat(offset_x_inp.value);
    const offset_y = Number.parseFloat(offset_y_inp.value);
    console.log(scale, offset_x, offset_y)
    generate_image(scale, offset_x, offset_y)
        .error(e => console.error(e));
}

const SLICE_WIDTH = 128;
const SLICE_HEIGHT = 128;

async function generate_image(scale, offset_x, offset_y){

    let promises = [];

    const img_w = ctx.canvas.width;
    const img_h = ctx.canvas.height;

    for (let i = 0; i < img_w; i+= SLICE_WIDTH){
        for (let j = 0; j < img_h; j+= SLICE_HEIGHT){
            promises.push(generate_slice(i, j, img_w, img_h));
        }
    }

    await Promise.all(promises);
}

async function generate_slice(x, y, img_w, img_h){

    console.log("generating",x,y);
    var response = await fetch("/api/generate",{
        method: "POST",
        body: JSON.stringify({
            scale,
            offset_x,
            offset_y,
            pixel_x: x,
            pixel_y: y,
            img_w,
            img_h,
            slice_w: SLICE_WIDTH,
            slice_h: SLICE_HEIGHT,
        }),
    }).catch(e => console.error(e));

    if (!response){
        return;
    }

    if (~~(response.status / 100) != 2){
        console.error("failed to load", response.statusText, await response.text());
        return;
    }
    
    const buffer = await response.arrayBuffer();
    const viewer = new DataView(buffer);

    const output = new Uint8ClampedArray(SLICE_WIDTH*SLICE_HEIGHT*4);

    for(let y = 0; y < SLICE_HEIGHT; y++){
        for(let x = 0; x < SLICE_WIDTH; x++){
            const index = x + y*SLICE_WIDTH;
            get_color(viewer.getUint16(index*2,true), index*4,output);
        }
    }

    const imageData = new ImageData(output, SLICE_WIDTH, SLICE_HEIGHT);
    ctx.putImageData(imageData, x, y);
}

function get_color(iteration, index, output){
    const scaled = iteration / 1000;
    const r = Math.pow(scaled * 256, 1.5) % 256;
    const g = Math.pow(scaled*scaled* 256, 1.5) % 256;
    const b = Math.pow(scaled*scaled*scaled* 256, 1.5) % 256;
    
    output[index]   = ~~r;
    output[index+1] = ~~g;
    output[index+2] = ~~b;
    output[index+3] = 255;
}


generate_image(1, 0, 0).then(() => console.log("generated")).catch(e => console.error(e));