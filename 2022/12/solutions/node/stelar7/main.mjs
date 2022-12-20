import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const map = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1).map(l => l.split(""));

function printGrid(grid) {
    for (let y = 0; y < grid.length; y++) {
        let line = "";
        for (let x = 0; x < grid[y].length; x++) {
            const char = grid[y][x];
            const mapChar = map[y][x];
            line += char == undefined ? mapChar : "█";
        }

        console.log(line);
    }
}

function buildGrid(lastNode) {
    const grid = Array(map.length).fill().map((v, i) => Array(map[0].length));
    let current = lastNode;
    while (current != null && current != undefined) {
        grid[current.y][current.x] = "█";
        current = current.parent;
    }

    return grid;
}

let abc = Array(26).fill().map((v, i) => String.fromCharCode(97 + i));

function validNeighbor(x, y, value) {
    if (map[y] == undefined) {
        return;
    }

    if (map[y][x] == undefined) {
        return;
    }

    const toCheck = map[y][x];

    if (abc.indexOf(toCheck) > abc.indexOf(value) + 1) {
        return;
    }

    return {
        x,
        y,
        value: toCheck
    };

}

function getNeighbors(current) {
    const neighbors = [];

    neighbors.push(validNeighbor(current.x + 1, current.y, current.value));
    neighbors.push(validNeighbor(current.x - 1, current.y, current.value));
    neighbors.push(validNeighbor(current.x, current.y + 1, current.value));
    neighbors.push(validNeighbor(current.x, current.y - 1, current.value));

    return neighbors.filter(i => i);
}

function astar(starts, e) {
    let open = [];
    let closed = [];

    for (let start of starts) {
        open.push({
            ...start,
            g: 0,
            q: 0,
            f: 0
        });
    }

    while (open.length > 0) {
        open = open.sort((a, b) => b.f - a.f);
        let current = open.pop();

        if (current.y == e.y && current.x == e.x) {
            return current;
        }

        const neighbors = getNeighbors(current);

        for (const successor of neighbors) {
            successor.g = current.g + 1;
            successor.h = Math.abs(successor.x - end.x) + Math.abs(successor.y - end.y);
            successor.f = successor.g + successor.h;
            successor.parent = current;

            if (closed.some(old => old.x == successor.x && old.y == successor.y && old.f <= successor.f)) {
                continue;
            }

            if (open.some(old => old.x == successor.x && old.y == successor.y && old.f <= successor.f)) {
                continue;
            }

            open.push(successor);
        }

        closed.push(current);
    }

    return null;
}

let start = {};
let end = {};
for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
        const element = map[y][x];

        if (element === "S") {
            start = {
                x,
                y,
                value: "a"
            };
            map[y][x] = start.value;
        }

        if (element === "E") {
            end = {
                x,
                y,
                value: "z"
            };
            map[y][x] = end.value;
        }
    }
}


const shortestStoE = astar([start], end);
console.log(shortestStoE.g);

const allA = [start];
for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
        const element = map[y][x];
        if (element == "a") {
            start = {
                x,
                y,
                value: "a"
            };

            allA.push(start);
        }
    }
}

const shortestA = astar(allA, end);
console.log(shortestA.f);