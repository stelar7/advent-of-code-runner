import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const UP = [0, -1];
const DOWN = [0, 1];
const LEFT = [-1, 0];
const RIGHT = [1, 0];

const OPPOSITES = {
  [UP]: DOWN,
  [DOWN]: UP,
  [LEFT]: RIGHT,
  [RIGHT]: LEFT,
};

const DIRECTIONS = {
  "|": [UP, DOWN],
  "-": [LEFT, RIGHT],
  L: [UP, RIGHT],
  J: [UP, LEFT],
  7: [DOWN, LEFT],
  F: [DOWN, RIGHT],
};

const graph = {};
let start = null;
for (let y = 0; y < lines.length; y++) {
  const line = lines[y];

  for (let x = 0; x < line.length; x++) {
    const char = line[x];

    const key = `${x},${y}`;

    if (Object.keys(DIRECTIONS).includes(char)) {
      graph[key] = DIRECTIONS[char];
    }

    if (char === "S") {
      start = [x, y];
    }
  }
}

const move = (from, direction) => {
  const [x, y] = from;
  const [dx, dy] = direction;

  return [x + dx, y + dy];
};

const toStart = [];
for (const direction of Object.values(OPPOSITES)) {
  const next = move(start, direction);
  const key = `${next[0]},${next[1]}`;
  if (key in graph && graph[key].includes(OPPOSITES[direction])) {
    toStart.push(direction);
  }
}

const startChar = Object.entries(DIRECTIONS).find(([_, directions]) => {
  return directions.every((direction) => toStart.includes(direction));
})[0];

let current = move(start, DIRECTIONS[startChar][0]);
const visited = [JSON.stringify(start)];

while (true) {
  visited.push(JSON.stringify(current));
  const key = `${current[0]},${current[1]}`;
  const options = graph[key];

  const unvisited = options
    .filter(
      (option) => !visited.includes(JSON.stringify(move(current, option)))
    )
    .map((option) => move(current, option));

  if (unvisited.length === 0) {
    break;
  }

  current = unvisited[0];
}

console.log(visited.length / 2);

let isInside = false;
let insideCount = 0;
const width = lines[0].length;
const height = lines.length;

for (let y = 0; y < height; y++) {
  const line = lines[y];

  for (let x = 0; x < width; x++) {
    const isMainLoop = visited.includes(JSON.stringify([x, y]));

    let char = line[x];
    if (char === "S") {
      char = startChar;
    }

    if (!isMainLoop && isInside) {
      insideCount++;
    } else if (isMainLoop) {
      if (["F", "7", "|"].includes(char)) {
        isInside = !isInside;
      }
    }
  }
}

console.log(insideCount);
