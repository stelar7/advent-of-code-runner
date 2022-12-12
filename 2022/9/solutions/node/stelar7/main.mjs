import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const tails = [
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0]
];

const tailPos = new Set();
const tenTailPos = new Set();

function followTail(head, tail) {
    // move tail in + axis
    if (head[0] == tail[0] + 2 && head[1] == tail[1]) {
        tail[0] += 1;
    } else if (head[0] == tail[0] - 2 && head[1] == tail[1]) {
        tail[0] -= 1;
    } else if (head[1] == tail[1] + 2 && head[0] == tail[0]) {
        tail[1] += 1;
    } else if (head[1] == tail[1] - 2 && head[0] == tail[0]) {
        tail[1] -= 1;
    } else if (head[0] != tail[0] && head[1] != tail[1] && (head[0] == tail[0] + 2 || head[1] == tail[1] + 2 || head[0] == tail[0] - 2 || head[1] == tail[1] - 2)) {
        if (head[0] > tail[0]) {
            tail[0] += 1;
        } else {
            tail[0] -= 1;
        }

        if (head[1] > tail[1]) {
            tail[1] += 1;
        } else {
            tail[1] -= 1;
        }
    }
}

function printTail(outputSet) {
    let output = "";
    for (let y = -100; y <= 100; y++) {
        for (let x = -100; x < 100; x++) {
            output += outputSet.has(JSON.stringify([x, y])) ? "#" : ".";
        }
        console.log(output);
        output = "";
    }
}

for (const element of lines) {
    const line = element;

    const dir = line[0];
    const count = +line.substring(2, line.length);

    for (let i = 0; i < count; i++) {
        // move head
        if (dir == "U") {
            tails[0][1] += 1;
        } else if (dir == "D") {
            tails[0][1] -= 1;
        } else if (dir == "L") {
            tails[0][0] -= 1;
        } else if (dir == "R") {
            tails[0][0] += 1;
        }

        for (let t = 0; t < tails.length - 1; t++) {
            followTail(tails[t], tails[t + 1]);
        }

        tailPos.add(JSON.stringify(tails[1]));
        tenTailPos.add(JSON.stringify(tails[9]));
    }
}

console.log(tailPos.size);
console.log(tenTailPos.size);