import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const parts = [];

for (let y = 0; y < lines.length; y++) {
  const line = lines[y];
  for (let x = 0; x < line.length; x++) {
    const char = line[x];
    const number = parseInt(char);
    if (!isNaN(number)) {
      const partStart = x;
      while (!isNaN(parseInt(line[x]))) {
        x++;
      }
      const partEnd = x;

      const part = line.slice(partStart, partEnd);
      const partNumber = parseInt(part);

      let foundPart = false;

      for (
        let checkY = Math.max(y - 1, 0);
        checkY < Math.min(y + 2, line.length);
        checkY++
      ) {
        for (
          let checkX = Math.max(partStart - 1, 0);
          checkX < Math.min(partEnd + 1, line.length);
          checkX++
        ) {
          if (!lines[checkY][checkX].toString().match(/[0-9.]/g)) {
            const partType = lines[checkY][checkX];
            parts.push({
              number: partNumber,
              type: partType,
              partIndex: {
                x: checkX,
                y: checkY,
              },
            });
            foundPart = true;
          }
        }
      }

      if (!foundPart) {
        parts.push({
          number: partNumber,
          type: "none",
        });
      }
    }
  }
}

console.log(
  parts
    .filter((part) => part.type != "none")
    .reduce((acc, part) => acc + part.number, 0)
);


const gears = parts.filter((part) => part.type == "*");
const gearConnections = gears
  .filter((part, index) =>
    gears.some(
      (otherPart, otherIndex) =>
        index != otherIndex &&
        part.partIndex.x == otherPart.partIndex.x &&
        part.partIndex.y == otherPart.partIndex.y
    )
  )
  .map((part) => {
    const key = `${part.partIndex.x}-${part.partIndex.y}`;
    const value = part.number;

    return {
      key,
      value,
    };
  });


const validConnections = gearConnections.filter(
  (connection) =>
    gearConnections.filter(
      (otherConnection) => otherConnection.key == connection.key
    ).length == 2
);

const connectionData = {};
validConnections.forEach((connection) => {
  if (!connectionData[connection.key]) {
    connectionData[connection.key] = 1;
  }

  connectionData[connection.key] *= connection.value;
});

console.log(Object.values(connectionData).reduce((acc, next) => acc + next, 0));
