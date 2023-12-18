import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

let memo = {};

const countOptions = (pattern, rule) => {
  memo = {};
  return countOptionsInner(0, 0, 0, pattern, rule);
};

const countOptionsInner = (i, j, r, pattern, rule) => {
  if (j == pattern.length) {
    if (i == j && rule.length == r) {
      return 1;
    }
    if (rule.length - 1 == r && j - i == rule[r]) {
      return 1;
    }
    return 0;
  }

  if (i >= pattern.length || j >= pattern.length || r > rule.length) {
    return 0;
  }

  const pos = [i, j, r];
  const key = JSON.stringify(pos);

  if (memo[key] != undefined) {
    return memo[key];
  }

  const c = pattern[j];
  if (c == "#") {
    const result = countOptionsInner(i, j + 1, r, pattern, rule);
    memo[key] = result;
    return result;
  }

  if (c == ".") {
    if (i != j) {
      if (r == rule.length || rule[r] != j - i) {
        return 0;
      }
      i = j;
      r++;
    }
    const result = countOptionsInner(i + 1, j + 1, r, pattern, rule);
    memo[key] = result;
    return result;
  }

  let result = countOptionsInner(i, j + 1, r, pattern, rule);
  if (i != j) {
    if (r == rule.length || rule[r] != j - i) {
      memo[key] = result;
      return result;
    }
    i = j;
    r++;
  }
  result += countOptionsInner(i + 1, j + 1, r, pattern, rule);
  memo[key] = result;
  return result;
};

let part1Sum = 0;
let part2Sum = 0;

for (const line of lines) {
  const [springs, digits] = line.split(" ");

  const springSequence = new Array(5).fill(springs).join("?");
  const digitSequence = new Array(5)
    .fill(digits)
    .join(",")
    .split(",")
    .map(Number);

  part1Sum += countOptions(springs, digits.split(",").map(Number));
  part2Sum += countOptions(springSequence, digitSequence);
}

console.log(part1Sum);
console.log(part2Sum);
