import Data.List.Split
import System.IO

type BreedingState = (Int, Int, Int, Int, Int, Int, Int, Int, Int)

addToBreedingState :: Int -> Int -> BreedingState -> BreedingState
addToBreedingState index v (a, b, c, d, e, f, g, h, i)
  | index == 0 = (a + v, b, c, d, e, f, g, h, i)
  | index == 1 = (a, b + v, c, d, e, f, g, h, i)
  | index == 2 = (a, b, c + v, d, e, f, g, h, i)
  | index == 3 = (a, b, c, d + v, e, f, g, h, i)
  | index == 4 = (a, b, c, d, e + v, f, g, h, i)
  | index == 5 = (a, b, c, d, e, f + v, g, h, i)
  | index == 6 = (a, b, c, d, e, f, g + v, h, i)
  | index == 7 = (a, b, c, d, e, f, g, h + v, i)
  | index == 8 = (a, b, c, d, e, f, g, h, i + v)
  | otherwise = error "Invalid breeding state index"

generateInitialBreedingState = foldr (\s -> addToBreedingState (read s) 1) (0, 0, 0, 0, 0, 0, 0, 0, 0)

countFish :: BreedingState -> Int
countFish (a, b, c, d, e, f, g, h, i) = a + b + c + d + e + f + g + h + i

calculateGenerations :: Int -> BreedingState -> BreedingState
calculateGenerations 0 b = b
calculateGenerations gen (a, b, c, d, e, f, g, h, i) = calculateGenerations (gen - 1) (b, c, d, e, f, g, h + a, i, a)

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle

        let initialState = generateInitialBreedingState (splitOn "," contents)

        putStrLn $ "Part 1: " ++ show (countFish (calculateGenerations 80 initialState))
        putStrLn $ "Part 2: " ++ show (countFish (calculateGenerations 256 initialState))
    )
