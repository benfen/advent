import Data.List.Split
import System.IO

isEmptyString :: String -> Bool
isEmptyString "" = True
isEmptyString _ = False

countNextDepthIncrease :: [Int] -> Int
countNextDepthIncrease [] = 0
countNextDepthIncrease [_] = 0
countNextDepthIncrease (n : c : ns)
  | c > n = 1 + countNextDepthIncrease (c : ns)
  | otherwise = countNextDepthIncrease (c : ns)

countNextWindowDepthIncrease :: [Int] -> Int
countNextWindowDepthIncrease [] = 0
countNextWindowDepthIncrease [_] = 0
countNextWindowDepthIncrease [_, _] = 0
countNextWindowDepthIncrease [_, _, _] = 0
countNextWindowDepthIncrease (n : a : b : c : ns)
  | c > n = 1 + countNextWindowDepthIncrease (a : b : c : ns)
  | otherwise = countNextWindowDepthIncrease (a : b : c : ns)

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle
        let depths = map read (filter (not . isEmptyString) (splitOn "\n" contents))

        putStr "Part 1: "
        putStr (show (countNextDepthIncrease depths))
        putStr "\n"

        putStr "Part 2: "
        putStr (show (countNextWindowDepthIncrease depths))
        putStr "\n"
    )
