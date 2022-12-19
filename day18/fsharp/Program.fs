open System.IO

let input = File.ReadAllLines("input")

let parse (line:string) =
    let parts = line.Split(',')
    int parts[0], int parts[1], int parts[2]
    
let neighbours (x,y,z) =
    seq {
        yield x, y + 1, z
        yield x + 1, y, z
        yield x, y, z + 1
        yield x, y - 1, z
        yield x - 1, y, z
        yield x, y, z - 1
    }
    
let cubes = input |> Seq.map parse |> Set.ofSeq

cubes |> Seq.collect neighbours |> Seq.filter (fun x -> (Set.contains x cubes) |> not) |> Seq.length |> printfn "%A"

    