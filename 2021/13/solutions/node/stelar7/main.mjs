import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");

const regex = /(\d+),(\d+)/g;
const lines = input.map(line => line.includes(",") ? [...line.matchAll(regex)][0].slice(1).map(d => +d) : []).filter(a => a.length > 0);
const maxX = lines.reduce((o, line) => Math.max(o, line[0]), 0);
const maxY = lines.reduce((o, line) => Math.max(o, line[1]), 0);

const folds = [];
let grid = [...Array(maxY + 1)].map(y => Array(maxX + 1).fill("."));

input.forEach(line => {
    if (line.includes("along")) folds.push({
        isX: line.includes("x"),
        offset: +line.split("=")[1]
    });
    if (line.includes(",")) {
        const [x, y] = line.split(",").map(d => +d);
        grid[y][x] = "#";
    }
});

function doFold(isX, offset) {
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            let sourceX = (isX ? offset : 0) + x;
            let sourceY = (isX ? 0 : offset) + y;
            let destX = 0;
            let destY = 0;

            if (isX) {
                destX = offset - x;
                destY = y;
            } else {
                destX = x;
                destY = offset - y;
            }

            if (sourceY > grid.length - 1) continue;
            if (sourceX > grid[y].length - 1) continue;
            if (destY < 0) continue;
            if (destX < 0) continue;

            if (grid[destY][destX] != "#" && grid[sourceY][sourceX] == "#") {
                grid[destY][destX] = grid[sourceY][sourceX];
            }
        }
    }

    if (isX) {
        for (let y = 0; y < grid.length; y++) {
            grid[y] = grid[y].slice(0, offset);
        }
    } else {
        grid = grid.slice(0, offset);
    }
}

let hasPrinted = false;
for (const {
        isX,
        offset
    } of folds) {
    doFold(isX, offset);

    if (!hasPrinted) {
        console.log(grid.flat().reduce((o, n) => o + (n == "#" ? 1 : 0), 0));
        hasPrinted = true;
    }
}

for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
        process.stdout.write(grid[y][x]);
    }
    console.log();
}