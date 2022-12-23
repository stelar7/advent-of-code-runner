import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

function parseValves(lines) {
  const valves = [];

  for (const line of lines) {
    const middle = line.split(";");
    const valveString = middle[0].split(" has flow rate=");

    const valve = {
      name: valveString[0].split(" ")[1],
      flowrate: parseInt(valveString[1]),
      paths: [],
    };

    const pathString = middle[1].split("valves ");
    if (pathString.length == 1) {
      valve.paths.push(pathString[0].slice(-2));
    } else {
      valve.paths.push(...pathString[1].split(", "));
    }

    valves.push(valve);
  }

  return valves;
}

function parseRates(valves) {
  const rates = [];

  for (let valve of valves) {
    rates[valve.name] = valve.flowrate;
  }

  return rates;
}

function floydWarshall(valves) {
  const distances = [];

  for (let valve of valves) {
    if (!distances[valve.name]) {
      distances[valve.name] = [];
    }

    distances[valve.name][valve.name] = 0;

    for (let path of valve.paths) {
      distances[valve.name][path] = 1;

      if (!distances[path]) {
        distances[path] = [];
      }

      distances[path][path] = 0;
    }
  }

  for (const a of valves) {
    for (const b of valves) {
      for (const c of valves) {
        const bc = distances[b.name][c.name] ?? Number.MAX_SAFE_INTEGER;
        const ba = distances[b.name][a.name] ?? Number.MAX_SAFE_INTEGER;
        const ac = distances[a.name][c.name] ?? Number.MAX_SAFE_INTEGER;

        if (ba + ac < bc) {
          distances[b.name][c.name] = ba + ac;
        }
      }
    }
  }

  return distances;
}

function score(rates, chosen) {
  let score = 0;

  for (let [valve, remaining] of Object.entries(chosen)) {
    score += rates[valve] * remaining;
  }

  return score;
}

function solve(distance, rates, valves, time, current, chosen = []) {
  const result = [chosen];

  for (let next of valves) {
    const newTime = time - (distance[current][next.name] + 1);
    if (newTime < 2) {
      continue;
    }

    const newChosen = { ...chosen, [next.name]: newTime };
    const newValves = valves.filter((v) => v.name != next.name);

    result.push(
      solve(distance, rates, newValves, newTime, next.name, newChosen)
    );
  }

  return result;
}

const valves = parseValves(lines);
const rates = parseRates(valves);
const goodValves = valves.filter((v) => v.flowrate > 0);
const distances = floydWarshall(valves);
const solutions = solve(distances, rates, goodValves, 30, "AA");
const flatSolutions = solutions.flat(Number.POSITIVE_INFINITY);
const scoredSolutions = flatSolutions
  .map((s) => score(rates, s))
  .sort((a, b) => b - a);

console.log(scoredSolutions[0]);

const solutionsLessTime = solve(distances, rates, goodValves, 26, "AA");
const flatLessTime = solutionsLessTime.flat(Number.POSITIVE_INFINITY);
const maxScores = [];
for (let solution of flatLessTime) {
  const keys = [...Object.keys(solution)].sort();
  const solutionScore = score(rates, solution);

  if (!maxScores[keys]) {
    maxScores[keys] = 0;
  }

  if (solutionScore > maxScores[keys]) {
    maxScores[keys] = solutionScore;
  }
}

let combinationScore = 0;
const entries = Object.entries(maxScores);
for (let a = 0; a < entries.length; a++) {
  const [keyA, valA] = entries[a];

  for (let b = a + 1; b < entries.length; b++) {
    const [keyB, valB] = entries[b];
    if (!keyA.split(",").some((v) => keyB.split(",").includes(v))) {
      if (combinationScore < valA + valB) {
        combinationScore = valA + valB;
      }
    }
  }
}

console.log(combinationScore);
