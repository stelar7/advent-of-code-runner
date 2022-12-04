import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

var contain = 0;
var overlap = 0;
for (const line of lines) {
    const first = line.split(",")[0];
    const second = line.split(",")[1];

    const min1 = +first.split("-")[0];
    const max1 = +first.split("-")[1];

    const min2 = +second.split("-")[0];
    const max2 = +second.split("-")[1];


    if (min1 <= min2 && max1 >= max2) {
        contain++;
    } else if (min2 <= min1 && max2 >= max1) {
        contain++;
    }

    if (max1 >= min2 && max2 >= min1) {
        overlap++;
    }
}


console.log(`${contain}`);
console.log(`${overlap}`);