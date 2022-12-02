const { readFileSync } = require('fs');

const inputLines = readFileSync('../input.txt').toString().split("\n");

const calledNumbers = inputLines[0].split(",").map(x => parseInt(x.trim()));

const boards = [];

function buildBoard(lines) {
  const board = {};

  lines.forEach((line, i) => {
    const numbers = line.split(" ").filter(x => x).map(x => x.trim());
    numbers.forEach((num, j) => {
      board[num] = [i, j];
    });
  });

  return board;
}

let i = 6;
while (i < inputLines.length) {
  boards.push(buildBoard(inputLines.slice(i - 4, i + 1)));
  i += 6;
}

function createEmptyScore() {
  return new Array(10).fill(0);
}

function evaluateBoard(board, calls) {
  const score = createEmptyScore();

  for (let i = 0; i < calls.length; i++) {
    if (board[calls[i]] === undefined) {
      continue;
    }

    const [row, column] = board[calls[i]];

    score[row] += 1;
    if (score[row] === 5) {
      return i;
    }

    score[column + 5] += 1;
    if (score[column + 5] === 5) {
      return i;
    }
  }
}

function scoreBoard(board, calls) {
  const boardCopy = { ...board };
  calls.forEach(num => delete boardCopy[num]);
  return Object.keys(boardCopy).reduce((acc, cur) => acc + parseInt(cur), 0) * calls[calls.length - 1];
}

let bestBoard = [null, Infinity];
let worstBoard = [null, -Infinity];

boards.forEach(b => {
  const score = evaluateBoard(b, calledNumbers);
  if (score < bestBoard[1]) {
    bestBoard = [b, score];
  }

  if (score > worstBoard[1]) {
    worstBoard = [b, score];
  }
});

console.log(`Part 1: ${scoreBoard(bestBoard[0], calledNumbers.slice(0, bestBoard[1] + 1))}`);
console.log(`Part 2: ${scoreBoard(worstBoard[0], calledNumbers.slice(0, worstBoard[1] + 1))}`);
