import { createServer } from "http";
import { Server } from "socket.io";
import onImageData from "./serial.js";
import { existsSync, readFileSync } from "fs";
import { cwd } from "process";
import { join } from "path";


const server = createServer((req, res) => {
    var url = req.url ?? "/";
    url = url.replace(/\.\.|\:/,"");
    if (url == "/"){
        url = "/index.html";
    }
    const ext = url.substring(url.lastIndexOf("."));
    const mediatype = 
        ext == ".html" ? "text/html"
       :ext == ".js"   ? "application/javascript"
       :                 "text/plain";

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

const io = new Server(server, {

});

io.on("connection", (socket) => {

    socket.on("image",onImageData);
});

server.listen(8080, "localhost", () => {
    console.log("Server listening on 8080");
});