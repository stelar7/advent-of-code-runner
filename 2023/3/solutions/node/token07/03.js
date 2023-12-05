const fs = require('fs');

let input = fs.readFileSync(0, "utf-8").trimEnd().replaceAll("\r", "");
let width = input.split("\n").length+1;
function getXY(pos) {
    return {x: (pos % width) + 1, y: Math.floor(pos / width) + 1}
}
function getCharInXY(coords) {
    return input[(coords.y-1) * width + coords.x-1];
}
function getAdjacent(coords) {
    return [
        {x: coords.x-1, y: coords.y}, // Left 1
        {x: coords.x+1, y: coords.y}, // Right 1
        {x: coords.x, y: coords.y-1}, // Up 1
        {x: coords.x, y: coords.y+1}, // Down 1
        {x: coords.x-1, y: coords.y-1}, // Left 1, Up 1
        {x: coords.x+1, y: coords.y-1}, // Right 1, Up 1
        {x: coords.x+1, y: coords.y+1}, // Right 1 Down 1
        {x: coords.x-1, y: coords.y+1} // Left 1, Down 1
    ]//.filter(e => e.x > 0 && e.y > 0 && e.x <= width && e.y <= length);
}
function getNumberAt(coords) {
    let startX = coords.x-1;
    let number = "";
    let pos = [];
    //if (!getCharInXY(coords).match(/d/)) return false;
    if (!(/\d/.test(getCharInXY(coords)))) return false;
    pos.push({x: startX+1, y: coords.y});
    number = getCharInXY(coords);
    while (startX > 0) {
        let val = getCharInXY({x: startX, y: coords.y});
        if (/\d/.test(val)) number = val.toString() + number.toString(); 
        else break;
        pos.push({x: startX, y: coords.y});
        startX--;
    }
    startX = coords.x+1
    while (startX < width) {
        let val = getCharInXY({x: startX, y: coords.y});
        if (/\d/.test(val)) number = number.toString() + val.toString(); 
        else break;
        pos.push({x: startX, y: coords.y});
        startX++;
    }
    console.assert(pos.length == number.length);
    return {number: number, pos: pos};
}
let accumulator = 0;
let accumulator2 = 0;

for (const idx in input) {
    if (/[^0-9.\n]/.test(input[idx])) {
        let sym = input[idx]
        let adjacent = getAdjacent(getXY(idx));
        let adjacentNumbers = [];
        while (adjacent.length > 0) {
            let curAdjacent = adjacent.shift();
            let numberAt = getNumberAt(curAdjacent);
            if (!numberAt) continue;
            //console.log(numberAt.number)
            accumulator += parseInt(numberAt.number);
            numberAt.pos.forEach(pos => adjacent = adjacent.filter(e => (!(e.x == pos.x && e.y == pos.y))))
            if (sym == "*") adjacentNumbers.push(numberAt.number);
        }
        if (sym == "*" && adjacentNumbers.length == 2) accumulator2 += adjacentNumbers[0]*adjacentNumbers[1];
    }
}
console.log(accumulator);
console.log(accumulator2);