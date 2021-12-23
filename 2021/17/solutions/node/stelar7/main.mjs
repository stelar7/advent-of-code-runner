import {
    readFileSync
} from "fs";
const STANDARD_IN = 0;
let raw = readFileSync(STANDARD_IN).toString();
const regex = /target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)/g;
const data = [...raw.matchAll(regex)][0];

const xMin = data[1];
const xMax = data[2];
const yMin = data[3];
const yMax = data[4];

function hitsTarget(x, y) {
    return x >= xMin && x <= xMax && y >= yMin && y <= yMax;
}

function doSim(xSpeed, ySpeed) {
    let probe = [0, 0];
    let velocity = [xSpeed, ySpeed];
    let maxY = 0;

    while (true) {
        probe[0] += velocity[0];
        probe[1] += velocity[1];

        velocity[0] += -Math.sign(velocity[0]);
        velocity[1] -= 1;

        maxY = Math.max(maxY, probe[1]);

        if (hitsTarget(...probe)) {
            return maxY;
            break;
        }

        if (probe[1] < yMin) {
            return null;
            break;
        }
    }

}

let maxY = -Infinity;
let count = 0;
for (let x = 0; x <= xMax; x++) {
    for (let y = Math.min(yMin, -yMin); y <= Math.max(yMin, -yMin); y++) {
        const didHit = doSim(x, y);
        if (didHit != null) {
            count++;
            maxY = Math.max(maxY, didHit);
        }
    }
}

console.log(maxY);
console.log(count);