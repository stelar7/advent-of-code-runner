import {
    readFileSync
} from "fs";
import {
    exit
} from "process";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

let cycle = 1;
let counter = 1;

let output = 0;
let outputString = "";

function sumSignal() {
    if ((cycle + 20) % 40 == 0) {
        output += cycle * counter;
    }
}

function drawImage() {
    let center = ((cycle - 1) % 40);
    if (counter - 1 == center || counter == center || counter + 1 == center) {
        outputString += "#";
    } else {
        outputString += ".";
    }

    if (cycle % 40 == 0) {
        outputString += "\n";
    }
}

for (const element of lines) {
    const line = element;

    if (line.startsWith("noop")) {
        drawImage();
        cycle++;
        sumSignal();
        continue;
    }

    drawImage();
    cycle++;
    sumSignal();

    drawImage();
    cycle++;
    const count = +line.substring(4, line.length);
    counter += count;
    sumSignal();
}

console.log(output);
console.log(outputString.trim());