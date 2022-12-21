import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const grid = [];

function fillStones(line) {
    const paths = line.split(" -> ");
    let fromX = null;
    let fromY = null;
    for (const path of paths) {
        const [xString, yString] = path.split(",");

        const x = parseInt(xString);
        const y = parseInt(yString);

        if (fromX == null && fromY == null) {
            fromX = x;
            fromY = y;
            continue;
        }

        //console.log("stone from", fromX, fromY, "to", x, y);

        for (let i = Math.min(fromY, y); i <= Math.max(fromY, y); i++) {
            if (grid[i] == undefined) {
                grid[i] = [];
            }


            //console.log("stone at", x, i);

            grid[i][x] = "#";
        }

        for (let i = Math.min(fromX, x); i <= Math.max(fromX, x); i++) {
            if (grid[y] == undefined) {
                grid[y] = [];
            }

            //console.log("stone at", i, y);

            grid[y][i] = "#";
        }

        fromX = x;
        fromY = y;
    }
}

lines.forEach(l => fillStones(l));
const lowestPoint = grid.length;

function printGrid(grid) {
    for (const element of grid) {
        let line = "";
        for (let x = 400; x < 600; x++) {
            let char = ".";
            if (element && element[x]) {
                char = element[x];
            }

            if (x == 500 && char == ".") {
                char = "+";
            }

            line += char;
        }

        console.log(line);
    }
}


function fallSand(floor) {
    let sand = {
        x: 500,
        y: 0
    };


    function positionDown(item) {
        return {
            ...item,
            y: item.y + 1,
        }
    }

    function positionLeft(item) {
        return {
            x: item.x - 1,
            y: item.y + 1,
        }
    }

    function positionRight(item) {
        return {
            x: item.x + 1,
            y: item.y + 1,
        }
    }

    function isBlocked(item, floorpos) {
        if (!grid[item.y]) {
            grid[item.y] = [];
        }

        return grid[item.y][item.x] == "#" || grid[item.y][item.x] == "O" || item.y >= floorpos;
    }

    while (true) {
        const floorLocation = !floor ? Number.MAX_SAFE_INTEGER : lowestPoint + 1;

        if (!isBlocked(positionDown(sand), floorLocation)) {
            sand = positionDown(sand);


            if (!floor && sand.y > lowestPoint) {
                return true;
            }

            continue;
        }

        if (!isBlocked(positionLeft(sand), floorLocation)) {
            sand = positionLeft(sand);
            continue;
        }

        if (!isBlocked(positionRight(sand), floorLocation)) {
            sand = positionRight(sand);
            continue;
        }

        if (!grid[sand.y]) {
            grid[sand.y] = [];
        }

        grid[sand.y][sand.x] = "O";

        if (floor && sand.y == 0 && sand.x == 500) {
            return true;
        }

        return false;
    }
}


let count = 0;
while (!fallSand()) {
    count++;
}
console.log(count);

count++;
while (!fallSand(true)) {
    count++;
}
console.log(count);