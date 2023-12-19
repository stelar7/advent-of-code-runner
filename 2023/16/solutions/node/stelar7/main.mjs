import { readFileSync, writeFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const UP = [0, -1];
const DOWN = [0, 1];
const LEFT = [-1, 0];
const RIGHT = [1, 0];

const MIRROR_A = "/";
const MIRROR_B = "\\";
const SPLIT_A = "|";
const SPLIT_B = "-";

const items = [];
for (let y = 0; y < lines.length; y++) {
  for (let x = 0; x < lines[y].length; x++) {
    if (lines[y][x] !== ".") {
      items.push({ x, y, type: lines[y][x] });
    }
  }
}

const getNext = ({ x, y }, direction) => {
  switch (direction) {
    case UP:
      return (
        items
          .filter((item) => item.x == x && item.y < y)
          .sort((a, b) => b.y - a.y)[0] ?? { x, y: 0, type: "edge" }
      );
    case DOWN:
      return (
        items
          .filter((item) => item.x == x && item.y > y)
          .sort((a, b) => a.y - b.y)[0] ?? {
          x,
          y: lines.length,
          type: "edge",
        }
      );
    case LEFT:
      return (
        items
          .filter((item) => item.y == y && item.x < x)
          .sort((a, b) => b.x - a.x)[0] ?? { x: 0, y, type: "edge" }
      );
    case RIGHT:
      return (
        items
          .filter((item) => item.y == y && item.x > x)
          .sort((a, b) => a.x - b.x)[0] ?? {
          x: lines[0].length,
          y,
          type: "edge",
        }
      );
  }
};

const newDirection = (from, type) => {
  switch (type) {
    case MIRROR_A: {
      if (from == LEFT) return [DOWN];
      if (from == UP) return [RIGHT];
      if (from == RIGHT) return [UP];
      if (from == DOWN) return [LEFT];
    }
    case MIRROR_B: {
      if (from == LEFT) return [UP];
      if (from == UP) return [LEFT];
      if (from == RIGHT) return [DOWN];
      if (from == DOWN) return [RIGHT];
    }
    case SPLIT_A: {
      if (from == LEFT) return [UP, DOWN];
      if (from == RIGHT) return [UP, DOWN];
      if (from == UP) return [UP];
      if (from == DOWN) return [DOWN];
    }
    case SPLIT_B: {
      if (from == LEFT) return [LEFT];
      if (from == RIGHT) return [RIGHT];
      if (from == UP) return [LEFT, RIGHT];
      if (from == DOWN) return [LEFT, RIGHT];
    }
    default:
      throw new Error("Unknown type: " + type);
  }
};

const drawGrid = (grid) => {
  writeFileSync(
    "grid",
    grid.map((row) => row.map((v) => (v ? "#" : ".")).join("")).join("\n"),
    { flag: "a" }
  );
  writeFileSync("grid", "\n\n", { flag: "a" });
};

const castRay = ({ x, y }, direction) => {
  const visitedSquares = JSON.parse(
    JSON.stringify(
      new Array(lines.length).fill(new Array(lines[0].length).fill(false))
    )
  );

  const markPath = ({ fromX, toX }, { fromY, toY }, isX) => {
    if (isX) {
      for (let i = Math.min(fromX, toX); i <= Math.max(fromX, toX); i++) {
        if (visitedSquares[fromY][i] == undefined) {
          continue;
        }

        visitedSquares[fromY][i] = true;
      }
    } else {
      for (let i = Math.min(fromY, toY); i <= Math.max(fromY, toY); i++) {
        if (!visitedSquares[i]) {
          continue;
        }

        visitedSquares[i][fromX] = true;
      }
    }
  };

  const seen = [];
  const toResolve = [{ x, y, direction }];
  while (toResolve.length > 0) {
    const entry = toResolve.shift();
    direction = entry.direction;
    if (
      seen.find(
        (x) => x.x == entry.x && x.y == entry.y && x.direction == direction
      )
    ) {
      continue;
    }

    seen.push({ x: entry.x, y: entry.y, direction: entry.direction });

    let next = getNext(entry, entry.direction);

    markPath(
      { fromX: entry.x, toX: next.x },
      { fromY: entry.y, toY: next.y },
      entry.direction == LEFT || entry.direction == RIGHT
    );

    if (next.type == "edge") {
      continue;
    }

    const newDirections = newDirection(entry.direction, next.type);

    for (const newDirection of newDirections) {
      toResolve.push({ x: next.x, y: next.y, direction: newDirection });
    }
  }

  return visitedSquares.flatMap((r) => r.filter((b) => b)).length;
};

const result = castRay({ x: -1, y: 0 }, RIGHT);
console.log(result);

let max = 0;
for (let y = 0; y < lines.length; y++) {
  let ray = castRay({ x: -1, y }, RIGHT);
  max = Math.max(max, ray);

  ray = castRay({ x: lines[0].length, y }, LEFT);
  max = Math.max(max, ray);

}

for (let x = 0; x < lines[0].length; x++) {
  let ray = castRay({ x, y: -1 }, DOWN);
  max = Math.max(max, ray);

  ray = castRay({ x, y: lines.length }, UP);
  max = Math.max(max, ray);
}

console.log(max);
