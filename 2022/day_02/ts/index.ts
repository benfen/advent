import fs from 'fs'
import path from 'path'

const input = fs
	.readFileSync(path.resolve(__dirname, 'input.txt'), 'utf8')
	.split('\n')

// Column 1:
// A for Rock, B for Paper, and C for Scissors

// Column 2:
// X for Rock, Y for Paper, and Z for Scissors

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

// Part 1
// const losingGame = {
// 	A: 'Z',
// 	B: 'X',
// 	C: 'Y',
// }
// const tieGame = {
// 	A: 'X',
// 	B: 'Y',
// 	C: 'Z',
// }

// const scoring = {
// 	'X': 1,
// 	'Y': 2,
// 	'Z': 3,
// 	win: 6,
// 	draw: 3,
// 	total: 0,
// }

// input
// 	.filter((item) => item)
// 	.forEach((game) => {
// 		const opponent = game[0] as 'A' | 'B' | 'C'
// 		const self = game[2] as 'X' | 'Y' | 'Z'
// 		if (losingGame[opponent] === self) {
// 			scoring.total += scoring[self]
// 			return
// 		}
// 		if (tieGame[opponent] === self) {
// 			scoring.total += scoring[self] + scoring.draw
// 			return
// 		} else {
// 			scoring.total += scoring[self] + scoring.win
// 			return
// 		}
// 	})

// console.log('💥 total 💥', scoring.total)

// Part 2
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
type Opponent = 'A' | 'B' | 'C'
type Self = 'X' | 'Y' | 'Z'
const losingGame: Record<Opponent, Self> = {
	A: 'Z',
	B: 'X',
	C: 'Y',
}
const tieGame: Record<Opponent, Self> = {
	A: 'X',
	B: 'Y',
	C: 'Z',
}
const winningGame: Record<Opponent, Self> = {
	A: 'Y',
	B: 'Z',
	C: 'X',
}

const scoring: Record<Self, number> = {
	'X': 1,
	'Y': 2,
	'Z': 3,
}

let total = 0

input
	.filter((item) => item)
	.forEach((game) => {
		const opponent = game[0] as 'A' | 'B' | 'C'
		const self = game[2] as 'X' | 'Y' | 'Z'
		if (self === 'X') {
			total += scoring[losingGame[opponent]]
		}
		if (self === 'Y') {
			total += scoring[tieGame[opponent]] + 3
			return
		}
		if (self === 'Z') {
			total += scoring[winningGame[opponent]] + 6
			return
		}
	})

console.log('💥 total 💥', total)
