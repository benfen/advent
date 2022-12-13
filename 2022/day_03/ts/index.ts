import fs from "fs";
import path from "path";

const input = fs
  .readFileSync(path.resolve(__dirname, "input.txt"), "utf8")
  .split("\n");

const rank: Record<string, number> = {
  a: 1,
  b: 2,
  c: 3,
  d: 4,
  e: 5,
  f: 6,
  g: 7,
  h: 8,
  i: 9,
  j: 10,
  k: 11,
  l: 12,
  m: 13,
  n: 14,
  o: 15,
  p: 16,
  q: 17,
  r: 18,
  s: 19,
  t: 20,
  u: 21,
  v: 22,
  w: 23,
  x: 24,
  y: 25,
  z: 26,
};

// Part 1
// const common: Record<number, number> = {};

// input.forEach((rudsack, index) => {
//   const numCompartments = rudsack.length;

//   const firstHalf = rudsack.slice(0, numCompartments / 2);
//   const secondHalf = rudsack.slice(numCompartments / 2, numCompartments);
//   const left: Record<string, number> = {};

//   for (let i = 0; i <= firstHalf.length - 1; i++) {
//     const leftLetter = firstHalf[i];

//     if (!left[leftLetter]) {
//       left[leftLetter] = 1;
//     }
//   }

//   for (let i = 0; i <= secondHalf.length - 1; i++) {
//     const rightLetter = secondHalf[i];

//     if (left[rightLetter]) {
//       if (rank[rightLetter]) {
//         common[index + 1] = rank[rightLetter];
//       } else {
//         common[index + 1] = rank[rightLetter.toLocaleLowerCase()] + 26;
//       }
//       break;
//     }
//   }
// });

// const total = Object.values(common).reduce((a, b) => a + b);
// console.log('💥 total 💥', total )

// Part 2
const common: Record<number, number> = {};
const rudsackGroup = 3;
let localCommonRudsack: Record<string, number> = {};

input.forEach((rudsack, index) => {
  const removedDuplicates = rudsack.split('').filter(
    (it, i) => rudsack.indexOf(it) === i
  ).join('');

  const numCompartments = removedDuplicates.length;
  
  for (let i = 0; i <= numCompartments - 1; i++) {
    const letter = removedDuplicates[i];
    if (index % rudsackGroup === 0) {
      if (!localCommonRudsack[letter]) {
        localCommonRudsack[letter] = 1;
      }
    }
    if (index % rudsackGroup === 1) {
      if (localCommonRudsack[letter] && localCommonRudsack[letter] === 1) {
        localCommonRudsack[letter] += 1;
      }
    }
    if (index % rudsackGroup === 2) {
      if (localCommonRudsack[letter] && localCommonRudsack[letter] === 2) {
        if (rank[letter]) {
          common[index + 1] = rank[letter];
        } else {
          common[index + 1] = rank[letter.toLocaleLowerCase()] + 26;
        }
        localCommonRudsack = {};
        break;
      }
    }
  }

});

const total = Object.values(common).reduce((a, b) => a + b);
console.log("💥 total 💥", total);
