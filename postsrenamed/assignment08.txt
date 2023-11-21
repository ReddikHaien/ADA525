---
title: "Assignment 08: Web server 2"
author: "Fredrik Eide Fluge"
timestamp: "2023-11-21T14:52:00"
---

For our 8th assignment, we were tasked with creating a server application that communicates with an external microchip. For this, I was challenged with streaming the webcam from a connected client to the server, and then stream the webcamdata to the microchip, the microchip is then tasked with displaying the webcam on a LED screen.

<img src="images/assignment08/assignment08-flowchart.png" class="image" />

# The hardware
## Led screen
The LED screen I was given was an IS31FL3731 display. This is an 15x7 LED screen that's interfaced with trough I2C. I received the screen without the pins soldered, and my lack in soldering skill did put a pause on the development, but thanks to Daniel's amazing skills in soldering, I got the piece finished. 
For the microcontroller I used an Arduino UNO R3. This chip is easy to work with and comes with a lot of built in functionality. 

The arduino supports I2C communication trough its A4(SLC) and A5(SDA) pins. This allowed for easy assembly and the circuitry looked like this:
<img src="images/assignment08/assignment08-circuit.png" class="image" />

After assembling the arduino and LED screen, I was left with the following circuit:


Adafruit has made a library for the IS31FL373, this made it easy to interface with the compontent from the Arduino, and the display could the written to by using `Adafruit_IS31FL3731_Wing::setPixel(int x, int y, uint8_t intensity)`. The led matrix supported intensities ranging from 0 (off) to 255 (full strength). I used this to my advantage by displaying the video feed as a gray scale image.

### Serial data
To simplify the code I decided to perform the scaling and recoloring in the client and let the server be a passtrough to the Arduino. The benefit of doing the processing on the client side, is that there is less traffic between the server and client, thereby allowing for faster messaging. Another benefit of doing it on the client was to easily visualize the data sent to the server by drawing the grayscale image to a canvas.

When the server recieves an `"image"` message from the client, it starts of by first checking if it's done with the previous message. Ff it's still working then the current message is discarded, otherwise the data is written to the server and a timeout of 100 ms is initiated, this timeout is then responsible for signaling that the message is complete and a new message may be processed.
The reason there is delay added after a message is posted is to allow the arduino to complete a frame before a new one is sent. I did try to use 20 ms or no delay but this caused the arduino to receive to many frames at once, creating a slow motion movie of the web camera. The 100 ms delay seems to solve the problem of frames backing up on the arduino, and it seems to be running at full speed in comparison to the web camera. A better solution for this method is discussed in "Improvements"

#### Implementation
Adafruit has made a library for interfacing with the IS31FL373. This library allows an easy setup by first declaring a variable of type `Adafruit_IS31FL3731_Wing` and then calling methods on the created object. For this assignment I used `Adafruit_IS31FL3731_Wing::begin` for initialization and `Adafruit_IS31FL3731_Wing::draw_pixel()` for changing the screen.

Each message sent from the server is annotated with an 0xFF byte. This is to signalize the arduino that the following message is an `"image"` message.
The arduino starts by polling for messages by checking if there are bytes available for reading, and then reading a byte from the Serial port. If the byte matches 0xFF, then it triggers the `draw_screen()` method.

The `draw_screen()` method starts off by creating an array of 105 bytes(15*7). This buffer is then filled by calling `Serial.readBytes(buffer, 105)`. The serial method returns an integer that represents the number of bytes read from the Serial. If the Arduino read less than 105 bytes, then the LED screen isn't updated and the Arduino will poll for a new message. Otherwise if the arduino managed to pull 105 bytes of data, it can draw the screen. This is simply done by using 2 for loops that iterates over every pixel. Each pixel value from the Serial buffer is first mapped from a full byte down to 20 values, this mapped value is then used to get a predefined value from a strength array. This mapping is used to better differerentiate between pixel values. The pixel is finally drawn by calling `ledmatrix.drawpixel(x, y, color)`. 

The full code for the Arduino is provided here:
```c
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_IS31FL3731.h>

Adafruit_IS31FL3731_Wing ledmatrix = Adafruit_IS31FL3731_Wing();

uint8_t strength[] = {1, 2, 3, 4, 6, 8, 10, 15, 20, 30, 40, 60, 80, 100, 120, 140, 170, 200, 230, 255};

void setup() {
  Serial.begin(9600);
  Serial.println("ISSI swirl test");

  if (! ledmatrix.begin()) {
    Serial.println("IS31 not found");
    while (1);
  }
  Serial.println("IS31 found!");
}

  void loop() {
  if (Serial.available() > 0){
    int a = Serial.read();
    if (a == 255){
     draw_screen();
    }
  }
}

void draw_screen(){
    uint8_t buffer[105];
    size_t bytes = Serial.readBytes(buffer, 105);
    if (bytes < 105)
      return;
    
    for (uint8_t y = 0; y < 9; y++)
      for (uint8_t x = 0; x <= 14; x++){
        uint8_t cx = x > 14 ? 14 : x;
        uint8_t cy = y > 7 ? 7 : y;
        uint8_t color = strength[map(buffer[x + y * 15], 0, 255, 0, 20)];
        ledmatrix.drawPixel(x, y, color);
      }
}
```

# Client and Server
## The server
For setting up the server i used NodeJs with the libraries http, socketio and serialport. Http is a simple server library for setting up a http server that can be used for serving api endpoints.

