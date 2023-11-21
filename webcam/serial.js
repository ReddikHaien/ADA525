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
