const { readFileSync } = require('fs');

const inputLines = readFileSync('../input.txt').toString().split("\n");

const scannerMap = {};
let index = -1;

inputLines.forEach((line) => {
  if (line === '') {
    return;
  } else if (line.includes('-')) {
    index += 1;
    scannerMap[index] = [];
  } else {
    scannerMap[index].push({ distances: [], coords: line.split(',').map(x => parseInt(x)) });
  }
});

for (let i = 0; i <= index; i++) {
  const scanner = scannerMap[i];
  for (let j = 0; j < scanner.length; j++) {
    for (let k = j + 1; k < scanner.length; k++) {
      const b0 = scanner[j].coords;
      const b1 = scanner[k].coords;

      const distance = Math.sqrt(Math.pow(b0[0] - b1[0], 2), Math.pow(b0[1] - b1[1], 2), Math.pow(b0[2] - b1[2], 2));

      scanner[j].distances.push(distance);
      scanner[k].distances.push(distance);
    }

    scanner[j].distances.sort((a, b) => a - b);
  }
}


