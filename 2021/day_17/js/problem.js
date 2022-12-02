const { readFileSync } = require('fs');

const contents = readFileSync('../input.txt').toString();
const regex = /-?\d*\.\.-?\d*/g;
const coordinates = contents.match(regex).map(x => x.split('..').map(y => parseInt(y)));

let xCoord = coordinates[0][1];
let maxIndex = 0;
const validX = {};

while (xCoord > 0) {
    let index = 0;
    let position = 0;
    while (index <= xCoord && position < coordinates[0][1]) {
        position += xCoord - index;
        index += 1;

        if (position >= coordinates[0][0] && position <= coordinates[0][1]) {
            if (validX[index]) {
                validX[index].push(xCoord);
            } else {
                validX[index] = [xCoord];
            }
            if (index > maxIndex) {
                maxIndex = index;
            }

            if ((index - 1) === xCoord) {
                if (validX.max) {
                    validX.max.push(xCoord);
                } else {
                    validX.max = [xCoord];
                }
            }
        }
    }
    xCoord -= 1;
}

let maxY = 0;
let count = 0;
let yCoord = coordinates[1][0];

while (yCoord <= Math.abs(coordinates[1][0] - 1)) {
    let value = Infinity;
    let index = 1;
    const hits = {};

    while (value > coordinates[1][0]) {
        if (yCoord < 0) {
            value = - ((yCoord - index + 1) * (yCoord - index + 2)) / 2 + (Math.pow(yCoord, 2) + 3 * yCoord + 2) / 2 - index;
        } else {
            value = index * (yCoord + 1) - index * (index + 1) / 2;
        }

        if (value >= coordinates[1][0] && value <= coordinates[1][1]) {
            if (yCoord > maxY) {
                maxY = yCoord;
            }

            if (index > maxIndex) {
                validX.max.forEach(k => {
                    hits[k] = true;
                });
            } else if (validX[index]) {
                validX[index].forEach(k => {
                    hits[k] = true;
                });
            }
        }
        index += 1;
    }

    count += Object.keys(hits).length;

    yCoord += 1;
}

console.log(`Part 1: ${maxY * (maxY + 1) / 2}`);
console.log(`Part 2: ${count}`);
