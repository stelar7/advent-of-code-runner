import { readFileSync } from "fs";

const STANDARD_IN = 0;
const input = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);
const regex = /(\d+),(\d+) -> (\d+),(\d+)/g;

const lines = input.map(line => [...line.matchAll(regex)][0].slice(1).map(d => +d));
const maxX = lines.reduce((o, line) => Math.max(o, line[0], line[2]), 0);
const maxY = lines.reduce((o, line) => Math.max(o, line[1], line[3]), 0);

function generateOverlap(lineData) {
    const grid = [...Array(maxY + 1)].map(y => Array(maxX + 1).fill(0));

    for (let line of lineData) {
        let diffX = line[2] - line[0];
        let diffY = line[3] - line[1];

        if (diffX != 0) {
            diffX = Math.sign(diffX);
        }

        if (diffY != 0) {
            diffY = Math.sign(diffY);
        }

        let x = line[0];
        let y = line[1];
        while(x != line[2] || y != line[3]) {
            grid[y][x]++;
            x += diffX;
            y += diffY;
        }
        grid[y][x]++;
    }

    return grid.flat().filter(i => i > 1).length;
}

console.log(generateOverlap(lines.filter(line => line[0] == line[2] || line[1] == line[3])));
console.log(generateOverlap(lines));