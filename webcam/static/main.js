var width = 800;
var height = 600;
const canvasW = 15;
const canvasH = 7;
const video = document.getElementsByTagName("video")[0];
const canvas = document.getElementsByTagName("canvas")[0];
const ctx = canvas.getContext("2d",{
    willReadFrequently: true
});
var isStreaming = false;
video.addEventListener("canplay", () => {
    if(!isStreaming){
        height = video.videoHeight;
        width = video.videoWidth;
        canvas.width = canvasW;
        canvas.height = canvasH;
        isStreaming = true;
        requestAnimationFrame(getImage);
    }

},false);

const imageBuffer = [];

const socket = io();

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

navigator.mediaDevices.getUserMedia({video: true, audio: false}).then(stream => {
    video.srcObject = stream;
    video.play();
});


