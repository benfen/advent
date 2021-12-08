import Data.List
import Data.List.Split
import Data.Vector (Vector)
import System.IO

type FuelPosition = (Int, Int)

data FuelPositionData = FuelPositionData
  { fsum :: Int,
    fpos :: Int,
    fcount :: Int,
    flast :: Int
  }
  deriving (Show)

getFuel :: FuelPosition -> Int
getFuel (_, s) = s

minimumFuel :: FuelPosition -> FuelPosition -> FuelPosition
minimumFuel (ap, as) (bp, bs)
  | as < bs = (ap, as)
  | otherwise = (bp, bs)

findBestDistance :: Int -> Int -> Int -> [Int] -> FuelPosition
findBestDistance s p _ [] = (p, s)
findBestDistance s p c (n : ns)
  | p == n = findBestDistance s p (c + 1) ns
  -- Could exit early, because fuel costs should only trend up once they start going up
  | p < n = minimumFuel (p, s) (findBestDistance (s + c - length (n : ns)) (p + 1) c (n : ns))
  | otherwise = error "This really shoudn't be happening"

newFuelPositionData :: Int -> FuelPositionData
newFuelPositionData pos = FuelPositionData {fsum = 0, fpos = pos, fcount = 0, flast = 0}

getPos :: FuelPositionData -> Int
getPos FuelPositionData {fpos = x} = x

getSum :: FuelPositionData -> Int
getSum FuelPositionData {fsum = x} = x

getCount :: FuelPositionData -> Int
getCount FuelPositionData {fcount = x} = x

getLast :: FuelPositionData -> Int
getLast FuelPositionData {flast = x} = x

addItem :: FuelPositionData -> Bool -> FuelPositionData
addItem f True = f {fcount = 1 + getCount f}
addItem f False = f {fcount = (-1) + getCount f}

alterPosition :: FuelPositionData -> Int -> Bool -> FuelPositionData
alterPosition f p True = f {fsum = getSum f + getCount f + getLast f, fpos = getPos f + p, flast = getLast f + getCount f}
alterPosition f p False = f {fsum = getSum f - getLast f, fpos = getPos f + p, flast = getLast f - getCount f}

createInitialFuelPosition :: FuelPositionData -> [Int] -> FuelPositionData
createInitialFuelPosition f [] = f
createInitialFuelPosition f (n : ns)
  | getPos f == n = createInitialFuelPosition (addItem f True) ns
  | otherwise = createInitialFuelPosition (alterPosition f 1 True) (n : ns)

findLowestFuelPosition :: FuelPositionData -> FuelPositionData -> [Int] -> FuelPosition
findLowestFuelPosition _ r [] = (getPos r, getSum r)
findLowestFuelPosition l r (n : ns)
  | getPos r == n = minimumFuel (pos, sum) (findLowestFuelPosition (addItem l False) (addItem r True) ns)
  | otherwise = minimumFuel (pos, sum) (findLowestFuelPosition (alterPosition l (-1) False) (alterPosition r (-1) True) (n : ns))
  where
    sum = getSum l + getSum r
    pos = getPos r

main = do
  withFile
    "../input.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle

        let crabs = sort (map read (splitOn "," contents)) :: [Int]

        putStrLn $ "Part 1: " ++ show (getFuel (findBestDistance (sum crabs) 0 0 crabs))
        let initialFuelPosition = createInitialFuelPosition (newFuelPositionData 0) crabs
        putStrLn $ "Part 2: " ++ show (getFuel (findLowestFuelPosition initialFuelPosition (newFuelPositionData (getPos initialFuelPosition)) (reverse crabs)))
    )
