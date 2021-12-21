import {
    readFileSync
} from "fs";
const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");
let grid = [];
input.forEach(line => {
    grid.push(line.split("").map(d => +d));
});
grid = grid.filter(e => e.length > 0);

function getNeighbours(data, x, y) {
    const near = [];
    if (data.length >= y - 1 && y - 1 >= 0) {
        near.push([x, y - 1, data[y - 1][x]]);
    }
    if (data.length > y + 1) {
        near.push([x, y + 1, data[y + 1][x]]);
    }
    if (data[y].length >= x - 1 && x - 1 >= 0) {
        near.push([x - 1, y, data[y][x - 1]]);
    }
    if (data[y].length >= x + 1) {
        near.push([x + 1, y, data[y][x + 1]]);
    }
    return near.filter(i => i != undefined);
}

function dijkstra(data) {
    let newNodes = [];
    let distanceToCells = {};
    for (let y = 0; y < data.length; y++) {
        if (!distanceToCells[y]) distanceToCells[y] = [...Array(data.length)];
        for (let x = 0; x < data[y].length; x++) {
            distanceToCells[y][x] = Infinity;
        }
    }
    
    distanceToCells[0][0] = 0;
    newNodes = newNodes.slice(1);
    newNodes.push([0, 0, 0]);

    const maxX = data[0].length - 1;
    const maxY = data.length - 1;
    
    while (newNodes.length > 0) {
        let [x, y] = newNodes.pop();
    
        const neighbours = getNeighbours(data, x, y);
    
        for (let [nx, ny, cost] of neighbours) {
            const oldCost = distanceToCells[y][x];
            const newCost = oldCost + cost;
            const neighbourCost = distanceToCells[ny][nx];
    
            if (newCost < neighbourCost) {
                distanceToCells[ny][nx] = newCost;
    
                if (ny == maxY && nx == maxX) {
                    newNodes = [];
                    break;
                }

                const oldPos = newNodes.findIndex(e => e[0] == nx && e[1] == ny);
                if (oldPos > -1) {
                    newNodes.splice(oldPos, 1, [nx, ny, newCost]);
                } else {
                    newNodes.push([nx, ny, newCost]);
                }
            }
        }
    
        newNodes.sort((a, b) => b[2] - a[2]);
    }
    
    console.log(distanceToCells[maxY][maxX]);
}

dijkstra(grid);

const maxX = grid[0].length;
const maxY = grid.length;

const largerGrid = [...Array(maxY * 5)].map(y => Array(maxX * 5).fill(0));
for (let cy = 0; cy < 5; cy++) {
    for (let cx = 0; cx < 5; cx++) {
        for (let y = 0; y < grid.length; y++) {
            for (let x = 0; x < grid[y].length; x++) {
                const oldValue = grid[y][x];
                const toAdd = cx + cy;
                let newValue = ((oldValue + toAdd) % 9);
                if (newValue == 0) newValue = 9;

                const newX = (cx * maxX) + x;
                const newY = (cy * maxY) + y;

                largerGrid[newY][newX] = newValue;
            }
        }
    }
}

dijkstra(largerGrid);