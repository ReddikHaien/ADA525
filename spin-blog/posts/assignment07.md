---
title: "Assignment 07: Web server 1"
author: "Fredrik Eide Fluge"
timestamp: "2023-11-14T15:18:00"
---


For our 7th assignment, we where tasked with creating a simple web application with a front- and backend. The frontend should function as an interface towards the backend and the backend should supply the frontend with realtime data. For my web application, i decided to create a simple simulation of the predator and prey formulas:

```
Rabbit(rabbits, foxes) = rabbitBirthRate * rabbits - rabbitConsumeRate * rabbits * foxes
Foxes(rabbits, foxes) = foxBirthRate * rabbits * foxes - foxDeathRate * foxes
```

This system is an implementation of the [Lotka-Volterra equations](https://en.wikipedia.org/wiki/Lotka%E2%80%93Volterra_equations). And describe a predator and prey model where the prey reproduces and gets consumed by the predator, and the predator reproduces based on the amount of prey available and dies of eventually. The model is quite simplified in comparison to the real world, but it does display how the two populations affect each the complex dynamic between the two.

# The server
## Simulation implementation.

The simulation is run at fixed timesteps of 50ms, and is controlled by using javascripts built-in method `setTimeout(callback, ms)`. This allowed me to enque a new iteration of the simulation
To make sure that each iteration of the model is enqueded approximately 20ms, each simulation step is timed. this is simply done by using `performance.now()` before and after executing the main simulation part, and then queuing the next iteration at `50ms - (now - prev)`, for example, if the main simulation part takes 10ms to run, then the next iteration will execute in 40ms and so on. 

It should be noted that the above system is not perfect due to the internal implementation of node's event system. The event system keeps every event in a single queue, meaning that each iteration will need to wait for every event queued before it. This means that a delay of 40ms might be delayed to 45 or 50ms, even though this is not a grave issue, it might introduce some small amount of jitter in the model if there is significant traffic on the event bus.

The result of the simulation is stored in two local variables that is accessed trough a helper method. The simulation requires a very small time step to prevent any erros for building up in the simulation, this was achieved by introducing a second variable `step_rate` which would describe the number of steps to do in each iteration. the step was then used to further scale the deltatime applied and this did stabilize the system quite well

Here is the implementation of the simulation manager:
```js
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
```

## The REST API

The api exposes one folder named static and one api endoint.
The api endpoint is located at `api/data` and returns the current state of the simulation. The simulation state is sent as a json object with the populations of foxes and rabbits.
the api implementation is as follows:
```js
app.get("/api/data", (req, res) => {
    const [rabbits, foxes] = getData();

    res.status(200).send({
        rabbits,
        foxes
    });
});
```

the remaining part of the server logic is serving the static files that represents the client. these are convieniently placed in the `static` folder in the server project.

# The client

## Content
The client consist of 3 files
| file | usage |
| ---  | ---   |
| index.html | the main file |
| main.js    | the main logic file |
| smoothie.js | the chart library in use |

The index.html defines a canvas and loads our main.js script(which in turn loads smoothie.js), while main.js defines the core logic for fetching data and updating the charts.

## Main loop
main.js defines a single method update that acts as the main loop. this method is executed every 50ms by using setTimeout. The method first fethes the simulation values from the api using javascripts `fetch(url)` method. It then parses the content as a json object before updating the chart values. 
The charts are powered by [SmoothieCharts](http://smoothiecharts.org/). Wich is a simple library for setting up streaming charts. There are 2 series defined in main.js a green chart for the foxes and a blue chart for the rabbits. They are both displayed in the same canvas for easier comparison. 

The entire main.js looks like this
```js
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
```

The import statement above was introduced in Ecma 6 (if I remember correctly) and allows referencing script from other scripts without needing to manually reference them in the html file or use 3rd party frameworks for modularization. Further, setTImeout does not handle Promises returned by async functions, the call to settimeout is therefore placed last in the function body. this ensures that any server call is completed before queuing a new call to update.

The final product looks like this:

<img src="images/assignment07/the-charts.png" class="image" />

# Improvements and other possibillities

A simple improvement to the current design would be to introduce the possibility to send a POST request to the server with new configuration values for the simulation. This would simply be a small handler that would call upon a function that would reset the simulation with new values. To make sure the simulation would be updated somewhat atomically, an wrapper object or class could be used that contained the parameters and state of the simulation. 

A second but more complicated improvement could be to make the server entirely stateless and only compute the current state of the simulation based on a query parameter provided by the client. This is possible to achieve due to the predator prey model being a system of ordinary differential equations, and can then be turned into a set of functions over time. With the solved system, the user would only need to send with a query parameter describing the current time in their simulation. Further, with the system being stateless, it would be possible to deploy the application at [Fermyon](https://developer.fermyon.com/cloud/index) which is an serverless microservice deploying service. If I get around to deploying this small app there I'll update the index page. Otherwise, the source code is located at [https://github.com/ReddikHaien/ADA525/tree/main/assignment07](https://github.com/ReddikHaien/ADA525/tree/main/assignment07) and can be executed by doigng `npm i ; npm server.js`