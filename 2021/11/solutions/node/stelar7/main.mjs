import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");
const grid = [];
input.forEach(line => {
    if (line.length > 0) grid.push(line.split("").map(d => +d));
});

function getNeighbours(startX, startY) {
    const near = [];
    for (let y = -1; y <= 1; y++) {
        const gridY = startY + y;
        if (gridY < 0) continue;
        if (gridY > grid.length - 1) continue;

        for (let x = -1; x <= 1; x++) {
            const gridX = startX + x;
            if (x == y && y == 0) continue;
            if (gridX < 0) continue;
            if (gridX > grid[gridY].length - 1) continue;

            near.push([gridX, gridY]);
        }
    }

    return near;
}


function doStep() {
    let flashCount = 0;

    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            grid[y][x]++;
        }
    }

    while (grid.flat().some(i => i > 9)) {
        for (let y = 0; y < grid.length; y++) {
            for (let x = 0; x < grid[y].length; x++) {
                if (grid[y][x] > 9) {
                    flashCount++;
                    const nearby = getNeighbours(x, y);
                    nearby.forEach(([x, y]) => grid[y][x]++);
                    grid[y][x] = Number.MIN_SAFE_INTEGER;
                }
            }
        }
    }

    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            if (grid[y][x] < 0) grid[y][x] = 0;
        }
    }

    return flashCount;
}

let maxFlashes = grid.length * grid[0].length;
let atEqual = {
    found: false,
    value: 0,
};
let at100 = {
    found: false,
    value: 0,
};
let totalFlashes = 0;
for (let i = 1; i < Number.MAX_SAFE_INTEGER; i++) {
    const stepFlashCount = doStep();
    totalFlashes += stepFlashCount;

    if (i == 100 && !at100.found) {
        at100.value = totalFlashes;
        at100.found = true;
    }

    if (stepFlashCount === maxFlashes && !atEqual.found) {
        atEqual.value = i;
        atEqual.found = true;
    }

    if (atEqual.found && at100.found) {
        break;
    }
}

console.log(at100.value);
console.log(atEqual.value);