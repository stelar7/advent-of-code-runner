import { readFileSync, writeFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const parseVector = (str) => {
  const [x, y, z] = str.split(",").map((e) => parseInt(e.trim()));
  return { x, y, z };
};

const hails = lines
  .map((line) => line.split("@"))
  .map((pair) => [parseVector(pair[0]), parseVector(pair[1])]);

const getRayIntersection = (a, b) => {
  const p0 = a[0];
  const p1 = b[0];
  const n0 = a[1];
  const n1 = b[1];

  const dx = p1.x - p0.x;
  const dy = p1.y - p0.y;
  const det = n1.x * n0.y - n1.y * n0.x;
  const u = (dy * n1.x - dx * n1.y) / det;
  const v = (dy * n0.x - dx * n0.y) / det;
  if (u < 0 || v < 0) {
    return undefined;
  }

  const m0 = n0.y / n0.x;
  const m1 = n1.y / n1.x;
  const b0 = p0.y - m0 * p0.x;
  const b1 = p1.y - m1 * p1.x;
  const x = (b1 - b0) / (m0 - m1);
  const y = m0 * x + b0;

  if (Number.isFinite(x)) {
    return [x, y];
  }

  return undefined;
};

const sub = (a, b) => ({
  x: a.x - b.x,
  y: a.y - b.y,
  z: a.z - b.z,
});

const cross = (a, b) => ({
  x: a.y * b.z - a.z * b.y,
  y: a.z * b.x - a.x * b.z,
  z: a.x * b.y - a.y * b.x,
});

const dot = (a, b) => a.x * b.x + a.y * b.y + a.z * b.z;

const findPlane = (hailA, hailB) => {
  const p12 = sub(hailA[0], hailB[0]);
  const v12 = sub(hailA[1], hailB[1]);
  const vv = cross(hailA[1], hailB[1]);
  return [cross(p12, v12), dot(p12, vv)];
};

const roundDiv = (a, b) => ({
  x: Math.round(a.x / b),
  y: Math.round(a.y / b),
  z: Math.round(a.z / b),
});

const mul = (a, b) => ({
  x: a.x * b,
  y: a.y * b,
  z: a.z * b,
});

const add = (a, b) => ({
  x: a.x + b.x,
  y: a.y + b.y,
  z: a.z + b.z,
});

const findRock = (hailA, hailB, hailC) => {
  const [a, A] = findPlane(hailA, hailB);
  const [b, B] = findPlane(hailA, hailC);
  const [c, C] = findPlane(hailB, hailC);

  const t = dot(a, cross(b, c));
  const A1 = mul(cross(b, c), A);
  const B1 = mul(cross(c, a), B);
  const C1 = mul(cross(a, b), C);
  const WC = add(add(A1, B1), C1);

  const w = roundDiv(WC, t);
  const w1 = sub(hailA[1], w);
  const w2 = sub(hailB[1], w);
  const ww = cross(w1, w2);

  const E = dot(ww, cross(hailB[0], w2));
  const F = dot(ww, cross(hailA[0], w1));
  const G = dot(hailA[0], ww);
  const S = dot(ww, ww);

  const R1 = mul(w1, E);
  const R2 = mul(w2, -F);
  const R3 = mul(ww, G);
  const rock = add(add(R1, R2), R3);

  return roundDiv(rock, S);
};

let count = 0;
const min = 200000000000000;
const max = 400000000000000;
for (let i = 0; i < hails.length; i++) {
  for (let j = i + 1; j < hails.length; j++) {
    const intersection = getRayIntersection(hails[i], hails[j]);
    if (!intersection) {
      continue;
    }

    const [x, y] = intersection;
    if (x < min || x > max || y < min || y > max) {
      continue;
    }

    count++;
  }
}

const rock = findRock(hails[0], hails[1], hails[2]);

console.log(count);
console.log(rock.x + rock.y + rock.z);
