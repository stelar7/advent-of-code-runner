import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const config = {
  red: 12,
  green: 13,
  blue: 14,
};

const parseColors = (color) => {
  const colors = {
    red: 0,
    green: 0,
    blue: 0,
  };

  const entry = color.trim().split(" ");
  for (let i = 1; i < entry.length; i += 2) {
    const element = entry[i].toString().trim().replaceAll(",", "");
    const count = parseInt(entry[i - 1]);

    if (element == "red") colors.red += count;
    else if (element == "green") colors.green += count;
    else if (element == "blue") colors.blue += count;
    else console.log("Unknown color: " + element);
  }

  return colors;
};

const isPossible = (game) => {
  const colors = parseColors(game);
  if (colors.red > config.red) return false;
  if (colors.green > config.green) return false;
  if (colors.blue > config.blue) return false;
  return true;
};

const fewestPossible = (games) => {
  let min = {
    red: 0,
    green: 0,
    blue: 0,
  };

  for (const game of games) {
    const colors = parseColors(game);
    if (colors.red > min.red) min.red = colors.red;
    if (colors.green > min.green) min.green = colors.green;
    if (colors.blue > min.blue) min.blue = colors.blue;
  }

  return min;
};

let sum = 0;
let powerSum = 0;
line: for (const line of lines) {
  const id = line.split(":")[0].split(" ")[1];
  const games = line.split(": ")[1].split(";");

  const min = fewestPossible(games);
  const power = min.red * min.green * min.blue;
  powerSum += power;

  for (const game of games) {
    if (!isPossible(game)) {
      continue line;
    } 
  }

  sum += parseInt(id);
}
console.log(sum);
console.log(powerSum);
