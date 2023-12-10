import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const lagrange = (values, solveFor) => {
  let result = 0;
  for (let i = 0; i < values.length; i++) {
    let term = values[i];
    for (let j = 0; j < values.length; j++) {
      if (i !== j) {
        term *= (solveFor - j) / (i - j);
      }
    }
    result += term;
  }
  return result;
};

const part1 = () => {
  let sum = 0;
  for (let i = 0; i < lines.length; i++) {
    const values = lines[i].split(" ").map((x) => parseInt(x));
    const nextValue = lagrange(values, values.length);
    sum += nextValue;
  }
  console.log(Math.round(sum));
};

const part2 = () => {
  let sum = 0;
  for (let i = 0; i < lines.length; i++) {
    const values = lines[i].split(" ").map((x) => parseInt(x));
    const nextValue = lagrange(values, -1);
    sum += nextValue;
  }
  console.log(Math.round(sum));
};

part1();
part2();