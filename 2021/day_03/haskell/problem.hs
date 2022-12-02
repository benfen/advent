import Data.List.Split
import System.IO

isEmptyString :: String -> Bool
isEmptyString "" = True
isEmptyString _ = False

checkFirstDiagnostic :: String -> [Int]
checkFirstDiagnostic "" = []
checkFirstDiagnostic ('0' : ss) = 0 : checkFirstDiagnostic ss
checkFirstDiagnostic ('1' : ss) = 1 : checkFirstDiagnostic ss
checkFirstDiagnostic _ = error "Non-binary input"

checkSingleDiagnostic :: String -> [Int] -> [Int]
checkSingleDiagnostic "" [] = []
checkSingleDiagnostic ('0' : ss) (n : ns) = n : checkSingleDiagnostic ss ns
checkSingleDiagnostic ('1' : ss) (n : ns) = (n + 1) : checkSingleDiagnostic ss ns
checkSingleDiagnostic (_ : ss) _ = error "Non-binary input"
checkSingleDiagnostic _ _ = error "Array lengths did not match"

checkDiagnostic :: [String] -> [Int]
checkDiagnostic [] = []
checkDiagnostic [s] = checkFirstDiagnostic s
checkDiagnostic (s : ss) = checkSingleDiagnostic s (checkDiagnostic ss)

data Rate = Rate Int Int

increaseGamma :: Int -> Rate -> Rate
increaseGamma n (Rate g e) = Rate (g + n) e

increaseEpsilon :: Int -> Rate -> Rate
increaseEpsilon n (Rate g e) = Rate g (e + n)

calculcatePowerConsumption :: Rate -> Int
calculcatePowerConsumption (Rate g e) = g * e

determineRate :: [Int] -> Int -> Rate
determineRate [] _ = Rate 0 0
determineRate [n] l
  | n > div l 2 = Rate 1 0
  | otherwise = Rate 0 1
determineRate (n : ns) l
  | n > div l 2 = increaseGamma (2 ^ length ns) (determineRate ns l)
  | otherwise = increaseEpsilon (2 ^ length ns) (determineRate ns l)

countAtIndex :: [String] -> Int -> Int
countAtIndex [] _ = 0
countAtIndex (s : ss) i
  | s !! i == '0' = countAtIndex ss i
  | s !! i == '1' = 1 + countAtIndex ss i
  | otherwise = error "Non-binary input"

findRatingCode :: [String] -> (Char -> Int -> Int -> Bool) -> Int -> [String]
findRatingCode [] _ _ = []
findRatingCode [s] _ _ = [s]
findRatingCode ss f i
  | i >= length (head ss) = ss
  | not (any (\x -> f (x !! i) (length ss) (countAtIndex ss i)) ss) = ss
  | otherwise = findRatingCode (filter (\x -> f (x !! i) (length ss) (countAtIndex ss i)) ss) f (i + 1)

binaryToInt :: String -> Int
binaryToInt "" = 0
binaryToInt ('0' : ss) = binaryToInt ss
binaryToInt ('1' : ss) = 2 ^ length ss + binaryToInt ss
binaryToInt _ = error "Non-binary input"

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle
        let diagnosticList = filter (not . isEmptyString) (splitOn "\n" contents)
        let diagnostic = checkDiagnostic diagnosticList
        let rate = determineRate diagnostic (length diagnosticList)

        putStr "Part 1: "
        putStr (show (calculcatePowerConsumption rate))
        putStr "\n"

        -- This is some real shitty stuff
        -- Also wildly inefficient
        let oxygenRating = findRatingCode diagnosticList (\c length count -> count >= (length - count) && c == '1' || count < (length - count) && c == '0') 0
        let scrubberRating = findRatingCode diagnosticList (\c length count -> count < (length - count) && c == '1' || count >= (length - count) && c == '0') 0

        putStr "Part 2: "
        putStr (show (binaryToInt (head oxygenRating) * binaryToInt (head scrubberRating)))
        putStr "\n"
    )
