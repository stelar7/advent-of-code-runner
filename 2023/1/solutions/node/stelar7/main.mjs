import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const optionsPartOne = {
  1: 1,
  2: 2,
  3: 3,
  4: 4,
  5: 5,
  6: 6,
  7: 7,
  8: 8,
  9: 9,
};

const options = {
  1: 1,
  2: 2,
  3: 3,
  4: 4,
  5: 5,
  6: 6,
  7: 7,
  8: 8,
  9: 9,
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};

const findDigit = function (line, array, method, comparison) {
  return Object.keys(array).reduce(
    (old, next) => {
      const best = old[0];
      const index = line[method](next);

      if (index === -1) {
        return old;
      }

      if (best === null) {
        return [index, next];
      }

      if (comparison(index, best)) {
        return [index, next];
      }

      return old;
    },
    [null, null]
  );
};

const runDay = function (array) {
  let sum = 0;
  for (const line of lines) {
    const bestDigit = findDigit(
      line,
      array,
      "indexOf",
      (index, best) => index < best
    );
    const bestValue = bestDigit[1] == null ? 0 : array[bestDigit[1]];

    const worstDigit = findDigit(
      line,
      array,
      "lastIndexOf",
      (index, worst) => index > worst
    );
    const worstValue = worstDigit[1] == null ? 0 : array[worstDigit[1]];

    const concat = parseInt(bestValue.toString() + worstValue.toString());
    sum += concat;
  }

  return sum;
};

console.log(runDay(optionsPartOne));
console.log(runDay(options));
