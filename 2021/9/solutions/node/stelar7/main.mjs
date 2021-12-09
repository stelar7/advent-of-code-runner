import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");
const grid = [];
input.forEach(line => {
    grid.push(line.split("").map(d => +d));
});

function getNeighbours(x, y) {
    const near = [];
    if (grid.length >= y - 1 && y - 1 >= 0) {
        near.push(grid[y - 1][x]);
    }
    if (grid.length >= y + 1) {
        near.push(grid[y + 1][x]);
    }
    if (grid[y].length >= x - 1 && x - 1 >= 0) {
        near.push(grid[y][x - 1]);
    }
    if (grid[y].length >= x + 1) {
        near.push(grid[y][x + 1]);
    }
    return near.filter(i => i != undefined);
}

const lowPoints = [];
let riskValue = 0;
for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
        const value = grid[y][x];
        const nearby = getNeighbours(x, y);
        const isLowpoint = nearby.every(i => i > value);
        if (isLowpoint) {
            lowPoints.push([x, y]);
            riskValue += value;
        }
    }
}
console.log(riskValue + lowPoints.length);


function appendNeighbours(holder, x, y) {
    if (y < 0 || y >= grid.length) return;
    if (x < 0 || x >= grid[y].length) return;

    const value = grid[y][x];
    if (value == 9) return;
    if (JSON.stringify(holder).includes(JSON.stringify([x,y]))) return;
    holder.push([x,y]);
    appendNeighbours(holder, x+1, y);
    appendNeighbours(holder, x-1, y);
    appendNeighbours(holder, x, y+1);
    appendNeighbours(holder, x, y-1);
}

const basins = [];
for (let [startX, startY] of lowPoints) {
    const parts = [];
    appendNeighbours(parts, startX, startY);
    basins.push(parts.length);
}

const top3 = basins.sort((a,b) => b - a).slice(0, 3);
const product = top3.reduce((o,n) => o*n, 1);
console.log(product);