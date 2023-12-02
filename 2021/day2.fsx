let g = System.IO.File.ReadAllText(@"day2.data")

let sol1 =
    g.Split("\n")
    |> List.ofSeq
    |> List.map (fun x ->
        let y = x.Split(" ")
        (y.[0], int y.[1]))
    |> List.fold (fun (h, d) (c, x) -> match c with | "forward" -> (h + x, d) | "down" -> (h, d + x) | "up"  -> (h, d - x))  (0, 0)
    |> (fun (x , y) -> x * y)

let sol2 =
    g.Split("\n")
    |> List.ofSeq
    |> List.map (fun x ->
        let y = x.Split(" ")
        (y.[0], int y.[1]))
    |> List.fold (fun (h, d, a) (c, x) -> match c with | "forward" -> (h + x, d + (x * a), a) | "down" -> (h, d, a + x) | "up"  -> (h, d, a - x))  (0, 0, 0)
    |> (fun (x , y, _) -> x * y)