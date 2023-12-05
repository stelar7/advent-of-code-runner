import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

class Range {
  constructor(dest, src, count) {
    this.dest = dest;
    this.src = src;
    this.count = count;
  }
}

class RangeMap {
  constructor(ranges) {
    this.start = [];
    this.end = [];
    this.between = [];

    for (const range of ranges) {
      this.start.push(range.src);
      this.end.push(range.dest);
      this.between.push(range.count);
    }
  }

  get(value) {
    let result = value;
    for (let i = 0; i < this.start.length; i++) {
      let start = this.start[i];
      let end = this.start[i] + this.between[i];

      if (value >= start && value < end) {
        result = this.end[i] + (value - start);
        break;
      }
    }
    return result;
  }

  getBound(value) {
    let nextStart = Number.MAX_SAFE_INTEGER;
    let result = value;

    for (let i = 0; i < this.start.length; i++) {
      let start = this.start[i];
      let end = this.start[i] + this.between[i];

      if (value >= start && value < end) {
        result = this.end[i] + (value - start);
        nextStart = this.between[i] - (value - start + 1);
        break;
      } else if (start > value) {
        nextStart = Math.min(nextStart, start - value + 1);
      }
    }

    return [result, nextStart];
  }
}

const seeds = lines[0]
  .substring("seeds: ".length)
  .split(" ")
  .map((x) => parseInt(x));
const seedSoilMapIndex = lines.indexOf("seed-to-soil map:");
const soilFertilizerMapIndex = lines.indexOf("soil-to-fertilizer map:");
const fertilizerWaterMapIndex = lines.indexOf("fertilizer-to-water map:");
const waterLightMapIndex = lines.indexOf("water-to-light map:");
const lightTemperatureMapIndex = lines.indexOf("light-to-temperature map:");
const temperatureHumidityMapIndex = lines.indexOf(
  "temperature-to-humidity map:"
);
const humidityLocationMapIndex = lines.indexOf("humidity-to-location map:");

function toRange(line) {
  const [dest, src, count] = line.split(" ").map((x) => parseInt(x));
  return new Range(dest, src, count);
}

const rangemaps = [
  new RangeMap(
    lines
      .slice(seedSoilMapIndex + 1, soilFertilizerMapIndex)
      .filter((a) => a)
      .map(toRange)
  ),
  new RangeMap(
    lines
      .slice(soilFertilizerMapIndex + 1, fertilizerWaterMapIndex)
      .filter((a) => a)
      .map(toRange)
  ),
  new RangeMap(
    lines
      .slice(fertilizerWaterMapIndex + 1, waterLightMapIndex)
      .filter((a) => a)
      .map(toRange)
  ),
  new RangeMap(
    lines
      .slice(waterLightMapIndex + 1, lightTemperatureMapIndex)
      .filter((a) => a)
      .map(toRange)
  ),
  new RangeMap(
    lines
      .slice(lightTemperatureMapIndex + 1, temperatureHumidityMapIndex)
      .filter((a) => a)
      .map(toRange)
  ),
  new RangeMap(
    lines
      .slice(temperatureHumidityMapIndex + 1, humidityLocationMapIndex)
      .filter((a) => a)
      .map(toRange)
  ),
  new RangeMap(
    lines
      .slice(humidityLocationMapIndex + 1)
      .filter((a) => a)
      .map(toRange)
  ),
];

const mappedSeeds = seeds.map((seed) => {
  let value = seed;

  for (const rangemap of rangemaps) {
    value = rangemap.get(value);
  }

  return value;
});

function getValueAndBound(value) {
  let bound = Number.MAX_SAFE_INTEGER;
  let val = value;

  for (const map of rangemaps) {
    const [computedValue, computedBound] = map.getBound(val);
    bound = Math.min(bound, computedBound);
    val = computedValue;
  }

  return [val, bound];
}

let min = Infinity;
for (let i = 0; i < seeds.length; i += 2) {
  for (let j = seeds[i]; j < seeds[i] + seeds[i + 1]; j++) {
    const [value, bound] = getValueAndBound(j);
    if (value < min) {
      min = value;
    }
    j += bound;
  }
}

console.log(Math.min(...mappedSeeds));
console.log(min);
