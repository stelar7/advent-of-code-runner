import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);
const array = lines.map((line) => line.split(""));

const transpose = (array) =>
  array[0].map((_, colIndex) => array.map((row) => row[colIndex]));
const transposed = transpose(array);

const empty = [];
const galaxies = [];

for (let x = 0; x < transposed.length; x++) {
  const line = transposed[x];

  if (!line.includes("#")) {
    empty.push({ x });
    continue;
  }
}

for (let y = 0; y < lines.length; y++) {
  const line = lines[y];

  if (!line.includes("#")) {
    empty.push({ y });
    continue;
  }

  for (let x = 0; x < line.length; x++) {
    const char = line[x];

    if (char === "#") {
      galaxies.push({ x, y });
    }
  }
}

const expandGalaxies = (galaxies, times) => {
  const expanded = [];

  for (const galaxy of galaxies) {
    const { x, y } = galaxy;

    const expandX = empty.filter((e) => e.x < x).length * times;
    const expandY = empty.filter((e) => e.y < y).length * times;

    expanded.push({ x: x + expandX, y: y + expandY });
  }

  return expanded;
};

const generateDistances = (expanded_galaxies) => {
  const distances = [];
  for (let i = 0; i < expanded_galaxies.length; i++) {
    for (let j = i + 1; j < expanded_galaxies.length; j++) {
      const galaxy1 = expanded_galaxies[i];
      const galaxy2 = expanded_galaxies[j];

      const x = Math.abs(galaxy1.x - galaxy2.x);
      const y = Math.abs(galaxy1.y - galaxy2.y);
      const distance = x + y;

      distances.push({
        galaxy1,
        galaxy2,
        distance,
      });
    }
  }

  return distances;
};

const part1ExpandFactor = 2;
const part2ExpandFactor = 1_000_000;

const calculatePart = (expandFactor) => {
  return generateDistances(expandGalaxies(galaxies, expandFactor - 1))
    .map((d) => d.distance)
    .reduce((a, b) => a + b, 0);
};

console.log(calculatePart(part1ExpandFactor));
console.log(calculatePart(part2ExpandFactor));
