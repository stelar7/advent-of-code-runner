import { readFileSync } from "fs";

const STANDARD_IN = 0
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1)


function generatePower(input) {
    const power = [];

    for (const line of input) {
        for (let i = 0; i < line.length; i++) {
            if (!power[i]) {
                power[i] = [];
            }
    
            const char = line.charAt(i);
            if (!power[i][char]) {
                power[i][char] = 0;
            }
    
            power[i][char]++;
        }
    }

    return power;
}


function gammaRate(power) {
    return power.reduce((o, n) => [...o, n.indexOf(Math.max(...n))], []);
}

function epsilonRate(power) {
    return power.reduce((o, n) => [...o, n.indexOf(Math.min(...n))], []);
}

function toDecimal(input) {
    return parseInt(input, 2);
}

function oxygenGenrerator() {
    let linesCopy = [...lines];

    for (let i = 0; i < lines[0].length; i++) {
        const power = generatePower(linesCopy);
        const gamma = gammaRate(power);
        const epsilon = epsilonRate(power);
        
        linesCopy = linesCopy.filter(line => {
            if (gamma[i] == epsilon[i]) {
                if (line[i] != "1") {
                    return false;
                }
            } else if (line[i] != gamma[i]) {
                return false;
            }

            return true;
        });

        if (linesCopy.length == 1) {
            break;
        }
    }

    return linesCopy[0];
}


function co2Scrubber() {
    let linesCopy = [...lines];

    for (let i = 0; i < lines[0].length; i++) {
        const power = generatePower(linesCopy);
        const gamma = gammaRate(power);
        const epsilon = epsilonRate(power);
        
        linesCopy = linesCopy.filter(line => {
            if (gamma[i] == epsilon[i]) {
                if (line[i] != "0") {
                    return false;
                }
            } else if (line[i] != epsilon[i]) {
                return false;
            }

            return true;
        });

        if (linesCopy.length == 1) {
            break;
        }
    }

    return linesCopy[0];
}

const inputPower = generatePower(lines);
console.log(toDecimal(gammaRate(inputPower).join(""), 2) * toDecimal(epsilonRate(inputPower).join(""), 2));
console.log(toDecimal(oxygenGenrerator(), 2) * toDecimal(co2Scrubber(), 2));