import { readFileSync } from "fs";

const STANDARD_IN = 0;
const patterns = readFileSync(STANDARD_IN).toString().split("\n\n");

const transpose = (array) =>
  array[0].map((_, colIndex) => array.map((row) => row[colIndex]).join(""));

const findReflectionIndex = (array, smudges) => {
  for (let i = 0; i < array.length; i++) {
    if (validateReflection(array, i, smudges)) {
      return i;
    }
  }

  return null;
};

const validateReflection = (array, reflectionIndex, smudges = 0) => {
  if (reflectionIndex == null || reflectionIndex == 0) {
    return false;
  }

  const firstHalf = array.slice(0, reflectionIndex).reverse();
  const secondHalf = array.slice(reflectionIndex);

  let diffCount = 0;
  for (let i = 0; i < Math.min(firstHalf.length, secondHalf.length); i++) {
    const diff = countDifference(firstHalf[i], secondHalf[i]);
    diffCount += diff;
  }

  if (diffCount != smudges) {
    return false;
  }

  return true;
};

const countDifference = (a, b) => {
  let count = 0;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) count++;
  }
  return count;
};

let sumPart1 = 0;
let sumPart2 = 0;

for (const pattern of patterns) {
  let rows = pattern.split("\n").filter((a) => a);
  const columns = transpose(rows.map((row) => row.split("")));

  const horizontalReflection = findReflectionIndex(rows);
  if (horizontalReflection) {
    sumPart1 += horizontalReflection * 100;
  }

  const verticalReflection = findReflectionIndex(columns);
  if (verticalReflection) {
    sumPart1 += verticalReflection;
  }

  const horizontalSmudgeReflection = findReflectionIndex(rows, 1);
  if (horizontalSmudgeReflection) {
    sumPart2 += horizontalSmudgeReflection * 100;
  }

  const verticalSmudgeReflection = findReflectionIndex(columns, 1);
  if (verticalSmudgeReflection) {
    sumPart2 += verticalSmudgeReflection;
  }
}

console.log(sumPart1);
console.log(sumPart2);
