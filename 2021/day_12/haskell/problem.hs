import Data.Char (isUpper)
import Data.List.Split (splitOn)
import Data.Map (Map, empty, insertWith, notMember, (!))
import qualified Data.Set as Set
import System.IO (IOMode (ReadMode), hGetContents, withFile)

type Path = (Set.Set String, String, [String])

getPath :: Path -> [String]
getPath (_, _, p) = reverse p

buildPathMap :: [[String]] -> Map String [String]
buildPathMap [] = empty
buildPathMap ((from : to) : ss) = insertWith (++) (head to) [from] (insertWith (++) from [head to] (buildPathMap ss))
buildPathMap _ = error "Interior arrays must be of length 2"

buildPaths :: Bool -> Map String [String] -> [Path] -> [Path]
buildPaths _ _ [] = []
buildPaths allowRepeat p ((s, next, path) : ps)
  | next == "start" = buildPaths allowRepeat p ps
  | next == "end" = (s, next, "end" : path) : buildPaths allowRepeat p ps
  | notMember next p = buildPaths allowRepeat p ps
  | all isUpper next = buildPaths allowRepeat p (map (\x -> (s, x, next : path)) (p ! next)) ++ buildPaths allowRepeat p ps
  | Set.member next s && not allowRepeat = buildPaths allowRepeat p ps
  | Set.member next s && allowRepeat = buildPaths False p (map (\x -> (s, x, next : path)) (p ! next)) ++ buildPaths allowRepeat p ps
  | otherwise = buildPaths allowRepeat p (map (\x -> ((Set.insert next s), x, next : path)) (p ! next)) ++ buildPaths allowRepeat p ps

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle

        let pathMap = buildPathMap (map (splitOn "-") (filter (/= "") (splitOn "\n" contents)))
        let pathsPart1 = buildPaths False pathMap (map (\x -> (Set.empty, x, ["start"])) (pathMap ! "start"))
        let pathsPart2 = buildPaths True pathMap (map (\x -> (Set.empty, x, ["start"])) (pathMap ! "start"))

        putStrLn $ "Part 1: " ++ show (length pathsPart1)
        putStrLn $ "Part 2: " ++ show (length pathsPart2)
    )
