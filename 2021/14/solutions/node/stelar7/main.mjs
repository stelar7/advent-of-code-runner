import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n");

const template = input[0];
const regex = /(..) -> (.)/g;
const rules = input.slice(2, -1).map(line => [...line.matchAll(regex)][0].slice(1, 3)).reduce((o, n) => {
    o[n[0]] = n[1];
    return o;
}, {});

let pairs = {};

for (let i = 0; i < template.length; i++) {
    const pair = template.substring(i, i + 2)
    if (pair.length < 2) continue;
    if (!pairs[pair]) pairs[pair] = 0;
    pairs[pair]++;
}

function doStep() {
    const newPairs = {};
    for (let pair in pairs) {
        const rule = rules[pair];
        const newPairA = pair[0] + rule;
        const newPairB = rule + pair[1];
        const count = pairs[pair];
        if (!newPairs[newPairA]) newPairs[newPairA] = 0;
        newPairs[newPairA] += count;
        if (!newPairs[newPairB]) newPairs[newPairB] = 0;
        newPairs[newPairB] += count;
    }

    pairs = newPairs;
}

function generateCounts() {
    const lastElement = template.substring(template.length - 1);
    const initialState = {};
    initialState[lastElement] = 1;

    let counts = Object.entries(pairs).reduce((o, n) => {
        let pair = n[0];
        let count = n[1];

        if (!o[pair[0]]) o[pair[0]] = 0;
        o[pair[0]] += count;
        return o;
    }, initialState);

    return counts;
}

function printMinMax(counts) {
    let min = Infinity;
    let max = -Infinity;
    for (let x in counts) {
        if (counts[x] < min) min = counts[x];
        if (counts[x] > max) max = counts[x];
    }

    console.log(max - min);
}

for (let i = 0; i < 10; i++) {
    doStep();
}

printMinMax(generateCounts());

for (let i = 0; i < 30; i++) {
    doStep();
}

printMinMax(generateCounts());