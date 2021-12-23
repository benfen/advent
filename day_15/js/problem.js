const { readFileSync } = require('fs');

const initialCavern = readFileSync('../input.txt').toString().split('\n').filter(line => line !== '').map((line) => {
    const items = line.split('');
    return items.map(x => parseInt(x));
});

const smallCavern = initialCavern.map(x => x.map(value => ({ value, score: Infinity })));

const largeCavern = new Array(5 * initialCavern.length).fill(0).map((_arr, x) => {
    let xOffset = Math.floor(x / initialCavern.length);
    return new Array(5 * initialCavern[0].length).fill(0).map((_arr2, y) => {
        let yOffset = Math.floor(y / initialCavern[0].length);

        let value = xOffset + yOffset + initialCavern[x % initialCavern.length][y % initialCavern[0].length];
        while (value > 9) {
            value -= 9;
        }
        return { value, score: Infinity };
    });
});

function checkCavern(cavern) {
    cavern[0][0].score = 0;
    let xMax = cavern.length - 1;
    let yMax = cavern[0].length - 1;

    const stack = [
        [1, 0, 0],
        [0, 1, 0],
    ];

    while (stack.length > 0) {
        const newStack = [];
        while (stack.length > 0) {
            const [x, y, value] = stack.pop();
            if (x > xMax || x < 0 || y > yMax || y < 0) {
                continue;
            }

            const chiton = cavern[x][y];
            const newValue = chiton.value + value;
            if (newValue < chiton.score && newValue < cavern[xMax][yMax].score) {
                chiton.score = newValue;
                newStack.push([x - 1, y, newValue]);
                newStack.push([x + 1, y, newValue]);
                newStack.push([x, y - 1, newValue]);
                newStack.push([x, y + 1, newValue]);
            }
        }
        stack.push(...newStack);
    }
}

checkCavern(smallCavern);
checkCavern(largeCavern);
console.log(`Part 1: ${smallCavern[smallCavern.length - 1][smallCavern[0].length - 1].score}`);
console.log(`Part 2: ${largeCavern[largeCavern.length - 1][largeCavern[0].length - 1].score}`);
