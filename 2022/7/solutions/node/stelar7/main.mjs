import {
    readFileSync
} from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const filesystem = [];
let currentPath = "";

function handleCommand([prefix, command, path]) {
    if (command == "ls") {
        return;
    }

    if (command == "cd") {
        if (path == "..") {
            const lastIndex = currentPath.substring(0, currentPath.length - 1).lastIndexOf("/");
            const newPath = currentPath.substring(0, lastIndex + 1);
            currentPath = newPath;
            return;
        }

        if (path == "/") {
            currentPath = "/";
            return;
        }

        const newPath = currentPath + path + "/";
        currentPath = newPath;
    }
}

for (let command of lines) {
    if (command.startsWith("$")) {
        handleCommand(command.split(" "));
        continue;
    }

    if (command.startsWith("dir")) {
        if (!filesystem[currentPath]) {
            filesystem[currentPath] = [];
        }

        filesystem[currentPath].push({
            type: "dir",
            name: command.split(" ")[1]
        });

        continue;
    }

    if (!filesystem[currentPath]) {
        filesystem[currentPath] = [];
    }

    filesystem[currentPath].push({
        type: "file",
        name: command.split(" ")[1],
        size: +command.split(" ")[0],
    });
}

const pathSizes = [];
const paths = Object.keys(filesystem)
    .sort((a, b) => (b.match(/\//g) || []).length - (a.match(/\//g) || []).length)
    .forEach(path => {
        const folder = filesystem[path];

        for (let item of folder) {
            if (item.type == "file") {
                if (!pathSizes[path]) {
                    pathSizes[path] = 0;
                }

                pathSizes[path] += item.size;
            }

            if (item.type == "dir") {
                if (!pathSizes[path]) {
                    pathSizes[path] = 0;
                }

                pathSizes[path] += pathSizes[path + item.name + "/"];
            }
        }
    });

    const sumOfBelow100K = Object.values(pathSizes).reduce((l, n) => n < 100_000 ? l + n : l, 0);
    console.log(sumOfBelow100K);


    const neededFreeSpace = 30_000_000;
    const maxSpace = 70_000_000;
    const usedSpace = pathSizes["/"];
    const currentFreeSpace = maxSpace - usedSpace;
    const minDirSize = neededFreeSpace - currentFreeSpace;

    const deletableOptions = Object.values(pathSizes).sort((a, b) => a - b).filter(a => a >= minDirSize);
    console.log(deletableOptions[0]);