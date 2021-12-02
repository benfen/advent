import Data.List.Split
import Data.Maybe
import System.IO

data Command = Forward Int | Up Int | Down Int

-- Part 1 data structure
data Position = Position Int Int

adjustDepth :: Position -> Int -> Position
adjustDepth (Position depth location) x = Position (depth + x) location

adjustLocation :: Position -> Int -> Position
adjustLocation (Position depth location) x = Position depth (location + x)

finalPositionValue :: Position -> Int
finalPositionValue (Position depth location) = depth * location

determinePosition :: [Maybe Command] -> Position
determinePosition [] = Position 0 0
determinePosition (Nothing : cs) = determinePosition cs
determinePosition (Just (Forward x) : cs) = adjustLocation (determinePosition cs) x
determinePosition (Just (Up x) : cs) = adjustDepth (determinePosition cs) (- x)
determinePosition (Just (Down x) : cs) = adjustDepth (determinePosition cs) x

-- Part 2 data structure
data AimedPosition = AimedPosition Int Int Int deriving (Show)

adjustAim :: AimedPosition -> Int -> AimedPosition
adjustAim (AimedPosition aim depth location) x = AimedPosition (aim + x) depth location

moveForward :: AimedPosition -> Int -> AimedPosition
moveForward (AimedPosition aim depth location) x = AimedPosition aim (depth + aim * x) (location + x)

determineAimedPosition :: [Maybe Command] -> AimedPosition
determineAimedPosition [] = AimedPosition 0 0 0
determineAimedPosition (Nothing : cs) = determineAimedPosition cs
determineAimedPosition (Just (Forward x) : cs) = moveForward (determineAimedPosition cs) x
determineAimedPosition (Just (Up x) : cs) = adjustAim (determineAimedPosition cs) (- x)
determineAimedPosition (Just (Down x) : cs) = adjustAim (determineAimedPosition cs) x

finalAimedPositionValue :: AimedPosition -> Int
finalAimedPositionValue (AimedPosition aim depth location) = depth * location

parseCommand :: String -> Maybe Command
parseCommand ('u' : 'p' : ' ' : n) = Just (Up (read n))
parseCommand ('d' : 'o' : 'w' : 'n' : ' ' : n) = Just (Down (read n))
parseCommand ('f' : 'o' : 'r' : 'w' : 'a' : 'r' : 'd' : ' ' : n) = Just (Forward (read n))
parseCommand _ = Nothing

isEmptyString :: String -> Bool
isEmptyString "" = True
isEmptyString _ = False

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle
        -- Since our approach is recursive and starts from the end, the list of commands needs to be reversed
        let commands = reverse (map parseCommand (filter (not . isEmptyString) (splitOn "\n" contents)))

        putStr "Part 1: "
        putStr (show (finalPositionValue (determinePosition commands)))
        putStr "\n"

        putStr "Part 2: "
        putStr (show (finalAimedPositionValue (determineAimedPosition commands)))
        putStr "\n"
    )
