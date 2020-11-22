module Test

let rec isSorted list =
    match list with
    | x :: y :: xs -> (x <= y) && isSorted (y :: xs)
    | _ -> true

let rec p1Valid list =
    match list with
    | x :: y :: xs -> (x = y) || p1Valid (y :: xs)
    | _ -> false

let rec p2Valid list =
    list
    |> Seq.groupBy (fun s -> s)
    |> Seq.exists (fun (_, g) -> (g |> Seq.length) = 2)

let sorted =
    [ for i in 236491..713787 ->
        (sprintf "%d" i)
         |> Seq.map int
         |> Seq.toList
    ]
    |> Seq.filter isSortedd

let part1 = sorted |> Seq.filter p1Valid |> Seq.length
printf "Part 1: %A" part1

let part2 = sorted |> Seq.filter p2Valid |> Seq.length
printf "Part 2: %A" part2
