using System;

namespace c_sharp
{
    class Program
    {
        static void Main(string[] args)
        {
			string[] fileContents = System.IO.File.ReadAllText("./input.txt").Split("\n");
			int contentLength = fileContents.Length;

			int[] nums = new int[contentLength];

			if (Int32.TryParse(fileContents[0], out int a)) {
				nums[0] = a;
			}
			int partOneCount = 0;
			int partTwoCount = 0;

			for (int i = 1; i < contentLength; i++) {
				if (Int32.TryParse(fileContents[i], out int j)) {
					nums[i] = j;
					if (nums[i] > nums[i-1]) {
						partOneCount++;
					}
					if (i >= 3 && nums[i] > nums[i-3]) {
						partTwoCount++;
					}
				}
			}

			Console.WriteLine("Part 1:" + partOneCount);
			Console.WriteLine("Part 2:" + partTwoCount);
        }
    }
}
