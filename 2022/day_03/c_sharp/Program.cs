using System;

namespace c_sharp
{
    class Program
    {
        static Dictionary<char, int> priorities = new Dictionary<char, int>() {
            {'a', 1},
            {'b', 2},
            {'c', 3},
            {'d', 4},
            {'e', 5},
            {'f', 6},
            {'g', 7},
            {'h', 8},
            {'i', 9},
            {'j', 10},
            {'k', 11},
            {'l', 12},
            {'m', 13},
            {'n', 14},
            {'o', 15},
            {'p', 16},
            {'q', 17},
            {'r', 18},
            {'s', 19},
            {'t', 20},
            {'u', 21},
            {'v', 22},
            {'w', 23},
            {'x', 24},
            {'y', 25},
            {'z', 26},
        };

        static int FindPriority(string rucksack) {
            int rucksackLength = rucksack.Length;

            var firstCompartment = new HashSet<char>(rucksack.Substring(0, rucksackLength/2));
            var secondCompartment = new HashSet<char>(rucksack.Substring(rucksackLength/2));

            firstCompartment.IntersectWith(secondCompartment);
            char overlappingChar = firstCompartment.ToArray()[0];

            if (priorities.ContainsKey(overlappingChar)) {
                return priorities[overlappingChar];
            } else if (priorities.ContainsKey(overlappingChar.ToString().ToLower().ToCharArray()[0])) {
                return priorities[overlappingChar.ToString().ToLower().ToCharArray()[0]] + 26;
            }
            return 0;
        }

        static void Main(string[] args)
        {


			string[] rucksacks = System.IO.File.ReadAllLines("./input.txt");
            int numOfRucksacks = rucksacks.Length;

            int solutionOne = 0;
            int solutionTwo = 0;

            for (int i = 0; i < numOfRucksacks; i += 3) {
                solutionOne += FindPriority(rucksacks[i]) + FindPriority(rucksacks[i + 1]) + FindPriority(rucksacks[i + 2]);

                var firstSack = new HashSet<char>(rucksacks[i]);
                var secondSack = new HashSet<char>(rucksacks[i + 1]);
                var thirdSack = new HashSet<char>(rucksacks[i + 2]);

                firstSack.IntersectWith(secondSack);
                firstSack.IntersectWith(thirdSack);
                char overlappingChar = firstSack.ToArray()[0];

                if (priorities.ContainsKey(overlappingChar)) {
                    solutionTwo += priorities[overlappingChar];
                } else if (priorities.ContainsKey(overlappingChar.ToString().ToLower().ToCharArray()[0])) {
                    solutionTwo += priorities[overlappingChar.ToString().ToLower().ToCharArray()[0]] + 26;
                }
            }

            System.Console.WriteLine(solutionOne);
            System.Console.WriteLine(solutionTwo);
        }
    }
}
