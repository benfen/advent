import Data.List.Split
import Data.Map hiding (showTreeWith)
import Data.Map.Internal.Debug
import System.IO

isEmptyChar :: String -> Bool
isEmptyChar "" = True
isEmptyChar _ = False

data Position = Position Int Int deriving (Show)

type BingoBoard = Map Int Position

addPosition :: [(Int, Int)] -> Int -> BingoBoard -> BingoBoard
addPosition [] _ b = b
addPosition ((n, x) : ns) y b = addPosition ns y (insert n (Position x y) b)

parseBingoBoard :: [String] -> BingoBoard
parseBingoBoard [] = empty
parseBingoBoard (s : ss) = addPosition (zip (Prelude.map read (Prelude.filter (not . isEmptyChar) (splitOn " " s))) [0 ..]) (length ss) (parseBingoBoard ss)

parseBoards :: [String] -> [BingoBoard]
parseBoards (_ : a : b : c : d : e : ss) = parseBingoBoard [a, b, c, d, e] : parseBoards ss
parseBoards _ = []

-- evaluateBoards :: [BingoBoard] -> [Int] -> BingoBoard
-- evaluateBoards [] _ = error "Can't evaluate an empty array"
-- evaluateBoards (b : bs) nums =

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        -- This does not work
        -- I gave up halfway through
        contents <- hGetContents handle
        let (firstLine : lines) = splitOn "\n" contents
        -- let calledNumbers = Prelude.map read (splitOn "," firstLine)
        let boards = parseBoards lines

        putStrLn $ showTreeWith (curry show) True True (head boards)

        putStr "Part 2: "
        print (head contents)
    )
