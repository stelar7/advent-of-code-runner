import { readFileSync, writeFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

function scanWord(word, ix, iy, dx, dy) {
    let x = ix;
    let y = iy;
    let i = 0;

    while (x >= 0 && y >= 0 && y < lines.length && x < lines[y].length) {
        if (lines[y][x] != word[i]) {
            return 0;
        }
        
        x += dx;
        y += dy;
        i++;

        if (i == word.length) {
            return 1;
        }
    }

    return 0;    
};

let part1 = 0;
for (let y = 0; y < lines.length; y++) {
    const line = lines[y];
    for (let x = 0; x < line.length; x++) {
        const letter = line[x];
        if (letter == 'X') {
            // right
            part1 += scanWord("XMAS", x, y, 1, 0);
            // down right
            part1 += scanWord("XMAS", x, y, 1, 1);
            // down
            part1 += scanWord("XMAS", x, y, 0, -1);
            // down left
            part1 += scanWord("XMAS", x, y, -1, -1);
            // left
            part1 += scanWord("XMAS", x, y, -1, 0);
            // up left
            part1 += scanWord("XMAS", x, y, -1, 1);
            // up
            part1 += scanWord("XMAS", x, y, 0, 1);
            // up right
            part1 += scanWord("XMAS", x, y, 1, -1);
        }
    }
}

console.log(part1);


let part2 = 0;
const MAS = ['MAS', 'MAS'.split('').reverse().join('')];
for (let i = 0; i < lines.length; i++) {
  for (let j = 0; j < lines.length; j++) {
    const char = lines[i][j];

    if (char !== 'A') continue;
    if (i - 1 < 0) continue;
    if (i + 1 >= lines.length) continue;
    if (j - 1 < 0) continue;
    if (j + 1 >= lines.length) continue;

    const topleft = lines[i - 1][j - 1];
    const topright = lines[i - 1][j + 1];
    const bottomleft = lines[i + 1][j - 1];
    const bottomright = lines[i + 1][j + 1];

    const diag1 = `${topleft}${char}${bottomright}`;
    const diag2 = `${topright}${char}${bottomleft}`;

    if (MAS.includes(diag1) && MAS.includes(diag2)) {
        part2 += 1;
    }
  }
}

console.log(part2);
