import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n\n").map(l => l.split("\n"));

function compare(leftString, rightString) {
    const left = eval(leftString);
    const right = eval(rightString);

    if (Number.isInteger(left) && Number.isInteger(right)) {
        if (left < right) {
            return true;
        }

        if (right < left) {
            return false;
        }

        if (right == left) {
            return null;
        }
    }

    if (Array.isArray(left) && Array.isArray(right)) {
        for (let i = 0; i < Math.max(left.length, right.length); i++) {
            const leftElem = left[i];
            const rightElem = right[i];

            if (leftElem == undefined && rightElem == undefined) {
                return true;
            }

            if (leftElem == undefined && rightElem != undefined) {
                return true;
            }

            if (rightElem == undefined && leftElem != undefined) {
                return false;
            }

            const state = compare(leftElem, rightElem);
            if (state != null) {
                return state;
            }
        }
    }

    if (Number.isInteger(left)) {
        return compare([left], right);
    }

    if (Number.isInteger(right)) {
        return compare(left, [right]);
    }
}

const validIndecies = [];
for (let i = 0; i < lines.length; i++) {
    if (compare(...lines[i])) {
        validIndecies.push(i + 1);
    }
}

console.log(validIndecies.reduce((o, n) => o + n, 0));

let packetList = lines.map(l => l.join("\n")).join("\n");
packetList += "[[2]]\n[[6]]";

const sorted = packetList.split("\n").sort((a, b) => compare(a, b) ? -1 : 1);
const twoIndex = sorted.indexOf("[[2]]") + 1;
const sixIndex = sorted.indexOf("[[6]]") + 1;
console.log(twoIndex * sixIndex);