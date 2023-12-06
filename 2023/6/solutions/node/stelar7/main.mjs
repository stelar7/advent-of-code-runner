import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const times = lines[0]
  .split(" ")
  .filter((a) => a)
  .slice(1)
  .map((x) => parseInt(x));
const distances = lines[1]
  .split(" ")
  .filter((a) => a)
  .slice(1)
  .map((x) => parseInt(x));

function checkWins(time, distance) {
  let wins = 0;
  for (let j = 0; j < time; j++) {
    const speed = j;
    const remaining = time - j;
    const distanceCovered = speed * remaining;
    if (distanceCovered > distance) {
      wins++;
    }
  }
  return wins;
}

const allWins = times.map((time, i) => checkWins(time, distances[i]));
console.log(allWins.reduce((a, b) => a * b, 1));

const finalRaceTime = parseInt(times.join(""));
const finalDistance = parseInt(distances.join(""));
console.log(checkWins(finalRaceTime, finalDistance));
