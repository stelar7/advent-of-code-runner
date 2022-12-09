import {
    readFileSync
} from "fs";
import { exit } from "process";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const grid = [];

for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (!grid[i]) {
        grid[i] = [];
    }

    for (let j = 0; j < line.length; j++) {
        if (!grid[i][j]) {
            grid[i][j] = [];
        }

        grid[i][j] = +line[j];
    }
}

const visibleIndex = new Set();

for (let i = 0; i < grid.length; i++) {
    let max = -1;
    for (let j = 0; j < grid[0].length; j++) {
        if (grid[i][j] > max) {
            visibleIndex.add(JSON.stringify({
                "x": i,
                "y": j,
                "v": grid[i][j]
            }));
            max = Math.max(max, grid[i][j]);
        }
    }

    max = -1;
    for (let j = grid[0].length - 1; j >= 0; j--) {
        if (grid[i][j] > max) {
            visibleIndex.add(JSON.stringify({
                "x": i,
                "y": j,
                "v": grid[i][j]
            }));
            max = Math.max(max, grid[i][j]);
        }
    }
}

for (let i = 0; i < grid.length; i++) {
    let max = -1;
    for (let j = 0; j < grid[0].length; j++) {
        if (grid[j][i] > max) {
            visibleIndex.add(JSON.stringify({
                "x": j,
                "y": i,
                "v": grid[j][i]
            }));
            max = Math.max(max, grid[j][i]);
        }
    }

    max = -1;
    for (let j = grid[0].length - 1; j >= 0; j--) {
        if (grid[j][i] > max) {
            visibleIndex.add(JSON.stringify({
                "x": j,
                "y": i,
                "v": grid[j][i]
            }));
            max = Math.max(max, grid[j][i]);
        }
    }
}

const outputGrid = [];
for (let i = 0; i < grid.length; i++) {
    if (!outputGrid[i]) {
        outputGrid[i] = [];
    }

    for (let j = 0; j < grid[0].length; j++) {
        if (!outputGrid[i][j]) {
            outputGrid[i][j] = [];
        }

        outputGrid[i][j] = "?";
    }
}


for (let tree of visibleIndex) {
    const obj = JSON.parse(tree);

    outputGrid[obj.x][obj.y] = "" + obj.v;
}



console.log(visibleIndex.size);

let maxScore = 0;
for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
        let values = [];
        const candidate = grid[i][j];
        let hit = false;
        
        hit = false;
        for (let y = i - 1; y > 0; y--) {
            if (grid[y][j] >= candidate) {
                const diff = (i - y);
                values.push(diff);
                hit = true;
                break;
            }
        }

        if (!hit) {
            let diff = i;
            values.push(diff);
        }

        
        hit = false;
        for (let x = j - 1; x > 0; x--) {
            if (grid[i][x] >= candidate) {
                const diff = (j - x);
                values.push(diff);
                hit = true;
                break;
            }
        }

        if (!hit) {
            let diff = j;
            values.push(diff);
        }

        hit = false;
        for (let y = i + 1; y < grid.length; y++) {
            if (grid[y][j] >= candidate) {
                const diff = (y - i);
                values.push(diff);
                hit = true;
                break;
            }
        }

        if (!hit) {
            let diff = grid.length - (i + 1);
            values.push(diff);
        }

        hit = false;
        for (let x = j + 1; x < grid[0].length; x++) {
            if (grid[i][x] >= candidate) {
                const diff = (x - j);
                values.push(diff);
                hit = true;
                break;
            }
        }

        if (!hit) {
            let diff = grid[0].length - (j + 1);
            values.push(diff);
        }

        const score = values.reduce((o, n) => o * n, 1);

        if (score > maxScore) {
            maxScore = score;
        }

    }
}

console.log(maxScore);