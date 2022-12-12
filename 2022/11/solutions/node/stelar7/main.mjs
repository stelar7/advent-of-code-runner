import {
    readFileSync
} from "fs";
import {
    exit
} from "process";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

class Monkey {
    initialItems = [];
    items = [];
    operation = null;
    test = null;
    ifTrue = null;
    ifFalse = null;
}

let currentMonkey = null;
let monkies = [];
let modulo = BigInt(1);

for (const element of lines) {
    const line = element;

    if (line.startsWith("Monkey")) {
        currentMonkey = new Monkey();
    }

    if (line.length == 0) {
        monkies.push(currentMonkey);
    }

    if (line.startsWith("  Starting items:")) {
        currentMonkey.items = line.split(":")[1].split(", ").map(i => BigInt(+i));
        currentMonkey.initialItems = line.split(":")[1].split(", ").map(i => BigInt(+i));
    }

    if (line.startsWith("  Operation:")) {
        let ops = line.split(":")[1].split("= ")[1].split(" ");
        let actionMap = {
            "+": (x, y) => x + y,
            "-": (x, y) => x - y,
            "*": (x, y) => x * y,
            "/": (x, y) => x / y,
        };

        const operation = actionMap[ops[1]];

        if (ops[0] == "old") {
            if (ops[2] == "old") {
                currentMonkey.operation = (o) => operation(o, o);
            } else {
                let value = BigInt(+ops[2]);
                currentMonkey.operation = (o) => operation(o, value);
            }
        } else if (ops[2] == "old") {
            if (ops[0] == "old") {
                currentMonkey.operation = (o) => operation(o, o);
            } else {
                let value = BigInt(+ops[0]);
                currentMonkey.operation = (o) => operation(value, o);
            }
        }

    }

    if (line.startsWith("  Test:")) {
        let divTest = BigInt(+line.split(" divisible by ")[1]);
        currentMonkey.test = (v) => v % divTest == 0;
        modulo *= divTest;
    }

    if (line.startsWith("    If true:")) {
        let toMonkey = +line.split(" throw to monkey ")[1];
        currentMonkey.ifTrue = (v) => monkies.at(toMonkey).items.push(v);
    }

    if (line.startsWith("    If false:")) {
        let toMonkey = +line.split(" throw to monkey ")[1];
        currentMonkey.ifFalse = (v) => monkies.at(toMonkey).items.push(v);
    }
}

monkies.push(currentMonkey);

function doRound(div = false) {
    for (let i = 0; i < monkies.length; i++) {
        const monkey = monkies[i];
        while (monkey.items.length > 0) {
            if (!inspectCount[i]) {
                inspectCount[i] = 0;
            }

            inspectCount[i]++;

            let item = monkey.items.shift();
            item = monkey.operation(item);
            if (div) {
                item = item / BigInt(3);
            }

            item %= modulo;

            if (monkey.test(item)) {
                monkey.ifTrue(item);
            } else {
                monkey.ifFalse(item);
            }
        }
    }
}

let inspectCount = [];
for (let i = 0; i < 20; i++) {
    doRound(true);
}

const sortedInspections = inspectCount.sort((a, b) => b - a);
const output = sortedInspections[0] * sortedInspections[1];

console.log(output);

inspectCount = [];
monkies.forEach(m => m.items = []);
monkies.forEach(m => m.initialItems.forEach(i => m.items.push(BigInt(i))));
for (let i = 0; i < 10000; i++) {
    doRound(false);
}

const sortedInspections2 = inspectCount.sort((a, b) => b - a);
const output2 = sortedInspections2[0] * sortedInspections2[1];

console.log(output2);