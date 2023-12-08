import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

class Node {
  constructor(name, left, right) {
    this.name = name;
    this.left = left;
    this.right = right;
  }
}

const directions = lines[0].split("");
const nodes = [];
for (let i = 2; i < lines.length; i++) {
  const line = lines[i];
  const [name, paths] = line.split(" = ");
  const [left, right] = paths.slice(1, -1).split(", ");

  nodes.push(new Node(name, left, right));
}

const gcd = function (a, b) {
  return !b ? a : gcd(b, a % b);
};

const lcm = function (a, b) {
  return (a * b) / gcd(a, b);
};

const stepsToSolve = function (input) {
  let current = input;
  let steps = 0;
  let directionIndex = 0;

  while (!current.name.endsWith("Z")) {
    const direction = directions[directionIndex++ % directions.length];
    const next = current[direction == "R" ? "right" : "left"];
    current = nodes.find((node) => node.name == next);
    steps++;
  }

  return steps;
};

const part1 = function () {
  const startNode = nodes.find((node) => node.name == "AAA");
  return stepsToSolve(startNode);
};

const part2 = function () {
  let commonEnd = 1;
  let movingNodes = nodes.filter((node) => node.name.endsWith("A"));
  for (let i = 0; i < movingNodes.length; i++) {
    const node = movingNodes[i];
    const nodeToEnd = stepsToSolve(node);
    commonEnd = lcm(commonEnd, nodeToEnd);
  }

  return commonEnd;
};

console.log(part1());
console.log(part2());
