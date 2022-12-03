using System;

namespace c_sharp
{
    class Program
    {
        static void Main(string[] args)
        {
            Dictionary<string, int[]> possibleMatches = new Dictionary<string, int[]>() {
                {"A X", new int[] {4, 3}},
                {"A Y", new int[] {8, 4}},
                {"A Z", new int[] {3, 8}},
                {"B X", new int[] {1, 1}},
                {"B Y", new int[] {5, 5}},
                {"B Z", new int[] {9, 9}},
                {"C X", new int[] {7, 2}},
                {"C Y", new int[] {2, 6}},
                {"C Z", new int[] {6, 7}},
            };

			string[] matchStrings = System.IO.File.ReadAllLines("./input.txt");
            int matchStringsLength = matchStrings.Length;

            int solutionOne = 0;
            int solutionTwo = 0;

            for (int i = 0; i < matchStringsLength; i++) {
                solutionOne += possibleMatches[matchStrings[i]][0];
                solutionTwo += possibleMatches[matchStrings[i]][1];
            }

            System.Console.WriteLine(solutionOne);
            System.Console.WriteLine(solutionTwo);
        }
    }
}
