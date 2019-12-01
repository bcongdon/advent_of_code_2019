import           System.Environment

fuelReq :: Integer -> Integer
fuelReq x = max 0 ((div x 3) - 2)

part1 :: [Integer] -> Integer
part1 ms = sum $ map fuelReq ms

part2 :: [Integer] -> Integer
part2 ms = sum $ map p2Rec ms

p2Rec :: Integer -> Integer
p2Rec m
        | f <= 0 = 0
        | otherwise = f + p2Rec f
    where f = fuelReq m

main = do
    args <- getArgs
    content <- readFile "1.txt"
    let inputLines = lines content
    let modules = map (\x -> read x :: Integer) inputLines

    putStrLn $ "Part 1: " ++ show (part1 modules)
    putStrLn $ "Part 2: " ++ show (part2 modules)
