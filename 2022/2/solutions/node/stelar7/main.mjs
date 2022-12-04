import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const mapping = {
    X: "rock",
    A: "rock",
    Y: "paper",
    B: "paper",
    Z: "scissor",
    C: "scissor",
};

const scores = {
    rock: 1,
    paper: 2,
    scissor: 3,
};

const outcome = {
    X: "lose",
    Y: "draw",
    Z: "win",
};

const winsOver = {
    rock: "paper",
    paper: "scissor",
    scissor: "rock",
};

const losesTo = {
    rock: "scissor",
    paper: "rock",
    scissor: "paper",
};

let score = 0;
for (const line of lines) {
    const opponent = mapping[line[0]];
    const mine = mapping[line[2]];

    const draw = mine == opponent;
    const win = (mine == "rock" && opponent == "scissor") || (mine == "scissor" && opponent == "paper") || (mine == "paper" && opponent == "rock");

    score += scores[mine];

    if (win) {
        score += 6;
    } else if (draw) {
        score += 3;
    }
}

console.log(`${score}`);

score = 0;
for (const line of lines) {
    const opponent = mapping[line[0]];
    const expected = outcome[line[2]];

    if (expected == "lose") {
        const mine = scores[losesTo[opponent]];
        score += mine;
    } else if (expected == "draw") {
        const mine = scores[opponent];
        score += mine + 3;
    } else {
        const mine = scores[winsOver[opponent]];
        score += mine + 6;
    }
}

console.log(`${score}`);