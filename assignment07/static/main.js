import {SmoothieChart, TimeSeries} from "./smoothie.js";

const canvas = document.getElementsByTagName("canvas")[0];
canvas.width = canvas.clientWidth;
canvas.height = canvas.clientWidth/2;

const smoothie = new SmoothieChart();
smoothie.streamTo(canvas, 200);
const rabbits = new TimeSeries();
const foxes = new TimeSeries();
smoothie.addTimeSeries(rabbits, {strokeStyle:'rgb(0, 0, 255)', fillStyle:'rgba(0, 0, 255, 0.4)', lineWidth:3 });
smoothie.addTimeSeries(foxes, { strokeStyle:'rgb(0, 255, 0)', fillStyle:'rgba(0, 255, 0, 0.4)', lineWidth:3 });

async function update(){
    const response = await fetch("api/data");
    
    const data = await response.json();

    rabbits.append(Date.now(), data.rabbits);
    foxes.append(Date.now(), data.foxes);
    
    setTimeout(update, 50);
}

update();