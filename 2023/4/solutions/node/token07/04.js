const fs = require('fs');

let input = fs.readFileSync(0, "utf-8").trimEnd().replaceAll("\r", "");

let accumulator = 0;
let dupes = [];
let cardNumber;
input.split("\n").forEach(line => {
    cardNumber = parseInt(line.substring(5, line.indexOf(":")).replaceAll(" ", ""))
    let winningNumbers = line.substring(line.indexOf(":")+2, line.indexOf("|")-1).replaceAll("  ", " ").trim().split(" ").map(n => parseInt(n));
    let numbersPlayed = line.substring(line.indexOf("|")+2).replaceAll("  ", " ").split(" ").map(n => parseInt(n))

    let numMatches = numbersPlayed.filter(number => winningNumbers.indexOf(number) !== -1);
    if (numMatches.length == 1) accumulator++;
    if (numMatches.length > 1) accumulator += 1 * Math.pow(2, numMatches.length-1);
    dupes.length += numMatches.length;
    for (let i = 0; i < numMatches.length; i++) {
        if (!dupes[cardNumber+i]) dupes[cardNumber+i] = 0;
        dupes[cardNumber+i]++;
    }
    if (dupes[cardNumber-1]) {
        for (let j = dupes[cardNumber-1]; j > 0; j--) {
            for (let i = 0; i < numMatches.length; i++) {
                if (!dupes[cardNumber+i]) dupes[cardNumber+i] = 0;
                dupes[cardNumber+i]++;
            }
        }
    }
})
console.log(accumulator)
console.log(dupes.filter(d => d).reduce((a,c) => a+c)+cardNumber);