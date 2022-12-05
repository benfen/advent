using System;

namespace c_sharp
{
    class Program
    {
        static void Main(string[] args)
        {

			string[] pairStrings = System.IO.File.ReadAllLines("./input.txt");
            int pairsLength = pairStrings.Length;
            int solutionOne = 0;
            int solutionTwo = 0;

            for (int i = 0; i < pairsLength; i++) {
                string[] pairs = pairStrings[i].Split(',');
                string[] firstElfRange = pairs[0].Split('-');
                string[] secondElfRange = pairs[1].Split('-');
                
                if (
                    int.Parse(firstElfRange[0]) <= int.Parse(secondElfRange[0]) && int.Parse(firstElfRange[1]) >= int.Parse(secondElfRange[1])
                    || int.Parse(secondElfRange[0]) <= int.Parse(firstElfRange[0]) && int.Parse(secondElfRange[1]) >= int.Parse(firstElfRange[1])
                ) {
                    solutionOne++;
                }

                if (
                    int.Parse(firstElfRange[0]) <= int.Parse(secondElfRange[1]) && int.Parse(firstElfRange[0]) >= int.Parse(secondElfRange[0])
                    || int.Parse(secondElfRange[0]) <= int.Parse(firstElfRange[1]) && int.Parse(secondElfRange[0]) >= int.Parse(firstElfRange[0])
                ) {
                    solutionTwo++;
                }
            }


            System.Console.WriteLine(solutionOne);
            System.Console.WriteLine(solutionTwo);
        }
    }
}
