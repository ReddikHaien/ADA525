const deltaTime = 1/20;

var foxes = 10;
var rabbits = 10;

var rabbitReproductionRate = 1.1;
var foxReproductionRate = 0.1;
var rabbitConsumeRate = 0.4;
var foxDeathRate = 0.4;


var step_rate = 30;

function simulate(){
    var old = performance.now();

    for (let i = 0; i < step_rate; i++){
        var delta_rabbits = rabbits * rabbitReproductionRate - rabbitConsumeRate * foxes * rabbits;
        var delta_foxes = foxes * rabbits * foxReproductionRate - foxDeathRate * foxes;
        
        //console.log(rabbits, foxes);
        rabbits += delta_rabbits * deltaTime/step_rate; 
        foxes += delta_foxes * deltaTime/step_rate;
    }
    
    var elapsed = performance.now() - old;
    var next = Math.max(0.0,(deltaTime * 1000) - elapsed)
    setTimeout(simulate, next);
}

export function start(){
    console.log("simulation starting")
    setTimeout(simulate, deltaTime * 1000);
}

export function getData(){
    return [rabbits, foxes]
}