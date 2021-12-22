import {
    readFileSync
} from "fs";
const STANDARD_IN = 0;
let raw = readFileSync(STANDARD_IN).toString().split("").map(c => {
    if (c == "0") return "0000";
    if (c == "1") return "0001";
    if (c == "2") return "0010";
    if (c == "3") return "0011";
    if (c == "4") return "0100";
    if (c == "5") return "0101";
    if (c == "6") return "0110";
    if (c == "7") return "0111";
    if (c == "8") return "1000";
    if (c == "9") return "1001";
    if (c == "A") return "1010";
    if (c == "B") return "1011";
    if (c == "C") return "1100";
    if (c == "D") return "1101";
    if (c == "E") return "1110";
    if (c == "F") return "1111";
}).join("");

const packets = [];

class Packet {
    constructor(version, id, data) {
        this.version = version;
        this.id = id;
        this.data = data;
    }

    value() {
        if (this.id == 0) {
            return this.data.reduce((o, n) => o + n.value(), 0);
        }
        if (this.id == 1) {
            return this.data.reduce((o, n) => o * n.value(), 1);
        }
        if (this.id == 2) {
            return Math.min(...this.data.map(p => p.value()));
        }
        if (this.id == 3) {
            return Math.max(...this.data.map(p => p.value()));
        }
        if (this.id == 4) {
            return this.data;
        }
        if (this.id == 5) {
            return this.data[0].value() > this.data[1].value() ? 1 : 0;
        }
        if (this.id == 6) {
            return this.data[0].value() < this.data[1].value() ? 1 : 0;
        }
        if (this.id == 7) {
            return this.data[0].value() == this.data[1].value() ? 1 : 0;
        }
    }
}

function parsePacket(input, index) {
    const version = parseInt(input.substring(index, index + 3), 2);
    const type = parseInt(input.substring(index + 3, index + 6), 2);
    const [offset, data] = parseVersion(input, type, index + 6);
    const packet = new Packet(version, type, data);

    return [offset, packet];
}

function parseVersion(input, type, index) {
    if (type == 4) {
        return parseHighBitNumber(input, index);
    } else {
        const isBitCount = input.substring(index, index + 1) == "0";
        const data = [];
        let readOffset = 0;
        if (isBitCount) {
            const length = parseInt(input.substring(index + 1, index + 16), 2);
            const packetString = input.substring(index + 16, index + 16 + length);

            while (readOffset < length) {
                const [offset, content] = parsePacket(packetString, readOffset);
                readOffset = offset;
                data.push(content);
            }

            return [index + length + 16, data];
        } else {
            const packetCount = parseInt(input.substring(index + 1, index + 12), 2);
            readOffset = index + 12;
            let parsedCount = 0;
            while (parsedCount < packetCount) {
                const [offset, content] = parsePacket(input, readOffset);
                readOffset = offset;
                parsedCount++;
                data.push(content);
            }

            return [readOffset, data];
        }
    }
}

function parseHighBitNumber(input, index) {
    let data = "";
    let currentByteValue = 0;
    let offset = 0;
    do {
        const currentByte = input.substring(index + offset, index + offset + 5);
        currentByteValue = parseInt(currentByte, 2);
        data += currentByte.substring(1);
        offset += 5;
    } while (currentByteValue >= 16);

    return [index + offset, parseInt(data, 2)];
}


function sumVersionNumbers(packet) {
    let count = packet.version;

    if (Array.isArray(packet.data)) {
        count += packet.data.map(p => sumVersionNumbers(p)).reduce((o, n) => o + n, 0);
    }

    return count;
}

const packet = parsePacket(raw, 0)[1];
console.log(sumVersionNumbers(packet));
console.log(packet.value());

