const { readFileSync } = require('fs');

const inputs = readFileSync('../input.txt')
    .toString()
    .split('\n')
    .map((str) => {
        let nums = [];
        for (let char of str) {
            nums.push(parseInt(char));
        }
        return nums;
    });

let holeSum = 0;
for (let i = 0; i < inputs.length; i++) {
    for (let j = 0; j < inputs[i].length; j++) {
        const item = inputs[i][j];
        if (
            !(item >= (inputs[i - 1] || [])[j]) &&
            !(item >= (inputs[i + 1] || [])[j]) &&
            !(item >= inputs[i][j - 1]) &&
            !(item >= inputs[i][j + 1])
        ) {
            holeSum += item + 1;
        }
    }
}

const markedInputs = inputs.map((i) => i.map((j) => [j, false]));

function visit(x, y) {
    const value = (markedInputs[x] || [])[y];

    if (!value || value[1] || value[0] === 9) {
        return 0;
    } else {
        value[1] = true;
        return 1 + visit(x + 1, y) + visit(x - 1, y) + visit(x, y + 1) + visit(x, y - 1);
    }
}

const best = [0, 0, 0];
for (let i = 0; i < markedInputs.length; i++) {
    for (let j = 0; j < markedInputs[i].length; j++) {
        const value = visit(i, j);

        if (value > best[2]) {
            best[0] = best[1];
            best[1] = best[2];
            best[2] = value;
        } else if (value > best[1]) {
            best[0] = best[1];
            best[1] = value;
        } else if (value > best[0]) {
            best[0] = value;
        }
    }
}

console.log(`Part 1: ${holeSum}`);
console.log(`Part 2: ${best[2] * best[1] * best[0]}`);
