import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
let input = readFileSync(STANDARD_IN).toString().split("\n").map(line => line.split("-"));

const edges = {};
input.forEach(([from, to]) => addEdge(from, to));

function createConnection(from, to) {
    if (from == "" || from == undefined) {
        return;
    }

    let vertex = from in edges ? edges[from] : {
        connections: new Set(),
        isBig: /[A-Z]/.test(from),
        key: from,
        once: ["start", "end"].includes(from),
    }

    vertex.connections.add(to);
    edges[from] = vertex;
}

function addEdge(from, to) {
    createConnection(from, to);
    createConnection(to, from);
}

function generateAllPathsRecursive(from, path, visited, part1, hasVisistedSmall) {
    let pathCount = 0;
    const newPath = [...path];

    if (visited.has(from.key)) {
        if (part1) return pathCount;

        if (hasVisistedSmall || from.once) return pathCount;
        else hasVisistedSmall = true;
    } else if (!from.isBig) visited.add(from.key);

    newPath.push(from.key);

    [...from.connections].forEach(link => {
        if (link == "end") pathCount++;
        else pathCount += generateAllPathsRecursive(edges[link], newPath, new Set([...visited]), part1, hasVisistedSmall);
    });

    return pathCount;
}

console.log(generateAllPathsRecursive(edges["start"], [], new Set(), true, false));
console.log(generateAllPathsRecursive(edges["start"], [], new Set(), false, false));