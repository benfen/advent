import fs from 'fs'
import path from 'path'

const input = fs
	.readFileSync(path.resolve(__dirname, 'input.txt'), 'utf8')
	.split('\n')

// Pt1
// let currentElfCals = 0
// let maxCalories = 0
// input.forEach((cals) => {
// 	if (cals) {
// 		currentElfCals += Number(cals)
// 		return
// 	}
// 	if (!cals) {
// 		if (currentElfCals > maxCalories) {
// 			maxCalories = currentElfCals
// 			currentElfCals = 0
// 			return
// 		} else {
// 			currentElfCals = 0
// 			return
// 		}
// 	}
// })

// console.log('Maximum calories is: ', maxCalories)

// Pt2
let currentElfCals = 0
const topThree: number[] = []

input.forEach((cals) => {
	topThree.sort((a, b) => a - b)
	if (cals) {
		currentElfCals += Number(cals)
	}
	if (!cals) {
		if (topThree.length < 3) {
			topThree.push(currentElfCals)
			currentElfCals = 0
			return
		}
		const switchIndex = topThree.findIndex((cals) => currentElfCals > cals)
		if (switchIndex !== -1) {
			topThree[switchIndex] = currentElfCals
			currentElfCals = 0
		} else {
			currentElfCals = 0
		}
	}
})

const total = topThree.reduce((accum, current) => (accum += current), 0)
console.log('Top three total: ', total)
