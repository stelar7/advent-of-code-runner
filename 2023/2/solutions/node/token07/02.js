let fs = require('fs');

const maxConditions = {
    "red": 12,
    "green": 13,
    "blue": 14
}

function parseGame(line) {
    if (!line.startsWith("Game")) return false;
    let gameID = line.substring(line.indexOf(" ")+1, line.indexOf(":"));
    let subsets = [];
    let maxPerSubset = {};
    line = line.substring(line.indexOf(":")+2);
    line.split(";").forEach(subset => {
        if (subset.startsWith(" ")) subset = subset.substring(1);
        let counts = {};
        subset.split(",").forEach(element => {
            if (element.startsWith(" ")) element = element.substring(1);
            let count = parseInt(element.substring(0, element.indexOf(" ")));
            let color = element.substring(element.indexOf(" ")+1);
            counts[color] = count;
            if (!maxPerSubset[color]) maxPerSubset[color] = 0;
            maxPerSubset[color] = Math.max(maxPerSubset[color], count);
        })
        subsets.push(counts);
    });
    return {
        "gameID": parseInt(gameID),
        "subsets": subsets,
        "maxPerSubset": maxPerSubset,
    }
}

let games = [];
let input = fs.readFileSync(0, "utf-8").trimEnd();

input.split(/\r?\n/).forEach(line => games.push(parseGame(line)));
console.log(games.filter(game => game.maxPerSubset.red <=maxConditions.red && game.maxPerSubset.green <= maxConditions.green && game.maxPerSubset.blue <= maxConditions.blue).reduce((prev, cur) => cur.gameID+prev, 0))
let accumulator = 0;
games.forEach(game => {
    accumulator += Object.values(game.maxPerSubset).reduce((prev, cur) => cur * prev, 1)
})
console.log(accumulator);