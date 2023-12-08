import { readFileSync } from "fs";

const STANDARD_IN = 0;
const lines = readFileSync(STANDARD_IN).toString().split("\n").slice(0, -1);

const scoreTypeInner = function (hand) {
  const counts = hand.split("").map((card, index, self) => {
    return self.filter((c) => c === card).length;
  });

  // five of a kind
  if (counts.includes(5)) {
    return 6;
  }

  // four of a kind
  if (counts.includes(4)) {
    return 5;
  }

  // full house
  if (counts.includes(3) && counts.includes(2)) {
    return 4;
  }

  // three of a kind
  if (counts.includes(3)) {
    return 3;
  }

  // two pair
  if (counts.filter((count) => count === 2).length === 4) {
    return 2;
  }

  // one pair
  if (counts.includes(2)) {
    return 1;
  }

  // high card
  return 0;
};

const scoreType = function (hand, part1 = true) {
  if (part1) {
    return scoreTypeInner(hand);
  }

  const nonJokerCards = hand.split("").filter((card) => card !== "J");
  if (nonJokerCards.length === 0) {
    return scoreTypeInner(hand);
  }

  let currentBest = -1;
  for (let i = 0; i < nonJokerCards.length; i++) {
    const card = nonJokerCards[i];

    const toTest = hand.replaceAll("J", card);
    const score = scoreTypeInner(toTest);
    if (score > currentBest) {
      currentBest = score;
    }
  }

  return currentBest;
};

const cardValues = function (part1 = true) {
  return {
    2: 2,
    3: 3,
    4: 4,
    5: 5,
    6: 6,
    7: 7,
    8: 8,
    9: 9,
    T: 10,
    J: part1 ? 11 : 1,
    Q: 12,
    K: 13,
    A: 14,
  };
};

const sortForPart = function (input, part1 = true) {
  return input.sort((a, b) => {
    const Ahand = a.split(" ")[0];
    const AhandScore = scoreType(Ahand, part1);

    const Bhand = b.split(" ")[0];
    const BhandScore = scoreType(Bhand, part1);

    if (AhandScore === BhandScore) {
      const perCardScore = cardValues(part1);
      const AhandValues = Ahand.split("").map((card) => perCardScore[card]);
      const BhandValues = Bhand.split("").map((card) => perCardScore[card]);

      for (let i = 0; i < AhandValues.length; i++) {
        if (AhandValues[i] !== BhandValues[i]) {
          return AhandValues[i] - BhandValues[i];
        }
      }

      return 0;
    }

    return AhandScore - BhandScore;
  });
};

const sortedLinesPart1 = sortForPart(lines, true);
const rank = sortedLinesPart1.reduce((prev, curr, index, array) => {
  return prev + (index + 1) * parseInt(curr.split(" ")[1]);
}, 0);

console.log(rank);

const sortedLinesPart2 = sortForPart(lines, false);
const rankPart2 = sortedLinesPart2.reduce((prev, curr, index, array) => {
  return prev + (index + 1) * parseInt(curr.split(" ")[1]);
}, 0);

console.log(rankPart2);
