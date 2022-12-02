import Data.List (sort)
import Data.List.Split
import Data.Maybe (mapMaybe)
import System.IO

data ParseResult = Ok | Incomplete String | ParseError Char

evaluateParseResultScore :: ParseResult -> Int
evaluateParseResultScore (ParseError ')') = 3
evaluateParseResultScore (ParseError ']') = 57
evaluateParseResultScore (ParseError '}') = 1197
evaluateParseResultScore (ParseError '>') = 25137
evaluateParseResultScore _ = 0

getIncompleteString :: ParseResult -> Maybe String
getIncompleteString (Incomplete ss) = Just (reverse ss)
getIncompleteString _ = Nothing

evaluateIncompleteResultScore :: String -> Int
evaluateIncompleteResultScore "" = 0
evaluateIncompleteResultScore (s : ss)
  | s == ')' = baseScore + 1
  | s == ']' = baseScore + 2
  | s == '}' = baseScore + 3
  | s == '>' = baseScore + 4
  | otherwise = baseScore
  where
    baseScore = 5 * evaluateIncompleteResultScore ss

parseLine :: String -> String -> ParseResult
parseLine "" [] = Ok
parseLine "" cs = Incomplete cs
parseLine ('[' : ss) cs = parseLine ss (']' : cs)
parseLine ('(' : ss) cs = parseLine ss (')' : cs)
parseLine ('{' : ss) cs = parseLine ss ('}' : cs)
parseLine ('<' : ss) cs = parseLine ss ('>' : cs)
parseLine (s : ss) (c : cs)
  | s /= c = ParseError s
  | otherwise = parseLine ss cs
parseLine (s : ss) [] = ParseError s

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle

        let lines = map (`parseLine` "") (splitOn "\n" contents)
        let part1 = sum (map evaluateParseResultScore lines)
        let part2 = sort $ map evaluateIncompleteResultScore (mapMaybe getIncompleteString lines)

        putStrLn $ "Part 1: " ++ show part1
        putStrLn $ "Part 2: " ++ show (part2 !! div (length part2) 2)
    )
