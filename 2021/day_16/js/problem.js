const { readFileSync } = require('fs');

const input = readFileSync('../input.txt').toString();
let binaryString = '';
for (let i of input) {
  if (i === "" || i === '\n') {
    continue;
  }
  binaryString += parseInt(i, 16).toString(2).padStart(4, '0');
}

let versionSum = 0;
let index = 0;

function parse() {
  const version = parseInt(binaryString.slice(index, index + 3), 2);
  versionSum += version;
  const type = parseInt(binaryString.slice(index + 3, index + 6), 2);
  index = index + 6;

  if (type === 4) {
    return parseLiteral();
  } else {
    return parseOperator(type);
  }
}

function parseLiteral() {
  let binaryNumber = '';

  let next = binaryString.slice(index, index + 5);
  binaryNumber = next.slice(1);
  index += 5;
  while (next[0] === '1') {
    next = binaryString.slice(index, index + 5);
    binaryNumber += next.slice(1);
    index += 5;
  }

  return parseInt(binaryNumber, 2);
}

function parseOperator(type) {
  const lengthType = binaryString[index];
  index += 1;
  let results = [];

  if (lengthType === '0') {
    const bitLength = parseInt(binaryString.slice(index, index + 15), 2);
    index += 15;
    const end = index + bitLength;

    while (index < end) {
      results.push(parse());
    }
  } else {
    const packetLength = parseInt(binaryString.slice(index, index + 11), 2);
    index += 11;

    for (let i = 0; i < packetLength; i++) {
      results.push(parse());
    }
  }

  if (type === 0) {
    return results.reduce((acc, cur) => acc + cur, 0);
  } else if (type === 1) {
    return results.reduce((acc, cur) => acc * cur, 1);
  } else if (type === 2) {
    let min = Infinity;
    results.forEach((r) => {
      if (r < min) {
        min = r;
      }
    });
    return min;
  } else if (type === 3) {
    let max = 0;
    results.forEach((r) => {
      if (r > max) {
        max = r;
      }
    });
    return max;
  } else {
    let func;
    if (type === 5) {
      func = (a, b) => {
        if (a > b) {
          return 1;
        }
        return 0;
      }
    } else if (type === 6) {
      func = (a, b) => {
        if (a < b) {
          return 1;
        }
        return 0;
      }
    } else {
      func = (a, b) => {
        if (a === b) {
          return 1;
        }
        return 0;
      }
    }

    return func(results[0], results[1]);
  }
}

let finalResult;
while (index < binaryString.length - 6) {
  finalResult = parse();
}

console.log(`Part 1: ${versionSum}`);
console.log(`Part 2: ${finalResult}`);
