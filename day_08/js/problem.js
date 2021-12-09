const { readFileSync } = require('fs');

const data = readFileSync('../input.txt')
  .toString()
  .split('\n')
  .filter(x => x !== "")
  .map(line =>
    line.split(" | ")
      .map(x => x.split(" ")
        .filter(a => a != "")));

const values = {
  'a': 2,
  'b': 3,
  'c': 5,
  'd': 7,
  'e': 11,
  'f': 13,
  'g': 17,
};

let firstPartCount = 0;
const sum = data.reduce((acc, datum) => {
  const [inputs, outputs] = datum;

  const unknowns = { '5': [], '6': [] };
  const knownValues = new Array(10);

  inputs.forEach((i) => {
    let value = 1;
    for (let char of i) {
      value *= values[char];
    }

    if (i.length === 2) {
      knownValues[1] = value;
    } else if (i.length === 3) {
      knownValues[7] = value;
    } else if (i.length === 4) {
      knownValues[4] = value;
    } else if (i.length === 5) {
      unknowns['5'].push(value);
    } else if (i.length === 6) {
      unknowns['6'].push(value);
    } else if (i.length === 7) {
      knownValues[8] = value;
    }
  });

  // Find 3, 5, and 6
  for (let unknown of unknowns['5']) {
    if (unknown % knownValues[1] === 0) {
      knownValues[3] = unknown;
    } else if (unknown % (knownValues[4] / knownValues[1]) === 0) {
      knownValues[5] = unknown;
    } else {
      knownValues[2] = unknown;
    }
  }

  // Find 6, 9, and 0
  for (let unknown of unknowns['6']) {
    if (unknown % knownValues[3] === 0) {
      knownValues[9] = unknown;
    } else if (unknown % knownValues[1] === 0) {
      knownValues[0] = unknown;
    } else {
      knownValues[6] = unknown;
    }
  }

  const outputValue = outputs.reduce((acc, cur) => {
    let value = 1;
    for (let char of cur) {
      value *= values[char];
    }

    const index = knownValues.findIndex(x => x === value);
    if (index === 1 || index === 4 || index === 7 || index === 8) {
      firstPartCount++;
    }

    return acc + index.toString();
  }, '');

  return acc += parseInt(outputValue);
}, 0);

console.log(`Part 1: ${firstPartCount}`);
console.log(`Part 2: ${sum}`);
