const fs = require('fs');

const input = fs.readFileSync(0, "utf-8").trim();
let splitInput = input.split(/\r?\n/);
let accumulator = 0

splitInput.forEach(line => {
    const symbols = line.split("").filter(char => char.match(/\d/));
    accumulator += parseInt(symbols[0] + symbols[symbols.length-1]);
})
console.log(accumulator);

const valueTable = {
    "1": 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    "0": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "zero": 0
}
accumulator = 0;
splitInput.forEach(line => {
    let valueKeys = Object.keys(valueTable);
    let firstVals = [];
    let lastVals = [];
    firstVals.length = lastVals.length = valueKeys.length
    firstVals.fill({},0, valueKeys.length);
    lastVals.fill({},0, valueKeys.length);

    firstVals = firstVals.map((value, index) => ({key: valueKeys[index], pos: line.indexOf(valueKeys[index])})).filter(e => e.pos !== -1).sort((a,b) => a.pos - b.pos);
    lastVals = lastVals.map((value, index) => ({key: valueKeys[index], pos: line.lastIndexOf(valueKeys[index])})).filter(e => e.pos !== -1).sort((a,b) => b.pos - a.pos);
    accumulator += parseInt(valueTable[firstVals[0].key]*10 + valueTable[lastVals[0].key]);
})
console.log(accumulator);