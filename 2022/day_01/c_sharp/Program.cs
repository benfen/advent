using System;

namespace c_sharp
{
    class Program
    {
        static int findHighestNumberOfCalories(string[] fileContents) {
            int listLengthForIndex = fileContents.Length - 1;
            int currentCalories = 0;
            int highestNumberOfCalories = 0;

            for (int i = 0; i < listLengthForIndex; i++) {
                if (fileContents[i] == string.Empty) {
                    highestNumberOfCalories = currentCalories;
                    currentCalories = 0;
                } else {
                    currentCalories += int.Parse(fileContents[i]);
                }
            }

            return highestNumberOfCalories;
        }

        static int findTotalCaloriesOfTopNumberOfElves(string[] fileContents, int topRanks) {
            int listLengthForIndex = fileContents.Length - 1;
            int currentCalories = 0;
            int calorieListLengthForIndex = -1;
            List<int> totalCaloriesPerElf = new List<int>();

            for (int i = 0; i < listLengthForIndex; i++) {
                if (fileContents[i] == string.Empty) {
                    totalCaloriesPerElf.Add(currentCalories);
                    calorieListLengthForIndex++;
                    currentCalories = 0;
                } else {
                    currentCalories += int.Parse(fileContents[i]);
                }
            }

            totalCaloriesPerElf.Sort();

            int topRanksTotal = 0;

            for (int i = calorieListLengthForIndex; i > (calorieListLengthForIndex - topRanks); i--) {
                topRanksTotal += totalCaloriesPerElf[i];
            }

            return topRanksTotal;
        }

        static void Main(string[] args)
        {
			string[] fileContents = System.IO.File.ReadAllLines("./input.txt");

            int highestNumberOfCalories = findHighestNumberOfCalories(fileContents);
            int caloriesOfTopThree = findTotalCaloriesOfTopNumberOfElves(fileContents, 3);

			Console.WriteLine(highestNumberOfCalories);
			Console.WriteLine(caloriesOfTopThree);
        }

    }
}
