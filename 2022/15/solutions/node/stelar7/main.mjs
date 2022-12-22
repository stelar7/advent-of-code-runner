import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

function distance(a, b) {
    return Math.abs(a.x - b.x) + Math.abs(a.y - b.y)
}

function parseSensors(lines) {
    const sensors = [];
    for (const line of lines) {
        const parts = line.split(":");
        const sensorString = parts[0].split(",");
        const sensorX = parseInt(sensorString[0].split("x=")[1]);
        const sensorY = parseInt(sensorString[1].split("y=")[1]);

        const beaconString = parts[1].split(",");
        const beaconX = parseInt(beaconString[0].split("x=")[1]);
        const beaconY = parseInt(beaconString[1].split("y=")[1]);

        const sensor = {
            x: sensorX,
            y: sensorY
        };
        const beacon = {
            x: beaconX,
            y: beaconY
        };
        sensor.radius = distance(sensor, beacon);
        sensors.push(sensor);
    }

    return sensors;
}

function parseIntervals(sensors) {
    const intervals = [];
    for (const sensor of sensors) {
        const distanceToLine = sensor.radius - Math.abs(sensor.y - rowToCheck);
        if (distanceToLine >= 0) {
            const interval = {
                min: sensor.x - distanceToLine,
                max: sensor.x + distanceToLine
            };

            intervals.push(interval);
        }
    }

    intervals.sort((a, b) => a.min - b.min);

    const finalIntervals = [intervals[0]];
    for (let i = 1; i < intervals.length; i++) {
        const next = intervals[i];

        if (next.min <= finalIntervals[finalIntervals.length - 1].max) {
            finalIntervals[finalIntervals.length - 1].max = Math.max(finalIntervals[finalIntervals.length - 1].max, next.max);
        } else {
            finalIntervals.push(next);
        }
    }

    let intervalLength = finalIntervals[0].max + Math.abs(finalIntervals[0].min);

    for (const sensor of sensors) {
        if (sensor.y == rowToCheck) {
            intervalLength--;
        }
    }
    return intervalLength;
}

function findFreeIntersect(maxDistance) {
    const diagonalOne = [];
    const diagonalTwo = [];

    for (const sensor of sensors) {
        diagonalOne.push(sensor.y - sensor.x + sensor.radius + 1);
        diagonalOne.push(sensor.y - sensor.x - sensor.radius - 1);

        diagonalTwo.push(sensor.y + sensor.x + sensor.radius + 1);
        diagonalTwo.push(sensor.y + sensor.x - sensor.radius - 1);
    }

    for (const first of diagonalOne) {
        for (const second of diagonalTwo) {
            const intersectA = Math.floor((second - first) / 2);
            const intersectB = Math.floor((first + second) / 2);
            const intersectPoint = {
                x: intersectA,
                y: intersectB
            }

            if (!(intersectA > 0 && intersectA < maxDistance && intersectB > 0 && intersectB < maxDistance)) {
                continue;
            }

            let bad = false;
            for (const sensor of sensors) {
                const outsideDistance = distance(intersectPoint, sensor);

                if (outsideDistance <= sensor.radius) {
                    bad = true;
                }
            }

            if (!bad) {
                return intersectPoint;
            }
        }
    }

    return null;
}

const rowToCheck = 2000000;
const intersectLimit = 4000000;

const sensors = parseSensors(lines);
console.log(parseIntervals(sensors));

const intersect = findFreeIntersect(intersectLimit);
console.log(intersect.x * 4000000 + intersect.y);