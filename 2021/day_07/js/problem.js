const { readFileSync } = require('fs');

const locations = readFileSync('../input.txt').toString().split(',').map(x => parseInt(x)).sort((a, b) => a - b);

function createFuelPositionData(position, index) {
  return {
    sum: 0,
    index,
    position,
    count: 0,
    last: 0,
  };
}

function alterPosition(fuelPosition, change, invert) {
  const { count, last, sum, position } = fuelPosition;

  if (invert) {
    return {
      ...fuelPosition,
      sum: sum - last,
      last: last - count,
      position: position + change,
    };
  } else {
    return {
      ...fuelPosition,
      sum: sum + count + last,
      position: position + change,
      last: last + count,
    };
  }
}

function alterCount(fuelPosition, change, invert) {
  let countChange = 1;
  if (invert) {
    countChange = -1;
  }
  return {
    ...fuelPosition,
    count: fuelPosition.count + countChange,
    index: fuelPosition.index + change,
  };
}

let leftFuelPosition = createFuelPositionData(0, 0);

while (leftFuelPosition.index < locations.length) {
  if (locations[leftFuelPosition.index] === leftFuelPosition.position) {
    leftFuelPosition = alterCount(leftFuelPosition, 1);
  } else {
    leftFuelPosition = alterPosition(leftFuelPosition, 1);
  }
}

leftFuelPosition = {
  ...leftFuelPosition,
  index: leftFuelPosition.index - 1,
};
let lowest = [leftFuelPosition.position, leftFuelPosition.sum];


let rightFuelPosition = createFuelPositionData(locations[locations.length - 1], locations.length - 1);

while (rightFuelPosition.index >= 0) {

  if (locations[rightFuelPosition.index] === rightFuelPosition.position) {
    rightFuelPosition = alterCount(rightFuelPosition, -1);
    leftFuelPosition = alterCount(leftFuelPosition, -1, true);
  } else {
    rightFuelPosition = alterPosition(rightFuelPosition, -1);
    leftFuelPosition = alterPosition(leftFuelPosition, -1, true);
  }

  const sum = rightFuelPosition.sum + leftFuelPosition.sum;
  if (sum < lowest[1]) {
    lowest = [rightFuelPosition.position, sum];
  }
}

console.log(`Part 2: ${lowest[1]}`)
