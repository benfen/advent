const { readFileSync } = require('fs');

const input = readFileSync('../input.txt').toString().split('\n');

function addSet(obj, key, value) {
  if (obj[key]) {
    obj[key] += value;
  } else {
    obj[key] = value;
  }
}

let startPattern = {};
for (let i = 0; i < input[0].length - 1; i++) {
  const patternSlice = input[0].slice(i, i + 2);
  addSet(startPattern, patternSlice, 1);
}

const rules = {};
for (let i = 2; i < input.length; i++) {
  if (input[i] === "") {
    continue;
  }

  const items = input[i].split(" -> ");
  rules[items[0]] = [items[0][0] + items[1], items[1] + items[0][1]];
}

function getDiff(initialPattern, iterations) {
  let pattern = initialPattern;
  for (let i = 0; i < iterations; i++) {
    let newPattern = {};

    Object.entries(pattern).forEach((pattern) => {
      const p = pattern[0];
      const count = pattern[1];

      addSet(newPattern, rules[p][0], count);
      addSet(newPattern, rules[p][1], count);
    });

    pattern = newPattern;
  }

  const counts = {};
  addSet(counts, input[0][0], 1);
  addSet(counts, input[0][input[0].length - 1], 1);

  Object.entries(pattern).forEach((pattern) => {
    const p = pattern[0];
    const count = pattern[1];

    addSet(counts, p[0], count);
    addSet(counts, p[1], count);
  });

  let min = Infinity, max = 0;
  Object.entries(counts).forEach((count) => {
    if (count[1] / 2 > max) {
      max = count[1] / 2;
    }

    if (count[1] / 2 < min) {
      min = count[1] / 2;
    }
  });

  return max - min;
}

console.log(`Part 1: ${getDiff({ ...startPattern }, 10)}`);
console.log(`Part 2: ${getDiff({ ...startPattern }, 40)}`);
