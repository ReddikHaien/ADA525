import express from "express"
import { getData, start } from "./simulation.js";

start();

const app = express();
app.use(express.static("static", {}))
app.get("/api/data", (req, res) => {
    const [rabbits, foxes] = getData();

    res.status(200).send({
        rabbits,
        foxes
    });
});

console.log("server starting at 8080");
app.listen(8080);
