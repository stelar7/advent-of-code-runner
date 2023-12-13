const fs = require('fs');

let galaxies = [];
let input = fs.readFileSync(0, "utf-8").trimEnd()
let lines = input.split("\n");

// Horizontal
for (let i = 0; i < lines.length; i++){ 
    if (lines[i].indexOf("#") === -1) {
        lines.splice(i, 0, lines[i]);
        i++;
    }
}
// Vertical
for (let i = 0; i < lines[0].length; i++) {
    let vLine = lines.map(lm => lm[i]).join("");
    if (vLine.indexOf("#") === -1) {
        lines = lines.map(lm => lm.slice(0, i) + "." + lm.slice(i));
        i++;
    }
}
// Find Galaxies
for (const y in lines) {
    let offset = 0;
    let x;
    while ((x = lines[y].indexOf("#", offset)) !== -1) {
        galaxies.push([x, parseInt(y)]);
        offset = x+1;
    }
}
let pairs = []
for (let i = 0; i < galaxies.length; i++) {
    for (let j = i + 1; j < galaxies.length; j++) {
        pairs.push([i, j]);
    }
}

let result = pairs.map(pair => {
    let g1Loc = galaxies[pair[0]];
    let g2Loc = galaxies[pair[1]];

    return Math.abs(g1Loc[0] - g2Loc[0]) + Math.abs(g1Loc[1] - g2Loc[1]); 
})
console.log(result.reduce((a,c) => a+c));


galaxies = [];
lines = input.split("\n");

// Part 2

// Horizontal
for (let i = 0; i < lines.length; i++){ 
    if (lines[i].indexOf("#") === -1) {
        lines[i] = lines[i].replaceAll(".", ",");
        i++;
    }
}
// Vertical
for (let i = 0; i < lines[0].length; i++) {
    let vLine = lines.map(lm => lm[i]).join("");
    if (vLine.indexOf("#") === -1) {
        lines = lines.map(lm => lm.substring(0, i) + "," + lm.substring(i+1));
        //i+=2;
    }
} 
// Find Galaxies
for (const y in lines) {
    let offset = 0;
    let x;
    while ((x = lines[y].indexOf("#", offset)) !== -1) {
        galaxies.push([x, parseInt(y)]);
        offset = x+1;
    }
}
pairs = []
for (let i = 0; i < galaxies.length; i++) {
    for (let j = i + 1; j < galaxies.length; j++) {
        pairs.push([i, j]);
    }
}

result = pairs.map(pair => {
    let xPath = 0;
    let yPath = 0;
    let g1Loc = galaxies[pair[0]];
    let g2Loc = galaxies[pair[1]];
    for (let q = Math.min(g1Loc[0], g2Loc[0]); q < Math.max(g1Loc[0], g2Loc[0]); q++) {
        if (lines[g1Loc[1]][q] == ",") xPath += 1000000;
        else xPath++;
    }
    for (let q = Math.min(g1Loc[1], g2Loc[1]); q < Math.max(g1Loc[1], g2Loc[1]); q++) {
        if (lines[q][g1Loc[0]] == ",") yPath += 1000000;
        else yPath++;
    }
    return xPath+yPath;
});

console.log(result.reduce((a,c) => a+c));
