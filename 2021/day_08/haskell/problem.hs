import Data.List.Split
import System.IO

countUniqueSegments :: [[String]] -> Int
countUniqueSegments =
  foldr
    ( (+)
        . length
        . filter
          (\x -> let l = length x in l == 2 || l == 3 || l == 4 || l == 7)
    )
    0

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle

        let outputs = map (filter (/= "") . splitOn " " . last . splitOn "|") (splitOn "\n" contents)

        putStrLn $ "Part 1: " ++ show (countUniqueSegments outputs)
    )
