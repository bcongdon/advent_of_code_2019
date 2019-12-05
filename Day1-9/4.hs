import           Data.List

isSorted (x:y:xs) = x <= y && isSorted (y:xs)
isSorted _        = True

p1Valid (x:y:xs) = x == y || p1Valid (y:xs)
p1Valid _        = False

p2Valid (x:xs) = (length group) == 1 || p2Valid rest
    where
        (group, rest) = partition (== x) xs
p2Valid _ = False

main = do
    let sorted = filter isSorted [show x | x <- [236491..713787]]
    putStrLn $ show $ length $ filter p1Valid sorted
    putStrLn $ show $ length $ filter p2Valid sorted

