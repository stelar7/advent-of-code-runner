import { readFileSync } from "fs";

const STANDARD_IN = 0;
const input = readFileSync(STANDARD_IN).toString();
const boards = input.split("\n\n");
const numbers = boards[0].split(",");
const regex = / *(\d+) +(\d+) +(\d+) +(\d+) +(\d+)/g;

class Board {
    constructor(data) {
        this.data = data;
    }

    markNumber(n) {
        for (let i = 0; i < this.data.length; i++) {
            for (let j = 0; j < this.data[0].length; j++) {
                if (this.data[j][i] == n) {
                    this.data[j][i] = "X";
                }
            }
        }
    }

    isMarked(x, y) {
        return this.data[y][x] == "X";
    }

    isWinner() {
        for (let i = 0; i < this.data.length; i++) {
            if (this.isMarked(i,0) && this.isMarked(i,1) && this.isMarked(i,2) && this.isMarked(i,3) && this.isMarked(i,4)) {
                return true;
            }

            if (this.isMarked(0,i) && this.isMarked(1,i) && this.isMarked(2,i) && this.isMarked(3,i) && this.isMarked(4,i)) {
                return true;
            }
        }

        return false;
    }

    sumOfUnmarked() {
        return this.data.flat().reduce((o,n) => o + (parseInt(n) || 0), 0);
    }

    toString() {
        return JSON.stringify(this.data) + " -- " + this.isWinner() + " -- " + this.sumOfUnmarked();
    }
}

const boardData = [];
for (let i = 1; i < boards.length; i++) {
    
    let tempBoard = [];
    const board = boards[i];

    for (let j = 0; j < board.split("\n").length; j++) {
        const line = board.split("\n")[j];
        const row = [...line.matchAll(regex)][0];
        if (row) {
            tempBoard.push([row[1],row[2],row[3],row[4],row[5]]);
        }
    }

    boardData.push(new Board(tempBoard));
}

function findPart1() {
    for (let i = 0; i < numbers.length; i++) {
        const num = numbers[i];
        
        for (let j = 0; j < boardData.length; j++) {
            const board = boardData[j];
            board.markNumber(num);
            if (board.isWinner()) {
                return board.sumOfUnmarked() * num;
            }
        }
    }
}

function findPart2() {
    let lastWinner = null;
    let lastNum = null;
    for (let i = 0; i < numbers.length; i++) {
        const num = numbers[i];
        
        for (let j = 0; j < boardData.length; j++) {
            const board = boardData[j];
            if (board.isWinner()) {
                continue;
            }

            
            board.markNumber(num);
            lastWinner = board;
            lastNum = num;
        }
    }

    return lastWinner.sumOfUnmarked() * lastNum;
}

console.log(findPart1());
console.log(findPart2());