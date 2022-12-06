import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const stacks = [];
for (const line of lines) {
    if (!line.startsWith("move")) {
        for (let i = 1; i < line.length; i += 4) {
            const index = (i - 1) / 4;
            if (!stacks[index]) {
                stacks[index] = [];
            }

            const toPush = line.charAt(i);
            if (toPush.match(/[A-Z]/)) {
                stacks[index].push(line.charAt(i));
            }
        }
    }
}

const useStack = JSON.parse(JSON.stringify(stacks));
for (const line of lines) {
    if (!line.startsWith("move")) {
        continue;
    }

    const count = +line.split(" ")[1];
    const from = +line.split(" ")[3] - 1;
    const to = +line.split(" ")[5] - 1;

    for (let i = 0; i < count; i++) {
        const elem = useStack[from].shift();
        useStack[to].unshift(elem);
    }
}

let output = "";
for (let i = 0; i < useStack.length; i++) {
    const elem = useStack[i][0];
    output += elem;
}

console.log(output);


const useStack2 = JSON.parse(JSON.stringify(stacks));
for (const line of lines) {
    if (!line.startsWith("move")) {
        continue;
    }

    const count = +line.split(" ")[1];
    const from = +line.split(" ")[3] - 1;
    const to = +line.split(" ")[5] - 1;

    const temp = [];
    for (let i = 0; i < count; i++) {
        const elem = useStack2[from].shift();
        temp.push(elem);
    }
    useStack2[to].unshift(...temp);
}

let output2 = "";
for (let i = 0; i < useStack2.length; i++) {
    const elem = useStack2[i][0];
    output2 += elem;
}

console.log(output2);