### Static files
The http library does not have a dedicated static file server(that I could find), so for this project i made my own implementation of it. It's not the most secure static file server, but it handles general files and their MIME types. 

```javascript
const server = createServer((req, res) => {
    var url = req.url ?? "/";

    //simple sanitizatin to remove .. and : from urls
    url = url.replace(/\.\.|\:/,"");

    //redirection
    if (url == "/"){
        url = "/index.html";
    }

    const ext = url.substring(url.lastIndexOf("."));
    
    // pattern matching with ternaries ;)
    const mediatype = 
        ext == ".html" ? "text/html"
       :ext == ".js"   ? "application/javascript"
       :                 "text/plain";


    // loads the file to mem and sends it(would be faster to stream it directly from disk)
    const path = join(cwd(), "static", ...url.split("/"));
    if (existsSync(path)){
        res.writeHead(200,{
            "Content-type": mediatype
        }).end(readFileSync(path));
    }
    else{
        console.log("missing url",req.url);
        res.writeHead(404).end();
    }
});
```

### Websockets

To make life simpler I decided to use websockets since they allow for arbitrary data to be sent between the server and the client. For managing the websockets I used socket.io. socket.io is a simple library that allows for konfiguring the sockets trough callbacks, and the api is shared on the client and server side, allowing for an easy development sycle.

As discussed above, each time a `"image"` message is recieved, and the previous message is done processing, the server starts sending the data to the Arduino. The library Serialport is used, which allows for communication over one of the Serial ports on the computer. It has simple api for writing data to the arduino, and to respond to incoming data.
```js
import { SerialPort } from "serialport";


const port = new SerialPort({
    path: "COM3",
    baudRate: 9600
});

port.on("readable", e => console.log(new TextDecoder().decode(e)));
port.on("error",e => console.error(e));

const HEADER_DATA = new Uint8Array([
    0xff
]);

let resolved = true;

/**
 * 
 * @param {Buffer} data 
 */
export default function onImageData(data){
    if (resolved){
        resolved = false;        
        port.write(HEADER_DATA);
        port.write(data);
        port.drain();
        setTimeout(() => {
            resolved = true;
        }, 100);
    }
}
```

## The Client
### Web Camera

For interfacing with the camera, I used Javascript's built in functionality for fetching the web camera:
```javascript
navigator.mediaDevices.getUserMedia({video: true, audio: false}).then(stream => {
    video.srcObject = stream;
    video.play();
});
```
`getUserMedia` returns a `Promise<MediaStream>`. To access the stream when the promise resolves, I'm using the method `then((MediaStream) => Promise): Promise` to configure an `<video>` element to be playing of the web camera.

### Image generation
The `<video>` object referenced above is used as an interface to the web camera, this object can be used to draw an still image onto an canvas by using `CanvasRenderingContext2D::drawImage(...)`. Since I needed to scale the image down to 15x7, i decided that the canvas would have 15x7 and that the internals of `drawImage()` would handle the required scaling. The final result did turn out Ok and i was looking at myself in nice minecraft graphics.

<img src="images/assignment08/me-in-pixels.png" class="image"/>

The next step in processing the image is to convert it to grayscale. This was done by using the [luminance formula](https://en.wikipedia.org/wiki/Grayscale#Luma_coding_in_video_systems). which gave the following method:
```js
    const color = 0.2989 * data[i] + 0.5870 * data[i+1] + 0.1140 * data[i+2];  
```

the data object was retrieved by using `CanvasRenderingContext2D::getImageData(x, y, w, h).buffer`. This returns an `Uint8Array` of the pixel data in the format [r1, g1, b1, a1, r2, g2, b2, a2, ....].
Since i wanted to display the grayscale image on the client, I decied that I'd reuse the data buffer retrieved with getImageData, and put the grayscale colors back into it. I could then display the image by calling `PutImageData(new ImageData(data, w, h), x, y)`.
For the messaging part I created a new Buffer `out_message`. This buffer had 105 bytes was written to on each iteration of the grayscale production. Once the image was done being greyed out, a simple `emit` call was made with the image data.

Here is the main loop for fetching, processing and sending images
```js
const getImage = () => {
    if (isStreaming){
        ctx.drawImage(video, 0, 0, width, height, 0, 0, canvasW, canvasH);
        
        const data = ctx.getImageData(0,0,canvasW,canvasH).data;
        

        //console.log("f", data);
        const l = data.length;
        const to_send = new Uint8ClampedArray(l/4);
        for(let i = 0, j = 0; i < l; i+=4,j++){
            
            const color = 0.2989 * data[i] + 0.5870 * data[i+1] + 0.1140 * data[i+2];  
            data[i] = color;
            data[i+1] = color;
            data[i+2] = color;
            to_send[j] = color;
        }
        ctx.putImageData(new ImageData(data, canvasW, canvasH,{
            colorSpace: "srgb"
        }),0,0);

        socket.emit("image", to_send)
    }
    requestAnimationFrame(getImage);
}
```

Finally a video of the result:

<video controls="controls" width="800" height="600" name="Video Name">
    <source src="images/assignment08/video.mp4"/>
</video>


# Improvements
One improvemnt I could see was in the commmunication between the arduino and the server. Currently it's a Fire and hope technique where the server sends the data and waits for a fixed amount of time before continuing. A better approach would be to use a ping-pong technique, where the server Sends the message, and waits for the Arduino to respond with a OK message, thereby allowing the next message to be sent. A benefit of using Ping-Pong would be to increase the speed in wich images are processed, and to prevent overflowing the Serialport buffer.